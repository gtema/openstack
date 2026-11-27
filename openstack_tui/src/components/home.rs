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

use eyre::{Result, WrapErr};
use ratatui::{layout::Rect, prelude::*, widgets::*};
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    cloud_worker::block_storage::v3::{
        BlockStorageApiRequest, BlockStorageQuotaSetApiRequest, BlockStorageQuotaSetShowBuilder,
    },
    cloud_worker::compute::{
        ComputeApiRequest, ComputeQuotaSetApiRequest, ComputeQuotaSetShowBuilder,
    },
    cloud_worker::dns::v2::{
        DnsApiRequest, DnsQuotaApiRequest, DnsQuotaShowBuilder, DnsRecordsetApiRequest,
        DnsRecordsetListBuilder, DnsZoneApiRequest, DnsZoneListBuilder,
    },
    cloud_worker::image::v2::{ImageApiRequest, ImageUsageApiRequest},
    cloud_worker::load_balancer::v2::{
        LoadBalancerApiRequest, LoadBalancerHealthmonitorApiRequest,
        LoadBalancerHealthmonitorListBuilder, LoadBalancerListenerApiRequest,
        LoadBalancerListenerListBuilder, LoadBalancerLoadbalancerApiRequest,
        LoadBalancerLoadbalancerListBuilder, LoadBalancerPoolApiRequest,
        LoadBalancerPoolListBuilder, LoadBalancerQuotaApiRequest, LoadBalancerQuotaShowBuilder,
    },
    cloud_worker::network::v2::{
        NetworkApiRequest, NetworkQuotaApiRequest, NetworkQuotaShowBuilder,
    },
    cloud_worker::types::ApiRequest,
    components::Component,
    config::Config,
    error::TuiError,
    mode::Mode,
};

/// Single resource quota details (used/limit pair). `i64` so that byte-based
/// quotas (e.g. Glance image size) cannot overflow.
#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
struct Quota {
    #[serde(alias = "in_use", alias = "usage", default)]
    used: i64,
    #[serde(default)]
    limit: i64,
}

/// Compute quota details
#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
struct ComputeQuota {
    #[serde(default)]
    cores: Option<Quota>,
    #[serde(default)]
    instances: Option<Quota>,
    #[serde(default)]
    key_pairs: Option<Quota>,
    #[serde(default)]
    metadata_items: Option<Quota>,
    #[serde(default)]
    ram: Option<Quota>,
    #[serde(default)]
    server_group_members: Option<Quota>,
    #[serde(default)]
    server_groups: Option<Quota>,
}

/// Network quota details
#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
struct NetworkQuota {
    #[serde(default)]
    floatingip: Option<Quota>,
    #[serde(default)]
    network: Option<Quota>,
    #[serde(default)]
    port: Option<Quota>,
    #[serde(default)]
    rbac_policy: Option<Quota>,
    #[serde(default)]
    router: Option<Quota>,
    #[serde(default)]
    security_group: Option<Quota>,
    #[serde(default)]
    security_group_rule: Option<Quota>,
    #[serde(default)]
    subnet: Option<Quota>,
    #[serde(default)]
    subnetpool: Option<Quota>,
}

/// Block storage quota details
#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
struct BlockStorageQuota {
    #[serde(default)]
    backup_gigabytes: Option<Quota>,
    #[serde(default)]
    backups: Option<Quota>,
    #[serde(default)]
    gigabytes: Option<Quota>,
    #[serde(default)]
    snapshots: Option<Quota>,
    #[serde(default)]
    volumes: Option<Quota>,
}

/// Load balancer quota limits (the API does not report usage)
#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
struct LbQuota {
    #[serde(default)]
    health_monitor: Option<i32>,
    #[serde(default)]
    healthmonitor: Option<i32>,
    #[serde(default)]
    l7policy: Option<i32>,
    #[serde(default)]
    l7rule: Option<i32>,
    #[serde(default)]
    listener: Option<i32>,
    #[serde(default)]
    load_balancer: Option<i32>,
    #[serde(default)]
    loadbalancer: Option<i32>,
    #[serde(default)]
    member: Option<i32>,
    #[serde(default)]
    pool: Option<i32>,
}

/// DNS quota limits (the API reports limits only, not usage). The response
/// body is a flat object of quota name -> limit.
#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
struct DnsQuotaLimits {
    #[serde(default)]
    api_export_size: Option<i32>,
    #[serde(default)]
    recordset_records: Option<i32>,
    #[serde(default)]
    zone_records: Option<i32>,
    #[serde(default)]
    zone_recordsets: Option<i32>,
    #[serde(default)]
    zones: Option<i32>,
}

/// Image (Glance) quota details, reported by `GET /info/usage` when the
/// `show_image_size_quotas` option is enabled on the server.
#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
struct ImageQuota {
    #[serde(default)]
    image_count_total: Option<Quota>,
    #[serde(default)]
    image_size_total: Option<Quota>,
}

/// Usage counted from the resource lists of services whose quota API
/// does not report usage (load balancer, dns)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct LbUsage {
    loadbalancer: Option<i64>,
    listener: Option<i64>,
    pool: Option<i64>,
    healthmonitor: Option<i64>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct DnsUsage {
    zones: Option<i64>,
    recordsets: Option<i64>,
}

/// A single gauge to be rendered: a title plus the known used/limit values.
/// `limit` of `Some(<= 0)` means unlimited.
#[derive(Debug, Clone, PartialEq, Default)]
struct GaugeData {
    title: &'static str,
    used: Option<i64>,
    limit: Option<i64>,
}

fn gauge(title: &'static str, quota: Option<&Quota>) -> GaugeData {
    GaugeData {
        title,
        used: quota.map(|q| q.used),
        limit: quota.map(|q| q.limit),
    }
}

/// A size gauge whose raw quota values are in bytes, rendered in GiB
fn size_gauge(quota: Option<&Quota>) -> GaugeData {
    const GIB: i64 = 1024 * 1024 * 1024;
    let (used, limit) = match quota {
        None => (None, None),
        Some(q) => (
            Some(q.used / GIB),
            Some(if q.limit > 0 { q.limit / GIB } else { q.limit }),
        ),
    };
    GaugeData {
        title: "Image size (GiB)",
        used,
        limit,
    }
}

