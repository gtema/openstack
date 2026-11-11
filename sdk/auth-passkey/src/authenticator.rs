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
//! Native CTAP2 (USB HID) authenticator selection.

use webauthn_authenticator_rs::{
    AuthenticatorBackend,
    ctap2::select_one_device,
    transport::{AnyTransport, Transport},
    ui::Cli,
};

use crate::PasskeyError;

/// Run closure with the first available CTAP2 authenticator.
///
/// The authenticator API is blocking (PIN prompt, user presence), therefore
/// the closure is executed on a blocking thread of the current tokio runtime.
pub async fn with_authenticator<F, R>(f: F) -> Result<R, PasskeyError>
where
    F: FnOnce(&mut dyn AuthenticatorBackend) -> R + Send + 'static,
    R: Send + 'static,
{
    let handle = tokio::runtime::Handle::current();
    tokio::task::spawn_blocking(move || {
        handle.block_on(async {
            let ui = Cli {};
            let transport = AnyTransport::new().await?;
            let stream = transport.watch().await?;
            let mut auth = select_one_device(stream, &ui)
                .await
                .ok_or(PasskeyError::NoAuthenticator)?;
            Ok(f(&mut auth))
        })
    })
    .await?
}
