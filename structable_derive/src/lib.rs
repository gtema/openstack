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

#![doc = include_str!("../README.md")]
mod structable;

use darling::FromDeriveInput;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StructTable, attributes(structable))]
/// Derive macro to implementing `VecTable` traits
pub fn openstack_result_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let receiver = structable::TableStructInputReceiver::from_derive_input(&input).unwrap();
    let tokens = quote!(#receiver);
    tokens.into()
}