fn limit_gauge(title: &'static str, limit: Option<i32>, used: Option<i64>) -> GaugeData {
    GaugeData {
        title,
        used,
        limit: limit.map(i64::from),
    }
}

/// Layout of the usage grid. Gauges have a fixed (non-stretching) width so the
/// view is not flooded with edge-to-edge widgets on wide terminals; the grid is
/// centred horizontally. On narrow terminals the gauge width shrinks to fit.
const COLUMNS: usize = 5;
const GAUGE_WIDTH: u16 = 20;
const GAUGE_HEIGHT: u16 = 4;

/// One row of the usage view: a section separator, a service header, or up to
/// `COLUMNS` gauges laid out on a fixed-width grid.
#[derive(Debug, Clone, PartialEq)]
enum HomeRow {
    /// Empty line used as padding between service sections
    Blank,
    Header(&'static str),
    Gauges(Vec<GaugeData>),
}

impl HomeRow {
    fn lines(&self) -> u16 {
        match self {
            HomeRow::Blank => 1,
            HomeRow::Header(_) => 1,
            HomeRow::Gauges(_) => GAUGE_HEIGHT,
        }
    }
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    area: Rect,
    is_loading: bool,
    is_error: bool,
    project_id: Option<String>,
    /// Index of the first visible row
    scroll: usize,
    /// Number of API responses still expected from the last refresh
    pending: u32,
    compute_quota: Option<ComputeQuota>,
    network_quota: Option<NetworkQuota>,
    block_storage_quota: Option<BlockStorageQuota>,
    lb_quota: Option<LbQuota>,
    dns_quota: Option<DnsQuotaLimits>,
    image_quota: Option<ImageQuota>,
    lb_usage: LbUsage,
    dns_usage: DnsUsage,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {}

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    fn reset_data(&mut self) {
        self.is_error = false;
        self.set_loading(true);
        self.scroll = 0;
        self.pending = 0;
        self.compute_quota = None;
        self.network_quota = None;
        self.block_storage_quota = None;
        self.lb_quota = None;
        self.dns_quota = None;
        self.image_quota = None;
        self.lb_usage = LbUsage::default();
        self.dns_usage = DnsUsage::default();
    }

    fn set_compute_data(&mut self, data: Value) -> Result<(), TuiError> {
        if !data.is_null() {
            self.compute_quota = Some(serde_json::from_value(data)?);
        }
        Ok(())
    }

    fn set_network_data(&mut self, data: Value) -> Result<(), TuiError> {
        if !data.is_null() {
            self.network_quota = Some(serde_json::from_value(data)?);
        }
        Ok(())
    }

    fn set_block_storage_data(&mut self, data: Value) -> Result<(), TuiError> {
        if !data.is_null() {
            self.block_storage_quota = Some(serde_json::from_value(data)?);
        }
        Ok(())
    }

    fn set_lb_quota_data(&mut self, data: Value) -> Result<(), TuiError> {
        if !data.is_null() {
            self.lb_quota = Some(serde_json::from_value(data)?);
        }
        Ok(())
    }

    fn set_dns_quota_data(&mut self, data: Value) -> Result<(), TuiError> {
        if !data.is_null() {
            self.dns_quota = Some(serde_json::from_value(data)?);
        }
        Ok(())
    }

    fn set_image_data(&mut self, data: Value) -> Result<(), TuiError> {
        if !data.is_null() {
            // Glance wraps the values: `{"usage": {"image_size_total": ..}}`
            let data = match data {
                Value::Object(mut map) if map.get("usage").is_some_and(Value::is_object) => {
                    map.remove("usage").unwrap_or_default()
                }
                other => other,
            };
            self.image_quota = Some(serde_json::from_value(data)?);
        }
        Ok(())
    }

    fn note_response(&mut self) {
        if self.pending == 0 {
            return;
        }
        self.pending = self.pending.saturating_sub(1);
        if self.pending == 0 {
            self.is_loading = false;
        }
    }

    /// All API requests needed to fill the usage view
    fn refresh_data(&mut self) -> Result<Option<Action>, TuiError> {
        if let Some(command_tx) = self.command_tx.clone()
            && let Some(project_id) = self.project_id.clone()
        {
            let requests = vec![
                ApiRequest::from(ComputeQuotaSetApiRequest::Details(Box::new(
                    ComputeQuotaSetShowBuilder::default()
                        .id(project_id.clone())
                        .build()
                        .wrap_err("cannot prepare compute quota request")?,
                ))),
                ApiRequest::from(NetworkQuotaApiRequest::Details(Box::new(
                    NetworkQuotaShowBuilder::default()
                        .id(project_id.clone())
                        .build()
                        .wrap_err("cannot prepare network quota request")?,
                ))),
                ApiRequest::from(BlockStorageQuotaSetApiRequest::Get(Box::new(
                    BlockStorageQuotaSetShowBuilder::default()
                        .project_id(project_id.clone())
                        .usage(true)
                        .build()
                        .wrap_err("cannot prepare block storage quota request")?,
                ))),
                ApiRequest::from(LoadBalancerQuotaApiRequest::Get(Box::new(
                    LoadBalancerQuotaShowBuilder::default()
                        .project_id(project_id.clone())
                        .build()
                        .wrap_err("cannot prepare load balancer quota request")?,
                ))),
                ApiRequest::from(DnsQuotaApiRequest::Get(Box::new(
                    DnsQuotaShowBuilder::default()
                        .project_id(project_id.clone())
                        .build()
                        .wrap_err("cannot prepare dns quota request")?,
                ))),
                ApiRequest::from(ImageUsageApiRequest::List(Box::default())),
                // Usage of the services whose quota API does not report it
                // is counted from the resource lists (load balancer lists are
                // scoped to the project, as admins would see all projects)
                ApiRequest::from(LoadBalancerLoadbalancerApiRequest::List(Box::new(
                    LoadBalancerLoadbalancerListBuilder::default()
                        .project_id(project_id.clone())
                        .build()
                        .wrap_err("cannot prepare loadbalancer list request")?,
                ))),
                ApiRequest::from(LoadBalancerListenerApiRequest::List(Box::new(
                    LoadBalancerListenerListBuilder::default()
                        .project_id(project_id.clone())
                        .build()
                        .wrap_err("cannot prepare listener list request")?,
                ))),
                ApiRequest::from(LoadBalancerPoolApiRequest::List(Box::new(
                    LoadBalancerPoolListBuilder::default()
                        .project_id(project_id.clone())
                        .build()
                        .wrap_err("cannot prepare pool list request")?,
                ))),
                ApiRequest::from(LoadBalancerHealthmonitorApiRequest::List(Box::new(
                    LoadBalancerHealthmonitorListBuilder::default()
                        .project_id(project_id.clone())
                        .build()
                        .wrap_err("cannot prepare healthmonitor list request")?,
                ))),
                ApiRequest::from(DnsZoneApiRequest::List(Box::new(
                    DnsZoneListBuilder::default()
                        .build()
                        .wrap_err("cannot prepare dns zone list request")?,
                ))),
                ApiRequest::from(DnsRecordsetApiRequest::List(Box::new(
                    DnsRecordsetListBuilder::default()
                        .build()
                        .wrap_err("cannot prepare dns recordset list request")?,
                ))),
            ];
            self.reset_data();
            self.pending = u32::try_from(requests.len()).unwrap_or(u32::MAX);
            for request in requests {
                command_tx.send(Action::PerformApiRequest(request))?;
            }
        }
        Ok(None)
    }

