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

//! Wait for a resource to reach a target state, or to disappear.
//!
//! `Wait` follows the same combinator pattern as [`crate::api::Find`] and
//! [`crate::api::Paged`]: it wraps a GET-style [`RestEndpoint`] and repeatedly re-issues it,
//! handing each observed body (or its absence) to a caller-supplied predicate until the
//! predicate reports the wait is over, a failure state is reached, or the timeout expires.

use std::time::Duration;

#[cfg(feature = "async")]
use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::api::{ApiError, RestEndpoint};

#[cfg(feature = "async")]
use crate::api::{AsyncClient, QueryAsync};
#[cfg(feature = "sync")]
use crate::api::{Client, Query};

/// What a single poll observed.
#[derive(Debug, Clone, Copy)]
pub enum Observation<T> {
    /// The resource still exists; here is its (deserialized) body.
    Present(T),
    /// The endpoint returned 404 — the resource is gone.
    Gone,
}

/// The caller's verdict after inspecting a single [`Observation`].
#[derive(Debug, Clone)]
pub enum WaitDecision {
    /// The condition is not met yet; keep polling.
    Continue,
    /// The condition is met; stop and report success.
    Done,
    /// The resource entered a terminal failure state; stop and report an error.
    Fail(String),
}

/// The result of a completed wait: either the resource in its final observed state, or
/// confirmation that it disappeared (for waits where that counts as success, e.g. deletion).
#[derive(Debug, Clone)]
pub enum WaitOutcome<T> {
    /// The resource reached the desired state; here is its last observed body.
    Present(T),
    /// The resource disappeared, and the predicate treated that as success.
    Gone,
}

impl<T> WaitOutcome<T> {
    /// Unwrap the `Present` case, for waits where disappearing is never the success path.
    pub fn into_present(self) -> Option<T> {
        match self {
            WaitOutcome::Present(v) => Some(v),
            WaitOutcome::Gone => None,
        }
    }
}

/// Polling backoff schedule.
#[derive(Debug, Clone, Copy)]
pub struct Backoff {
    initial: Duration,
    multiplier: f64,
    max: Duration,
}

impl Backoff {
    /// Poll at a fixed interval.
    pub fn fixed(interval: Duration) -> Self {
        Self {
            initial: interval,
            multiplier: 1.0,
            max: interval,
        }
    }

    /// Poll with exponentially increasing intervals, capped at `max`.
    pub fn exponential(initial: Duration, multiplier: f64, max: Duration) -> Self {
        Self {
            initial,
            multiplier,
            max,
        }
    }

    fn next(&self, current: Duration) -> Duration {
        let scaled = current.as_secs_f64() * self.multiplier;
        Duration::from_secs_f64(scaled).min(self.max)
    }
}

impl Default for Backoff {
    /// 1s, x1.5 per poll, capped at 15s — gentle enough not to hammer a cloud across a fleet of
    /// concurrent waiters, responsive enough not to make short waits feel sluggish.
    fn default() -> Self {
        Self::exponential(Duration::from_secs(1), 1.5, Duration::from_secs(15))
    }
}

/// A query modifier that polls an endpoint until a caller-supplied predicate is satisfied.
pub struct Wait<E, F> {
    endpoint: E,
    check: F,
    backoff: Backoff,
    timeout: Option<Duration>,
    max_transient_errors: u32,
}

/// Wait on `endpoint`, polling until `check` returns [`WaitDecision::Done`] or
/// [`WaitDecision::Fail`]. A 404 is routed through `check` as [`Observation::Gone`] rather than
/// as an error, so both "wait until gone" and "wait until ready" (where disappearing is a
/// failure) share this one combinator.
///
/// Default timeout is 10 minutes; override with [`Wait::timeout`].
pub fn wait<E, F, T>(endpoint: E, check: F) -> Wait<E, F>
where
    F: Fn(Observation<&T>) -> WaitDecision,
{
    Wait {
        endpoint,
        check,
        backoff: Backoff::default(),
        timeout: Some(Duration::from_secs(600)),
        max_transient_errors: 3,
    }
}

/// Convenience: wait until the resource disappears (typically after issuing a delete).
pub fn wait_deleted<E>(
    endpoint: E,
) -> Wait<E, impl Fn(Observation<&serde_json::Value>) -> WaitDecision> {
    wait(endpoint, |obs| match obs {
        Observation::Present(_) => WaitDecision::Continue,
        Observation::Gone => WaitDecision::Done,
    })
}

