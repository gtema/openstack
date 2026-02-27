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

//! Prunes objects from the container.
#[cfg(feature = "async")]
use async_trait::async_trait;
use futures::stream::{StreamExt, TryStreamExt};
use serde::Deserialize;
use tracing::debug;

use crate::{OpenStackError, RestError};

#[cfg(feature = "async")]
use crate::api::{AsyncClient, QueryAsync};
// #[cfg(feature = "sync")]
// use crate::api::Client;

use crate::api::{
    Pagination, ignore,
    object_store::v1::{
        container::get::Request as ListRequest, object::delete::Request as DeleteRequest,
    },
    paged,
};

/// Delete concurrency
const DELETE_CONCURRENCY: usize = 4;

#[derive(Deserialize, Debug, Clone)]
pub struct Object {
    name: String,
}

#[cfg(feature = "async")]
#[async_trait]
pub trait PruneAsyncExt {
    async fn object_store_container_prune_async<
        S1: AsRef<str> + Send + Sync,
        S2: AsRef<str> + Send + Sync,
        S3: AsRef<str> + Send + Sync,
    >(
        &self,
        account: S1,
        container: S2,
        prefix: Option<S3>,
    ) -> Result<(), OpenStackError>;
}

#[cfg(feature = "async")]
#[async_trait]
impl<C> PruneAsyncExt for C
where
    C: AsyncClient<Error = RestError> + Sync,
{
    async fn object_store_container_prune_async<
        S1: AsRef<str> + Send + Sync,
        S2: AsRef<str> + Send + Sync,
        S3: AsRef<str> + Send + Sync,
    >(
        &self,
        account: S1,
        container: S2,
        prefix: Option<S3>,
    ) -> Result<(), OpenStackError> {
        let mut list_builder = ListRequest::builder();
        // Set path parameters
        list_builder.account(account.as_ref());
        list_builder.container(container.as_ref());
        // Set query filter parameters
        if let Some(pref) = prefix {
            list_builder.prefix(pref.as_ref().to_owned());
        }
        let list_ep = list_builder
            .build()
            .map_err(|x| OpenStackError::EndpointBuild(x.to_string()))?;

        paged(list_ep, Pagination::All)
            .iter_async::<C, Object>(self)
            .map(Ok)
            .try_for_each_concurrent(DELETE_CONCURRENCY, |item| async {
                if let Ok(object) = item {
                    let object_name = object.name.clone();
                    debug!("Deleting object {:?}", object_name);
                    let mut delete_builder = DeleteRequest::builder();
                    delete_builder.account(account.as_ref());
                    delete_builder.container(container.as_ref());
                    delete_builder.object(object.name);
                    let delete_ep = delete_builder
                        .build()
                        .map_err(|x| OpenStackError::EndpointBuild(x.to_string()))?;
                    ignore(delete_ep).query_async(self).await?;
                }
                Ok::<(), OpenStackError>(())
            })
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_prune() {}
}