    /// Kind of usage count an (unfiltered) list request provides, if any
    fn count_request(request: &ApiRequest) -> Option<&'static str> {
        request.usage_count_kind()
    }

    /// Height of the area the rows are drawn into (inside the borders)
    fn visible_lines(&self) -> u16 {
        self.area.height.saturating_sub(2)
    }

    /// Largest scroll position that still fills the viewport: scrolling
    /// further would only add blank space. Zero when everything fits.
    fn max_scroll(rows: &[HomeRow], visible_lines: u16) -> usize {
        let mut acc = 0u16;
        let mut start = rows.len();
        for (i, row) in rows.iter().enumerate().rev() {
            acc = acc.saturating_add(row.lines());
            if acc > visible_lines {
                break;
            }
            start = i;
        }
        start.min(rows.len().saturating_sub(1))
    }

    fn handle_scroll(&mut self, action: &Action) {
        let rows = self.build_rows();
        if rows.is_empty() {
            self.scroll = 0;
            return;
        }
        let visible = self.visible_lines();
        let max = Self::max_scroll(&rows, visible);
        let current = self.scroll.min(max);
        match action {
            Action::HomeScrollUp => self.scroll = current.saturating_sub(1),
            Action::HomeScrollDown => self.scroll = (current + 1).min(max),
            Action::HomeScrollTop => self.scroll = 0,
            Action::HomeScrollEnd => self.scroll = max,
            Action::HomePageUp => {
                // Rows that fit above the current position
                let mut acc = 0u16;
                let mut count = 0usize;
                for row in rows[..current].iter().rev() {
                    acc = acc.saturating_add(row.lines());
                    if count > 0 && acc > visible {
                        break;
                    }
                    count += 1;
                }
                self.scroll = current.saturating_sub(count.max(1));
            }
            Action::HomePageDown => {
                let page = Self::page_rows(&rows[current..], visible);
                self.scroll = (current + page).min(max);
            }
            _ => {}
        }
    }

    /// Number of rows that fit into `visible_lines`, at least one
    fn page_rows(rows: &[HomeRow], visible_lines: u16) -> usize {
        let mut acc = 0u16;
        let mut count = 0usize;
        for row in rows {
            let h = row.lines();
            if count > 0 && acc.saturating_add(h) > visible_lines {
                break;
            }
            acc = acc.saturating_add(h);
            count += 1;
        }
        count.max(1)
    }

    /// The rows of the usage view, grouped by service. Each section is a header
    /// followed by gauge rows of up to `COLUMNS` gauges; sections are separated
    /// by a blank line.
    fn build_rows(&self) -> Vec<HomeRow> {
        let mut rows = Vec::new();
        let push_section =
            |rows: &mut Vec<HomeRow>, title: &'static str, gauges: Vec<GaugeData>| {
                if !rows.is_empty() {
                    rows.push(HomeRow::Blank);
                }
                rows.push(HomeRow::Header(title));
                for chunk in gauges.chunks(COLUMNS) {
                    rows.push(HomeRow::Gauges(chunk.to_vec()));
                }
            };