/// Convenience: wait until the resource's status (read via a JSON Pointer into the response
/// body, e.g. `"/server/status"`) reaches one of `target`, treating any status in `failures` as
/// a terminal failure. A pointer is used rather than a typed status field because which
/// generated `Response` struct actually matches a given call is only known once the server's
/// negotiated microversion is known — the pointer, unlike a Rust type, is something codegen can
/// resolve unambiguously ahead of time from the versioned response schema.
pub fn wait_for_status<E>(
    endpoint: E,
    status_pointer: &'static str,
    target: &'static [&'static str],
    failures: &'static [&'static str],
) -> Wait<E, impl Fn(Observation<&serde_json::Value>) -> WaitDecision> {
    wait(
        endpoint,
        move |obs: Observation<&serde_json::Value>| match obs {
            Observation::Present(body) => {
                match body
                    .pointer(status_pointer)
                    .and_then(serde_json::Value::as_str)
                {
                    Some(status) if target.contains(&status) => WaitDecision::Done,
                    Some(status) if failures.contains(&status) => {
                        WaitDecision::Fail(status.to_string())
                    }
                    _ => WaitDecision::Continue,
                }
            }
            Observation::Gone => WaitDecision::Fail("resource disappeared".into()),
        },
    )
}

impl<E, F> Wait<E, F> {
    /// Overall wait timeout. `None` disables it (wait forever, or until a `Fail`/hard error).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Disable the timeout entirely.
    pub fn no_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }

    /// Set the polling backoff schedule.
    pub fn backoff(mut self, backoff: Backoff) -> Self {
        self.backoff = backoff;
        self
    }

    /// How many consecutive transient (5xx) errors to tolerate mid-wait before giving up. A
    /// single blip should not abort a ten-minute server build.
    pub fn max_transient_errors(mut self, max: u32) -> Self {
        self.max_transient_errors = max;
        self
    }
}

#[cfg(feature = "sync")]
impl<E, F, T, C> Query<WaitOutcome<T>, C> for Wait<E, F>
where
    E: RestEndpoint,
    F: Fn(Observation<&T>) -> WaitDecision,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<WaitOutcome<T>, ApiError<C::Error>> {
        let start = std::time::Instant::now();
        let mut delay = self.backoff.initial;
        let mut transient_errors = 0u32;
        loop {
            match Query::<T, C>::query(&self.endpoint, client) {
                Ok(body) => match (self.check)(Observation::Present(&body)) {
                    WaitDecision::Done => return Ok(WaitOutcome::Present(body)),
                    WaitDecision::Fail(reason) => return Err(ApiError::WaitFailed { reason }),
                    WaitDecision::Continue => {}
                },
                Err(e) if e.is_not_found() => match (self.check)(Observation::Gone) {
                    WaitDecision::Done => return Ok(WaitOutcome::Gone),
                    WaitDecision::Fail(reason) => return Err(ApiError::WaitFailed { reason }),
                    WaitDecision::Continue => return Err(ApiError::WaitResourceVanished),
                },
                Err(e) if e.is_transient() && transient_errors < self.max_transient_errors => {
                    transient_errors += 1;
                }
                Err(e) => return Err(e),
            }

            if let Some(timeout) = self.timeout
                && start.elapsed() >= timeout
            {
                return Err(ApiError::WaitTimeout {
                    elapsed: start.elapsed(),
                });
            }

            std::thread::sleep(delay);
            delay = self.backoff.next(delay);
        }
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<E, F, T, C> QueryAsync<WaitOutcome<T>, C> for Wait<E, F>
where
    E: RestEndpoint + Sync,
    F: Fn(Observation<&T>) -> WaitDecision + Sync,
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<WaitOutcome<T>, ApiError<C::Error>> {
        let start = std::time::Instant::now();
        let mut delay = self.backoff.initial;
        let mut transient_errors = 0u32;
        loop {
            match QueryAsync::<T, C>::query_async(&self.endpoint, client).await {
                Ok(body) => match (self.check)(Observation::Present(&body)) {
                    WaitDecision::Done => return Ok(WaitOutcome::Present(body)),
                    WaitDecision::Fail(reason) => return Err(ApiError::WaitFailed { reason }),
                    WaitDecision::Continue => {}
                },
                Err(e) if e.is_not_found() => match (self.check)(Observation::Gone) {
                    WaitDecision::Done => return Ok(WaitOutcome::Gone),
                    WaitDecision::Fail(reason) => return Err(ApiError::WaitFailed { reason }),
                    WaitDecision::Continue => return Err(ApiError::WaitResourceVanished),
                },
                Err(e) if e.is_transient() && transient_errors < self.max_transient_errors => {
                    transient_errors += 1;
                }
                Err(e) => return Err(e),
            }

            if let Some(timeout) = self.timeout
                && start.elapsed() >= timeout
            {
                return Err(ApiError::WaitTimeout {
                    elapsed: start.elapsed(),
                });
            }

            tokio::time::sleep(delay).await;
            delay = self.backoff.next(delay);
        }
    }
}

#[cfg(test)]
mod tests {
    use httpmock::MockServer;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[cfg(feature = "sync")]
    use crate::api::Query;
    #[cfg(feature = "async")]
    use crate::api::QueryAsync;
    use crate::api::rest_endpoint_prelude::*;
    use crate::api::wait::{
        Backoff, Observation, WaitDecision, WaitOutcome, wait_deleted, wait_for_status,
    };
    use crate::test::client::FakeOpenStackClient;

