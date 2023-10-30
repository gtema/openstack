//! Subnet find
use derive_builder::Builder;
use serde::de::DeserializeOwned;
use tracing::{debug, info, span, trace, Level};

use itertools::Itertools;

use crate::api::common::CommaSeparatedList;
use crate::api::find::Findable;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query};

use crate::api::network::v2::{
    subnet::get::Subnet as GetSubnet, subnets::get::Subnets as ListSubnets,
};

/// Find for subnet by NameOrId.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Subnet<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
}

impl<'a> Subnet<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> SubnetBuilder<'a> {
        SubnetBuilder::default()
    }
}

impl<'a> Findable for Subnet<'a> {
    type G = GetSubnet<'a>;
    type L = ListSubnets<'a>;
    fn get_ep(&self) -> GetSubnet<'a> {
        GetSubnet::builder().id(self.id.clone()).build().unwrap()
    }
    fn list_ep(&self) -> ListSubnets<'a> {
        ListSubnets::builder()
            .name(self.id.clone())
            .build()
            .unwrap()
    }
}