        if let Some(cq) = &self.compute_quota {
            push_section(
                &mut rows,
                "Compute",
                vec![
                    gauge("Instances", cq.instances.as_ref()),
                    gauge("CPU", cq.cores.as_ref()),
                    gauge("RAM (MB)", cq.ram.as_ref()),
                    gauge("Key pairs", cq.key_pairs.as_ref()),
                    gauge("Metadata items", cq.metadata_items.as_ref()),
                    gauge("Server groups", cq.server_groups.as_ref()),
                    gauge("SG members", cq.server_group_members.as_ref()),
                ],
            );
        }
        if let Some(nq) = &self.network_quota {
            push_section(
                &mut rows,
                "Network",
                vec![
                    gauge("Floating IPs", nq.floatingip.as_ref()),
                    gauge("Networks", nq.network.as_ref()),
                    gauge("Subnets", nq.subnet.as_ref()),
                    gauge("Ports", nq.port.as_ref()),
                    gauge("Routers", nq.router.as_ref()),
                    gauge("Security groups", nq.security_group.as_ref()),
                    gauge("SG rules", nq.security_group_rule.as_ref()),
                    gauge("Subnetpools", nq.subnetpool.as_ref()),
                    gauge("RBAC policies", nq.rbac_policy.as_ref()),
                ],
            );
        }
        if let Some(bq) = &self.block_storage_quota {
            push_section(
                &mut rows,
                "Block storage",
                vec![
                    gauge("Volumes", bq.volumes.as_ref()),
                    gauge("Volume GB", bq.gigabytes.as_ref()),
                    gauge("Snapshots", bq.snapshots.as_ref()),
                    gauge("Backups", bq.backups.as_ref()),
                    gauge("Backup GB", bq.backup_gigabytes.as_ref()),
                ],
            );
        }
        if self.lb_quota.is_some()
            || self.lb_usage.loadbalancer.is_some()
            || self.lb_usage.listener.is_some()
            || self.lb_usage.pool.is_some()
            || self.lb_usage.healthmonitor.is_some()
        {
            let default_lq = LbQuota::default();
            let lq = self.lb_quota.as_ref().unwrap_or(&default_lq);
            let lu = self.lb_usage;
            push_section(
                &mut rows,
                "Load balancer",
                vec![
                    limit_gauge(
                        "Load balancers",
                        lq.loadbalancer.or(lq.load_balancer),
                        lu.loadbalancer,
                    ),
                    limit_gauge("Listeners", lq.listener, lu.listener),
                    limit_gauge("Pools", lq.pool, lu.pool),
                    limit_gauge("Members", lq.member, None),
                    limit_gauge(
                        "Health monitors",
                        lq.healthmonitor.or(lq.health_monitor),
                        lu.healthmonitor,
                    ),
                    limit_gauge("L7 policies", lq.l7policy, None),
                    limit_gauge("L7 rules", lq.l7rule, None),
                ],
            );
        }
        if self.dns_quota.is_some()
            || self.dns_usage.zones.is_some()
            || self.dns_usage.recordsets.is_some()
        {
            let default_dq = DnsQuotaLimits::default();
            let dq = self.dns_quota.as_ref().unwrap_or(&default_dq);
            let du = self.dns_usage;
            push_section(
                &mut rows,
                "DNS",
                vec![
                    limit_gauge("Zones", dq.zones, du.zones),
                    limit_gauge("Zone recordsets", dq.zone_recordsets, du.recordsets),
                    limit_gauge("Zone records", dq.zone_records, None),
                    limit_gauge("Records", dq.recordset_records, None),
                    limit_gauge("Export size", dq.api_export_size, None),
                ],
            );
        }
        // Glance only reports these when `show_image_size_quotas` is enabled;
        // hide the section entirely when nothing was reported.
        if let Some(iq) = &self.image_quota
            && (iq.image_count_total.is_some() || iq.image_size_total.is_some())
        {
            push_section(
                &mut rows,
                "Images",
                vec![
                    gauge("Images", iq.image_count_total.as_ref()),
                    size_gauge(iq.image_size_total.as_ref()),
                ],
            );
        }
        rows
    }

    fn render_gauge(&self, gauge: &GaugeData, f: &mut Frame, area: Rect) {
        let (ratio, label, color) = match (gauge.used, gauge.limit) {
            (None, None) => (0.0, "N/A".to_string(), Color::DarkGray),
            (Some(used), None) => (0.0, format!("used {used}"), self.config.styles.fg),
            (None, Some(limit)) if limit > 0 => {
                (0.0, format!("limit {limit}"), self.config.styles.fg)
            }
            (None, Some(_)) => (0.0, "limit ∞".to_string(), self.config.styles.fg),
            (Some(used), Some(limit)) if limit > 0 => {
                let ratio = used as f64 / limit as f64;
                let ratio = ratio.clamp(0.0, 1.0);
                let color = match ratio {
                    0.0..0.5 => Color::Green,
                    0.5..0.8 => Color::Yellow,
                    _ => Color::Red,
                };
                (ratio, format!("used {used}/{limit}"), color)
            }
            (Some(used), Some(_)) => (0.0, format!("used {used}/∞"), Color::Green),
        };
        let gauge_widget = ratatui::widgets::Gauge::default()
            .block(Block::bordered().title(gauge.title))
            .label(Span::styled(label, color))
            .gauge_style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .ratio(ratio);
        f.render_widget(gauge_widget, area);
    }
}

