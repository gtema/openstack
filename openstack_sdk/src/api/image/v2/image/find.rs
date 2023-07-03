//! Image find
use derive_builder::Builder;
use serde::de::DeserializeOwned;
use tracing::{debug, info, span, trace, Level};

use itertools::Itertools;

use crate::api::common::CommaSeparatedList;
use crate::api::find::Findable;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query};

use crate::api::image::v2::{image::get::Image as GetImage, images::get::Images as ListImages};

/// Find for image by NameOrId.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Image<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
}

impl<'a> Image<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ImageBuilder<'a> {
        ImageBuilder::default()
    }
}

impl<'a> Findable for Image<'a> {
    type G = GetImage<'a>;
    type L = ListImages<'a>;
    fn get_ep(&self) -> GetImage<'a> {
        GetImage::builder().id(self.id.clone()).build().unwrap()
    }
    fn list_ep(&self) -> ListImages<'a> {
        ListImages::builder().name(self.id.clone()).build().unwrap()
    }
}
