// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
//! `tracing` utilities
//!
//! The module provides mechanics for capturing HTTP requests to output timing statistics when
//! requested by user.
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use tracing::{
    field::{Field, Visit},
    Event, Subscriber,
};
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

/// HTTP Request statistics container
#[derive(Default)]
pub(crate) struct HttpRequestStats {
    pub requests: Vec<HttpRequest>,
}

impl HttpRequestStats {
    /// Summarize captured requests by url without query parameters and method
    pub fn summarize_by_url_method(&self) -> impl IntoIterator<Item = (String, String, u128)> + '_ {
        let mut timings: BTreeMap<String, BTreeMap<String, u128>> = BTreeMap::new();
        for rec in &self.requests {
            let url: String = rec
                .url
                .get(0..rec.url.find('?').unwrap_or(rec.url.len()))
                .unwrap_or(&rec.url)
                .to_string();
            timings
                .entry(url)
                .and_modify(|x| {
                    x.entry(rec.method.clone())
                        .and_modify(|t| *t = t.wrapping_add(rec.duration))
                        .or_insert(rec.duration);
                })
                .or_insert(BTreeMap::from([(rec.method.clone(), rec.duration)]));
        }
        timings
            .into_iter()
            .flat_map(move |(u, v)| v.into_iter().map(move |(m, d)| (u.clone(), m, d)))
    }
}

/// Tracing collector capturing HTTP request metrics
///
/// Added as a `tracing` layer it captures all events with name "request" and mandatory fields: [url,
/// duration_ms, method] (additional optional fields: [status, request_id]
pub(crate) struct RequestTracingCollector {
    pub stats: Arc<Mutex<HttpRequestStats>>,
}

/// Single HTTP request profile record
#[derive(Debug, Default)]
pub(crate) struct HttpRequest {
    pub url: String,
    pub method: String,
    pub duration: u128,
    pub status: u16,
    pub request_id: Option<String>,
}

impl Visit for HttpRequest {
    fn record_u64(&mut self, field: &Field, value: u64) {
        if field.name() == "status" {
            self.status = value as u16;
        }
    }

    fn record_u128(&mut self, field: &Field, value: u128) {
        if field.name() == "duration_ms" {
            self.duration = value;
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "url" => self.url = String::from(value),
            "method" => self.method = String::from(value),
            "request_id" => self.request_id = Some(String::from(value)),
            _ => {}
        };
    }
    fn record_debug(&mut self, _: &Field, _: &dyn (core::fmt::Debug)) {}
}

impl<C> Layer<C> for RequestTracingCollector
where
    C: Subscriber + Send + Sync + 'static,
{
    /// Notifies this layer that an event has occurred.
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, C>) {
        let fields = event.metadata().fields();
        if event.metadata().name() == "http_request"
            && fields.field("url").is_some()
            && fields.field("duration_ms").is_some()
        {
            let mut record = HttpRequest::default();
            event.record(&mut record);
            self.stats.lock().unwrap().requests.push(record);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarize() {
        let records = vec![
            HttpRequest {
                url: String::from("http://foo.bar/"),
                method: String::from("get"),
                duration: 1,
                status: 200,
                request_id: None,
            },
            HttpRequest {
                url: String::from("http://foo.bar/1?foo=bar"),
                method: String::from("get"),
                duration: 2,
                status: 200,
                request_id: None,
            },
            HttpRequest {
                url: String::from("http://foo.bar/1?foo=bar"),
                method: String::from("get"),
                duration: 3,
                status: 200,
                request_id: None,
            },
            HttpRequest {
                url: String::from("http://foo.bar/1?foo=baz"),
                method: String::from("get"),
                duration: 4,
                status: 200,
                request_id: None,
            },
            HttpRequest {
                url: String::from("http://foo.bar/"),
                method: String::from("post"),
                duration: 5,
                status: 200,
                request_id: None,
            },
        ];
        let r = HttpRequestStats { requests: records };

        let summaries: Vec<(String, String, u128)> =
            r.summarize_by_url_method().into_iter().collect();

        assert!(summaries
            .iter()
            .any(|x| *x == (String::from("http://foo.bar/"), String::from("get"), 1)));
        assert!(summaries
            .iter()
            .any(|x| *x == (String::from("http://foo.bar/1"), String::from("get"), 9)));
        assert!(summaries
            .iter()
            .any(|x| *x == (String::from("http://foo.bar/"), String::from("post"), 5)));
    }
}