impl Component for Home {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action, current_mode: Mode) -> Result<Option<Action>, TuiError> {
        match action {
            Action::ConnectToCloud(_) | Action::CloudChangeScope(_) | Action::SwitchToRegion(_) => {
                self.reset_data();
            }
            Action::ConnectedToCloud(auth) => {
                self.reset_data();
                if let Some(project) = auth.project
                    && let Some(pid) = project.id
                {
                    self.project_id = Some(pid);
                }

                if current_mode == Mode::Home {
                    return self.refresh_data();
                }
            }
            Action::Mode {
                mode: Mode::Home, ..
            } if !self.is_loading => {
                return self.refresh_data();
            }
            Action::Refresh if current_mode == Mode::Home && !self.is_loading => {
                return self.refresh_data();
            }
            Action::HomeScrollUp
            | Action::HomeScrollDown
            | Action::HomePageUp
            | Action::HomePageDown
            | Action::HomeScrollTop
            | Action::HomeScrollEnd
                if current_mode == Mode::Home =>
            {
                self.handle_scroll(&action);
            }
            Action::Tick => {
                self.tick();
            }
            Action::ApiResponseData { request, data } => {
                let result = match request {
                    ApiRequest::Compute(ComputeApiRequest::QuotaSet(_)) => {
                        self.set_compute_data(data)
                    }
                    ApiRequest::Network(NetworkApiRequest::Quota(_)) => self.set_network_data(data),
                    ApiRequest::BlockStorage(BlockStorageApiRequest::QuotaSet(_)) => {
                        self.set_block_storage_data(data)
                    }
                    ApiRequest::LoadBalancer(LoadBalancerApiRequest::Quota(_)) => {
                        self.set_lb_quota_data(data)
                    }
                    ApiRequest::Dns(DnsApiRequest::Quota(_)) => self.set_dns_quota_data(data),
                    ApiRequest::Image(ImageApiRequest::Usage(_)) => self.set_image_data(data),
                    _ => Ok(()),
                };
                // Count the response even when it cannot be parsed so that
                // the view never stays in the loading state.
                self.note_response();
                result?;
            }
            Action::ApiResponsesData { request, data, .. } => {
                if self.is_loading
                    && let Some(kind) = Self::count_request(&request)
                {
                    let count = i64::try_from(data.len()).unwrap_or(i64::MAX);
                    match kind {
                        "loadbalancer" => self.lb_usage.loadbalancer = Some(count),
                        "listener" => self.lb_usage.listener = Some(count),
                        "pool" => self.lb_usage.pool = Some(count),
                        "healthmonitor" => self.lb_usage.healthmonitor = Some(count),
                        "zone" => self.dns_usage.zones = Some(count),
                        "recordset" => self.dns_usage.recordsets = Some(count),
                        _ => unreachable!("unknown usage count kind {kind}"),
                    }
                    self.note_response();
                }
            }

            Action::Error { action, .. } => {
                // A failed usage probe is not an error: the corresponding
                // quota is simply omitted from the view. Account for it
                // regardless of the current mode.
                if action.as_deref().is_some_and(
                    |a| matches!(a, Action::PerformApiRequest(r) if r.is_usage_probe()),
                ) {
                    if self.is_loading {
                        self.note_response();
                    }
                } else if current_mode == Mode::Home {
                    self.is_error = true;
                    self.set_loading(false);
                    self.pending = 0;
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<(), TuiError> {
        self.area = area;
        let rows = self.build_rows();
        let max_scroll = Self::max_scroll(&rows, area.height.saturating_sub(2));
        let scroll = self.scroll.min(max_scroll);

        let mut title = vec![" Usage ".white()];
        if self.is_loading {
            title.push(Span::styled(
                " ...Loading... ",
                self.config.styles.title_loading_fg,
            ));
        } else if self.is_error {
            title.push(Span::styled(
                " (Cannot be fetched) ",
                self.config.styles.title_loading_fg,
            ));
        }
        if max_scroll > 0 {
            title.push(Span::styled(
                format!(" {scroll}/{max_scroll} "),
                self.config.styles.title_filters_fg,
            ));
        }

        let block = Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(self.config.styles.border_fg));

        let inner = block.inner(area);
        f.render_widget(block, area);

        if rows.is_empty() && !self.is_loading {
            let paragraph = Paragraph::new("Not available")
                .style(Style::new().red().on_black())
                .alignment(Alignment::Center);
            f.render_widget(paragraph, inner);
        }

        // Fixed-width grid, centred horizontally so gauges are clearly
        // separated instead of stretching edge-to-edge on wide terminals.
        let columns = u16::try_from(COLUMNS).unwrap_or(u16::MAX);
        let grid_w = GAUGE_WIDTH.saturating_mul(columns);
        let gauge_w = if inner.width >= grid_w {
            GAUGE_WIDTH
        } else {
            (inner.width / columns).max(1)
        };
        let grid_width = gauge_w.saturating_mul(columns);
        let x_off = inner.x + inner.width.saturating_sub(grid_width) / 2;

        let mut y = inner.y;
        for row in rows.iter().skip(scroll) {
            let remaining = inner.height.saturating_sub(y.saturating_sub(inner.y));
            if remaining == 0 {
                break;
            }
            let h = row.lines().min(remaining);
            match row {
                HomeRow::Blank => {}
                HomeRow::Header(text) => {
                    let title_w = u16::try_from(text.len() + 2).unwrap_or(grid_width);
                    let filler = "─".repeat(usize::from(grid_width.saturating_sub(title_w)));
                    let paragraph = Paragraph::new(Line::from(vec![
                        Span::styled(
                            format!(" {text} "),
                            Style::default()
                                .fg(self.config.styles.title_details_fg)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(filler, Style::default().fg(self.config.styles.border_fg)),
                    ]));
                    f.render_widget(paragraph, Rect::new(x_off, y, grid_width, h));
                }
                HomeRow::Gauges(gauges) => {
                    for (i, gauge) in gauges.iter().enumerate() {
                        let cell_x = x_off + (i as u16).saturating_mul(gauge_w);
                        self.render_gauge(gauge, f, Rect::new(cell_x, y, gauge_w, h));
                    }
                }
            }
            y += h;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_worker::block_storage::v3::BlockStorageQuotaSetShow;
    use crate::cloud_worker::dns::v2::DnsQuotaShow;
    use crate::cloud_worker::load_balancer::v2::LoadBalancerLoadbalancerList;
    use crate::cloud_worker::network::v2::NetworkQuotaShow;
    use serde_json::json;

    fn make_home() -> (Home, tokio::sync::mpsc::UnboundedReceiver<Action>) {
        let mut home = Home::new();
        home.register_config_handler(Config::default()).unwrap();
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        home.register_action_handler(tx).unwrap();
        (home, rx)
    }

    fn compute_json() -> Value {
        json!({
            "instances": {"in_use": 3, "limit": 10, "reserved": 0},
            "cores": {"in_use": 4, "limit": 20, "reserved": 0},
            "ram": {"in_use": 8192, "limit": 65536, "reserved": 0},
            "key_pairs": {"in_use": 1, "limit": 100, "reserved": 0},
            "metadata_items": {"in_use": 0, "limit": 128, "reserved": 0},
            "server_groups": {"in_use": 0, "limit": 10, "reserved": 0},
            "server_group_members": {"in_use": 0, "limit": 10, "reserved": 0}
        })
    }

    fn network_json() -> Value {
        json!({
            "floatingip": {"used": 2, "limit": 10, "reserved": 0},
            "network": {"used": 2, "limit": 10, "reserved": 0},
            "subnet": {"used": 2, "limit": 10, "reserved": 0},
            "port": {"used": 5, "limit": 50, "reserved": 0},
            "router": {"used": 1, "limit": 10, "reserved": 0},
            "security_group": {"used": 2, "limit": 10, "reserved": 0},
            "security_group_rule": {"used": 10, "limit": 100, "reserved": 0},
            "subnetpool": {"used": 0, "limit": 10, "reserved": 0},
            "rbac_policy": {"used": 0, "limit": 10, "reserved": 0}
        })
    }

    fn block_storage_json() -> Value {
        json!({
            "volumes": {"limit": 10, "reserved": 0, "used": 2},
            "gigabytes": {"limit": 1000, "reserved": 0, "used": 50},
            "snapshots": {"limit": 10, "reserved": 0, "used": 0},
            "backups": {"limit": 10, "reserved": 0, "used": 0},
            "backup_gigabytes": {"limit": 1000, "reserved": 0, "used": 0},
            "project_id": "proj-1"
        })
    }

    fn lb_quota_json() -> Value {
        json!({
            "loadbalancer": 10,
            "listener": 10,
            "pool": 10,
            "member": 10,
            "healthmonitor": 10,
            "l7policy": 10,
            "l7rule": 10
        })
    }

    fn dns_quota_json() -> Value {
        json!({
            "zones": 10,
            "zone_recordsets": 30,
            "zone_records": 300,
            "recordset_records": 30,
            "api_export_size": 100
        })
    }

    fn image_quota_json() -> Value {
        json!({
            "usage": {
                "image_count_total": {"usage": 5, "limit": 100},
                "image_size_total": {"usage": 3221225472i64, "limit": 107374182400i64},
                "image_stage_total": {"usage": 0, "limit": 100},
                "image_count_uploading": {"usage": 0, "limit": 100}
            }
        })
    }

    #[test]
    fn quota_deserializes_with_in_use_alias() {
        let q: Quota = serde_json::from_value(json!({"in_use": 1, "limit": 2})).unwrap();
        assert_eq!(q.used, 1);
        assert_eq!(q.limit, 2);
        let q: Quota = serde_json::from_value(json!({"used": 1, "limit": 2})).unwrap();
        assert_eq!(q.used, 1);
        let q: Quota = serde_json::from_value(json!({"limit": 2})).unwrap();
        assert_eq!(q.used, 0);
    }

    #[test]
    fn all_quota_payloads_deserialize() {
        let c: ComputeQuota = serde_json::from_value(compute_json()).unwrap();
        assert_eq!(c.instances.unwrap().used, 3);
        let n: NetworkQuota = serde_json::from_value(network_json()).unwrap();
        assert_eq!(n.floatingip.unwrap().used, 2);
        let b: BlockStorageQuota = serde_json::from_value(block_storage_json()).unwrap();
        assert_eq!(b.volumes.unwrap().used, 2);
        let l: LbQuota = serde_json::from_value(lb_quota_json()).unwrap();
        assert_eq!(l.loadbalancer, Some(10));
        let d: DnsQuotaLimits = serde_json::from_value(dns_quota_json()).unwrap();
        assert_eq!(d.zones, Some(10));
    }

    #[test]
    fn refresh_sends_all_quota_requests() {
        let (mut home, mut rx) = make_home();
        home.project_id = Some("proj-1".into());
        home.set_loading(true);
        home.refresh_data().unwrap();
        let mut count = 0;
        while let Ok(Action::PerformApiRequest(_)) = rx.try_recv() {
            count += 1;
        }
        assert_eq!(count, 12);
        assert_eq!(home.pending, 12);
        assert!(home.is_loading);
    }

    fn sample_data_for(request: &ApiRequest) -> Value {
        match request {
            ApiRequest::Compute(ComputeApiRequest::QuotaSet(_)) => compute_json(),
            ApiRequest::Network(NetworkApiRequest::Quota(_)) => network_json(),
            ApiRequest::BlockStorage(BlockStorageApiRequest::QuotaSet(_)) => block_storage_json(),
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Quota(_)) => lb_quota_json(),
            ApiRequest::Dns(DnsApiRequest::Quota(_)) => dns_quota_json(),
            ApiRequest::Image(ImageApiRequest::Usage(_)) => image_quota_json(),
            _ => Value::Null,
        }
    }

    #[test]
    fn responses_fill_data_and_clear_loading() {
        let (mut home, mut rx) = make_home();
        home.project_id = Some("proj-1".into());
        home.refresh_data().unwrap();
        let requests: Vec<ApiRequest> = std::iter::from_fn(|| match rx.try_recv() {
            Ok(Action::PerformApiRequest(r)) => Some(r),
            _ => None,
        })
        .collect();
        assert_eq!(requests.len(), 12);

        for request in requests {
            let is_list = Home::count_request(&request).is_some();
            let action = if is_list {
                Action::ApiResponsesData {
                    request: request.clone(),
                    data: vec![json!({"id": "x"})],
                    negotiated_version: None,
                }
            } else {
                Action::ApiResponseData {
                    request: request.clone(),
                    data: sample_data_for(&request),
                }
            };
            home.update(action, Mode::Home).unwrap();
        }

        assert!(!home.is_loading);
        assert_eq!(home.lb_usage.loadbalancer, Some(1));
        assert_eq!(home.dns_usage.zones, Some(1));
        assert!(home.compute_quota.is_some());
        assert!(home.network_quota.is_some());
        assert!(home.block_storage_quota.is_some());
        assert!(home.lb_quota.is_some());
        assert!(home.dns_quota.is_some());
        assert!(home.image_quota.is_some());
    }

    #[test]
    fn error_stops_loading() {
        let (mut home, _rx) = make_home();
        home.project_id = Some("proj-1".into());
        home.refresh_data().unwrap();
        home.update(
            Action::Error {
                msg: "no service".into(),
                action: None,
            },
            Mode::Home,
        )
        .unwrap();
        assert!(!home.is_loading);
        assert!(home.is_error);
        assert_eq!(home.pending, 0);
    }

    #[test]
    fn usage_probe_error_is_not_fatal() {
        let (mut home, _rx) = make_home();
        home.project_id = Some("proj-1".into());
        home.set_loading(true);
        home.refresh_data().unwrap();
        assert_eq!(home.pending, 12);

        let image_req = ApiRequest::from(ImageUsageApiRequest::List(Box::default()));
        home.update(
            Action::Error {
                msg: "404".into(),
                action: Some(Box::new(Action::PerformApiRequest(image_req))),
            },
            Mode::Home,
        )
        .unwrap();

        // A failed usage probe is not an error: the quota is simply omitted
        assert!(!home.is_error);
        assert_eq!(home.pending, 11);
        assert!(home.is_loading);
    }

    #[test]
    fn build_rows_groups_services() {
        let (mut home, _rx) = make_home();
        home.set_compute_data(compute_json()).unwrap();
        home.set_network_data(network_json()).unwrap();
        home.set_block_storage_data(block_storage_json()).unwrap();
        home.set_lb_quota_data(lb_quota_json()).unwrap();
        home.set_dns_quota_data(dns_quota_json()).unwrap();
        home.set_image_data(image_quota_json()).unwrap();
        home.lb_usage = LbUsage {
            loadbalancer: Some(1),
            listener: Some(2),
            pool: Some(3),
            healthmonitor: Some(4),
        };
        home.dns_usage = DnsUsage {
            zones: Some(2),
            recordsets: Some(7),
        };

        let rows = home.build_rows();
        assert_eq!(rows.len(), 20);
        assert_eq!(rows[0], HomeRow::Header("Compute"));
        assert_eq!(rows[3], HomeRow::Blank);
        assert_eq!(rows[4], HomeRow::Header("Network"));
        assert_eq!(rows[8], HomeRow::Header("Block storage"));
        assert_eq!(rows[11], HomeRow::Header("Load balancer"));
        assert_eq!(rows[15], HomeRow::Header("DNS"));
        assert_eq!(rows[18], HomeRow::Header("Images"));
        let total_lines: u16 = rows.iter().map(HomeRow::lines).sum();
        assert_eq!(total_lines, 6 + 5 + 9 * 4);

        if let HomeRow::Gauges(gauges) = &rows[1] {
            assert_eq!(gauges[0].title, "Instances");
            assert_eq!(gauges[0].used, Some(3));
            assert_eq!(gauges[0].limit, Some(10));
        } else {
            panic!("expected gauge row");
        }
        if let HomeRow::Gauges(gauges) = &rows[12] {
            assert_eq!(gauges[0].title, "Load balancers");
            assert_eq!(gauges[0].used, Some(1));
            assert_eq!(gauges[0].limit, Some(10));
        } else {
            panic!("expected gauge row");
        }
        if let HomeRow::Gauges(gauges) = &rows[16] {
            assert_eq!(gauges[0].title, "Zones");
            assert_eq!(gauges[0].used, Some(2));
            assert_eq!(gauges[0].limit, Some(10));
        } else {
            panic!("expected gauge row");
        }
        if let HomeRow::Gauges(gauges) = &rows[19] {
            assert_eq!(gauges[0].title, "Images");
            assert_eq!(gauges[0].used, Some(5));
            assert_eq!(gauges[0].limit, Some(100));
            assert_eq!(gauges[1].title, "Image size (GiB)");
            assert_eq!(gauges[1].used, Some(3));
            assert_eq!(gauges[1].limit, Some(100));
        } else {
            panic!("expected gauge row");
        }
    }

    #[test]
    fn build_rows_empty_without_data() {
        let (home, _rx) = make_home();
        assert!(home.build_rows().is_empty());
    }

    #[test]
    fn count_request_matches_only_unfiltered_lists() {
        let req = ApiRequest::from(LoadBalancerLoadbalancerApiRequest::List(Box::default()));
        assert_eq!(Home::count_request(&req), Some("loadbalancer"));

        let filtered = LoadBalancerLoadbalancerList {
            name: Some("x".into()),
            ..Default::default()
        };
        let req = ApiRequest::from(LoadBalancerLoadbalancerApiRequest::List(Box::new(filtered)));
        assert_eq!(Home::count_request(&req), None);

        let req = ApiRequest::from(DnsZoneApiRequest::List(Box::default()));
        assert_eq!(Home::count_request(&req), Some("zone"));

        let req = ApiRequest::from(DnsQuotaApiRequest::Get(Box::new(DnsQuotaShow {
            project_id: "p".into(),
        })));
        assert_eq!(Home::count_request(&req), None);

        let req = ApiRequest::from(BlockStorageQuotaSetApiRequest::Get(Box::new(
            BlockStorageQuotaSetShow {
                project_id: "p".into(),
                usage: Some(true),
            },
        )));
        assert_eq!(Home::count_request(&req), None);

        let req = ApiRequest::from(NetworkQuotaApiRequest::Details(Box::new(
            NetworkQuotaShow {
                id: "p".into(),
                name: None,
            },
        )));
        assert_eq!(Home::count_request(&req), None);
    }

    #[test]
    fn scroll_stays_within_bounds() {
        let (mut home, _rx) = make_home();
        home.set_compute_data(compute_json()).unwrap();
        let rows = home.build_rows();
        let max = rows.len() - 1;

        home.update(Action::HomeScrollEnd, Mode::Home).unwrap();
        assert_eq!(home.scroll, max);

        home.update(Action::HomeScrollDown, Mode::Home).unwrap();
        assert_eq!(home.scroll, max, "scroll must not exceed the last row");

        home.update(Action::HomeScrollTop, Mode::Home).unwrap();
        assert_eq!(home.scroll, 0);

        home.update(Action::HomeScrollUp, Mode::Home).unwrap();
        assert_eq!(home.scroll, 0, "scroll must not go below zero");

        home.update(Action::HomeScrollDown, Mode::Home).unwrap();
        assert_eq!(home.scroll, 1);
        home.update(Action::HomeScrollUp, Mode::Home).unwrap();
        assert_eq!(home.scroll, 0);
    }

    #[test]
    fn scroll_ignored_outside_home_mode() {
        let (mut home, _rx) = make_home();
        home.set_compute_data(compute_json()).unwrap();
        home.update(Action::HomeScrollEnd, Mode::Describe).unwrap();
        assert_eq!(home.scroll, 0);
    }

    fn render_to_string(width: u16, height: u16, home: &mut Home) -> String {
        use ratatui::Terminal;
        use ratatui::backend::TestBackend;

        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                home.draw(f, f.area()).unwrap();
            })
            .unwrap();
        let buf = terminal.backend().buffer();
        (0..buf.area().height)
            .map(|y| {
                (0..buf.area().width)
                    .map(|x| {
                        buf.cell((x, y))
                            .map(|c| c.symbol().chars().next().unwrap_or(' '))
                            .unwrap_or(' ')
                    })
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn render_shows_sections_and_scroll_indicator() {
        let (mut home, _rx) = make_home();
        home.set_compute_data(compute_json()).unwrap();
        home.set_network_data(network_json()).unwrap();
        home.set_block_storage_data(block_storage_json()).unwrap();
        home.set_lb_quota_data(lb_quota_json()).unwrap();
        home.set_dns_quota_data(dns_quota_json()).unwrap();
        home.is_loading = false;

        let out = render_to_string(100, 24, &mut home);
        assert!(out.contains("Usage"), "title missing:\n{out}");
        assert!(out.contains("Compute"), "compute header missing:\n{out}");
        assert!(out.contains("Instances"), "instances gauge missing:\n{out}");
        assert!(out.contains("used 3/10"), "instances usage missing:\n{out}");
        assert!(out.contains("0/7"), "scroll indicator missing:\n{out}");
    }

    #[test]
    fn render_scrolled_shows_later_sections() {
        let (mut home, _rx) = make_home();
        home.set_compute_data(compute_json()).unwrap();
        home.set_network_data(network_json()).unwrap();
        home.set_block_storage_data(block_storage_json()).unwrap();
        home.set_lb_quota_data(lb_quota_json()).unwrap();
        home.set_dns_quota_data(dns_quota_json()).unwrap();
        home.is_loading = false;
        // Render once so that the view knows its size
        render_to_string(100, 24, &mut home);
        home.update(Action::HomeScrollEnd, Mode::Home).unwrap();

        let out = render_to_string(100, 24, &mut home);
        assert!(out.contains("Zones"), "zones gauge missing:\n{out}");
        assert!(
            out.contains("7/7"),
            "scroll indicator should be at the end:\n{out}"
        );
        assert!(
            !out.contains("Compute"),
            "compute header should be scrolled out:\n{out}"
        );
        assert!(
            !out.contains("Instances"),
            "instances gauge should be scrolled out:\n{out}"
        );
    }

    #[test]
    fn max_scroll_zero_when_everything_fits() {
        let rows = vec![
            HomeRow::Header("A"),
            HomeRow::Gauges(vec![]),
            HomeRow::Blank,
        ];
        let total: u16 = rows.iter().map(HomeRow::lines).sum();
        assert_eq!(Home::max_scroll(&rows, total), 0);
        assert_eq!(Home::max_scroll(&rows, total + 10), 0);
    }

    #[test]
    fn max_scroll_stops_when_viewport_still_filled() {
        let rows = vec![
            HomeRow::Header("A"),
            HomeRow::Gauges(vec![]),
            HomeRow::Gauges(vec![]),
        ];
        let last = rows[2].lines();
        assert_eq!(Home::max_scroll(&rows, last), 2);
        assert_eq!(Home::max_scroll(&rows, last + rows[1].lines()), 1);
    }

    #[test]
    fn count_request_accepts_project_scoped_lb_list() {
        let req = ApiRequest::from(LoadBalancerLoadbalancerApiRequest::List(Box::new(
            LoadBalancerLoadbalancerList {
                project_id: Some("p".into()),
                ..Default::default()
            },
        )));
        assert_eq!(Home::count_request(&req), Some("loadbalancer"));
        let req = ApiRequest::from(LoadBalancerLoadbalancerApiRequest::List(Box::new(
            LoadBalancerLoadbalancerList {
                project_id: Some("p".into()),
                name: Some("x".into()),
                ..Default::default()
            },
        )));
        assert_eq!(Home::count_request(&req), None);
    }

    #[test]
    fn render_empty_shows_not_available() {
        let (mut home, _rx) = make_home();
        home.is_loading = false;
        let out = render_to_string(80, 24, &mut home);
        assert!(out.contains("Not available"), "empty state missing:\n{out}");
    }

    #[test]
    fn render_tiny_area_does_not_panic() {
        let (mut home, _rx) = make_home();
        home.set_compute_data(compute_json()).unwrap();
        home.set_network_data(network_json()).unwrap();
        home.set_block_storage_data(block_storage_json()).unwrap();
        home.set_lb_quota_data(lb_quota_json()).unwrap();
        home.set_dns_quota_data(dns_quota_json()).unwrap();
        home.is_loading = false;
        home.update(Action::HomeScrollEnd, Mode::Home).unwrap();
        let _out = render_to_string(20, 5, &mut home);
        let _out = render_to_string(30, 2, &mut home);
    }

    #[test]
    fn unlimited_limit_renders_as_infinity() {
        let (mut home, _rx) = make_home();
        home.set_compute_data(json!({
            "instances": {"in_use": 3, "limit": -1, "reserved": 0}
        }))
        .unwrap();
        home.is_loading = false;
        let out = render_to_string(100, 24, &mut home);
        assert!(out.contains("used 3/∞"), "unlimited gauge missing:\n{out}");
    }

    #[test]
    fn page_rows_fits_visible_lines() {
        let rows = vec![
            HomeRow::Header("A"),
            HomeRow::Gauges(vec![GaugeData::default(); 5]),
            HomeRow::Gauges(vec![GaugeData::default(); 5]),
            HomeRow::Gauges(vec![GaugeData::default(); 5]),
        ];
        // 1 + 4 = 5 lines fit into 5
        assert_eq!(Home::page_rows(&rows, 5), 2);
        // only the header fits into 1 line
        assert_eq!(Home::page_rows(&rows, 1), 1);
        // nothing fits, but at least one row per page
        assert_eq!(Home::page_rows(&rows, 0), 1);
        let empty: Vec<HomeRow> = Vec::new();
        assert_eq!(Home::page_rows(&empty, 10), 1);
    }

    #[test]
    fn connected_to_cloud_refreshes_only_in_home_mode() {
        use openstack_sdk::types::identity::v3::{Project, TokenInfo};

        let (mut home, mut rx) = make_home();
        let auth = TokenInfo {
            project: Some(Project {
                id: Some("proj-1".into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        // In another mode: no request
        home.update(
            Action::ConnectedToCloud(Box::new(auth.clone())),
            Mode::Describe,
        )
        .unwrap();
        assert_eq!(home.project_id.as_deref(), Some("proj-1"));
        assert!(rx.try_recv().is_err());

        // In home mode: requests are sent
        home.update(Action::ConnectedToCloud(Box::new(auth)), Mode::Home)
            .unwrap();
        assert_eq!(home.pending, 12);
    }
}