    #[derive(Debug, Deserialize, Serialize)]
    struct DummyResult {
        status: String,
    }

    struct GetDummy;

    impl RestEndpoint for GetDummy {
        fn method(&self) -> http::Method {
            http::Method::GET
        }
        fn endpoint(&self) -> Cow<'static, str> {
            "dummies/abc".into()
        }
        fn service_type(&self) -> ServiceType {
            ServiceType::from("dummy")
        }
    }

    fn fast_backoff() -> Backoff {
        Backoff::fixed(std::time::Duration::from_millis(1))
    }

    #[cfg(feature = "sync")]
    #[test]
    fn wait_for_status_reaches_target() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"status": "ACTIVE"}));
        });

        let w =
            wait_for_status(GetDummy, "/status", &["ACTIVE"], &["ERROR"]).backoff(fast_backoff());
        let res: WaitOutcome<serde_json::Value> = Query::query(&w, &client).unwrap();
        match res {
            WaitOutcome::Present(r) => assert_eq!(r["status"], "ACTIVE"),
            WaitOutcome::Gone => panic!("expected Present"),
        }
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn wait_for_status_reaches_target_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"status": "ACTIVE"}));
        });

        let w =
            wait_for_status(GetDummy, "/status", &["ACTIVE"], &["ERROR"]).backoff(fast_backoff());
        let res: WaitOutcome<serde_json::Value> =
            QueryAsync::query_async(&w, &client).await.unwrap();
        match res {
            WaitOutcome::Present(r) => assert_eq!(r["status"], "ACTIVE"),
            WaitOutcome::Gone => panic!("expected Present"),
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn wait_for_status_fails_on_failure_state() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"status": "ERROR"}));
        });

        let w =
            wait_for_status(GetDummy, "/status", &["ACTIVE"], &["ERROR"]).backoff(fast_backoff());
        let res: Result<WaitOutcome<serde_json::Value>, _> = Query::query(&w, &client);
        match res {
            Err(crate::api::ApiError::WaitFailed { reason }) => assert_eq!(reason, "ERROR"),
            other => panic!("expected WaitFailed, got {other:?}"),
        }
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn wait_for_status_fails_on_failure_state_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"status": "ERROR"}));
        });

        let w =
            wait_for_status(GetDummy, "/status", &["ACTIVE"], &["ERROR"]).backoff(fast_backoff());
        let res: Result<WaitOutcome<serde_json::Value>, _> =
            QueryAsync::query_async(&w, &client).await;
        match res {
            Err(crate::api::ApiError::WaitFailed { reason }) => assert_eq!(reason, "ERROR"),
            other => panic!("expected WaitFailed, got {other:?}"),
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn wait_deleted_succeeds_on_404() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(404);
        });

        let w = wait_deleted(GetDummy).backoff(fast_backoff());
        let res: WaitOutcome<serde_json::Value> = Query::query(&w, &client).unwrap();
        assert!(matches!(res, WaitOutcome::Gone));
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn wait_deleted_succeeds_on_404_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(404);
        });

        let w = wait_deleted(GetDummy).backoff(fast_backoff());
        let res: WaitOutcome<serde_json::Value> =
            QueryAsync::query_async(&w, &client).await.unwrap();
        assert!(matches!(res, WaitOutcome::Gone));
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn wait_for_status_vanishing_is_a_failure() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(404);
        });

        let w =
            wait_for_status(GetDummy, "/status", &["ACTIVE"], &["ERROR"]).backoff(fast_backoff());
        let res: Result<WaitOutcome<serde_json::Value>, _> = Query::query(&w, &client);
        match res {
            Err(crate::api::ApiError::WaitFailed { reason }) => {
                assert_eq!(reason, "resource disappeared")
            }
            other => panic!("expected WaitFailed, got {other:?}"),
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn wait_times_out() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"status": "BUILD"}));
        });

        let w = wait_for_status(GetDummy, "/status", &["ACTIVE"], &["ERROR"])
            .backoff(Backoff::fixed(std::time::Duration::from_millis(5)))
            .timeout(std::time::Duration::from_millis(20));
        let res: Result<WaitOutcome<serde_json::Value>, _> = Query::query(&w, &client);
        assert!(matches!(res, Err(crate::api::ApiError::WaitTimeout { .. })));
        assert!(mock.calls() >= 1);
    }

    #[cfg(feature = "sync")]
    #[test]
    fn wait_custom_predicate_receives_observations() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"status": "ACTIVE"}));
        });

        let w = super::wait(GetDummy, |obs: Observation<&DummyResult>| match obs {
            Observation::Present(r) if r.status == "ACTIVE" => WaitDecision::Done,
            Observation::Present(_) => WaitDecision::Continue,
            Observation::Gone => WaitDecision::Fail("gone".into()),
        })
        .backoff(fast_backoff());
        let res: WaitOutcome<DummyResult> = Query::query(&w, &client).unwrap();
        assert!(matches!(res, WaitOutcome::Present(r) if r.status == "ACTIVE"));
        mock.assert();
    }
}
