use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// Support parsing from a full derive input. Unlike FromMeta, this isn't
/// composable; each darling-dependent crate should have its own struct to handle
/// when its trait is derived.
#[derive(Debug, FromDeriveInput)]
// This line says that we want to process all attributes declared with `my_trait`,
// and that darling should panic if this receiver is given an enum.
#[darling(attributes(structable), supports(struct_any))]
pub(crate) struct TableStructInputReceiver {
    /// The struct ident.
    ident: syn::Ident,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<(), TableStructFieldReceiver>,
}

#[derive(Debug, FromField)]
#[darling(attributes(structable))]
pub(crate) struct TableStructFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<syn::Ident>,

    // /// This magic field name pulls the type from the input.
    // ty: syn::Type,
    title: Option<String>,

    /// Whether option is returned in wide mode only
    #[darling(default)]
    wide: bool,

    /// Whether option is returned is optional or not
    #[darling(default)]
    optional: bool,

    /// apply `to_string_pretty` instead of `to_string` for the value
    #[darling(default)]
    pretty: bool,

    /// Serialize field as json for output
    #[darling(default)]
    serialize: bool,

    /// `status` field
    #[darling(default)]
    status: bool,
}

impl ToTokens for TableStructInputReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TableStructInputReceiver {
            ref ident,
            ref generics,
            ref data,
        } = *self;

        let (imp, ty, wher) = generics.split_for_impl();
        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        let mut struct_fields = Vec::new();
        let mut vec_struct_headers = Vec::new();
        let mut vec_struct_fields = Vec::new();
        let mut status_field: Option<&TableStructFieldReceiver> = None;
        let mut status_alt_field: Option<&TableStructFieldReceiver> = None;

        for field in fields.iter() {
            let field_ident = field.ident.as_ref().unwrap();
            let field_title = field.title.clone().unwrap_or(field_ident.to_string());

            // Determine how to get the data based in `optional` and `pretty`
            let field_value = match field.optional {
                false => match field.serialize || field.pretty {
                    false => quote!(
                        self. #field_ident .to_string()
                    ),
                    true => quote!(
                        if options.pretty {
                            serde_json::to_string_pretty(&self. #field_ident)
                        } else {
                            serde_json::to_string(&self. #field_ident)
                        }.unwrap_or_else(|_| String::from("<ERROR SERIALIZING DATA>"))
                    ),
                },
                true => match field.serialize || field.pretty {
                    false => quote!(
                        self. #field_ident .clone().map_or(String::from(" "), |v| v.to_string())
                    ),
                    true => quote!(
                        self. #field_ident
                            .clone()
                            .map_or(
                                String::from(" "),
                                |v| if options.pretty {
                                    serde_json::to_string_pretty(&v)
                                } else {
                                    serde_json::to_string(&v)
                                }.unwrap_or_else(|_| String::from("<ERROR SERIALIZING DATA>"))
                            )
                    ),
                },
            };

            // Determine how to get the data based in `optional` and `pretty` for list row column
            //let field_vec_value = field_value.clone();
            let field_vec_value = match field.optional {
                false => match field.serialize || field.pretty {
                    false => quote!(
                        x. #field_ident .to_string()
                    ),
                    true => quote!(
                        if options.pretty {
                            serde_json::to_string_pretty(&x. #field_ident)
                        } else {
                            serde_json::to_string(&x. #field_ident)
                        }.unwrap_or_else(|_| String::from("<ERROR SERIALIZING DATA>"))
                    ),
                },
                true => match field.serialize || field.pretty {
                    false => quote!(
                        x. #field_ident .clone().map_or(String::from(" "), |v| v.to_string())
                    ),
                    true => quote!(
                        x. #field_ident
                            .clone()
                            .map_or(
                                String::from(" "),
                                |v| if options.pretty {
                                    serde_json::to_string_pretty(&v)
                                } else {
                                    serde_json::to_string(&v)
                                }.unwrap_or_else(|_| String::from("<ERROR SERIALIZING DATA>"))
                            )
                    ),
                },
            };

            // Build field rows for <T> impl
            let struct_row = match field.wide {
                false => match field.optional {
                    false => quote!(
                        if options.fields.is_empty() || options.fields.contains(#field_title) {
                            res.push(Vec::from([
                                #field_title .to_string(),
                                #field_value
                            ]));
                        }
                    ),
                    true => quote!(
                        if self. #field_ident .is_some() && (options.fields.is_empty() || options.fields.contains(#field_title)) {
                            res.push(Vec::from([
                                #field_title.to_string(),
                                #field_value
                            ]));
                        }
                    ),
                },

                true => quote!(
                    if options.fields.contains(#field_title) || (options.fields.is_empty() && options.wide) {
                        res.push(Vec::from([
                            #field_title.to_string(),
                            #field_value
                        ]));
                    }
                ),
            };
            // Build field values processing for Vec<T> impl
            let vec_struct_row = match field.wide {
                false => quote!(
                    if options.fields.is_empty() || options.fields.contains(#field_title) {
                        row.push(#field_vec_value);
                    }
                ),
                true => quote!(
                    if options.fields.contains(#field_title) || (options.fields.is_empty() && options.wide)  {
                        row.push(#field_vec_value);
                    }
                ),
            };
            // Build field headers processing for the Vec<T> impl
            let vec_struct_header_row = match field.wide {
                false => quote!(
                    if options.fields.is_empty() || options.fields.contains(#field_title) {
                        headers.push(#field_title .to_string());
                    }
                ),
                true => quote!(
                    if options.fields.contains(#field_title) || (options.fields.is_empty() && options.wide)  {
                        headers.push(#field_title .to_string());
                    }
                ),
            };

            struct_fields.push(struct_row);
            vec_struct_fields.push(vec_struct_row);
            vec_struct_headers.push(vec_struct_header_row);

            // Save the status or status_alt (the one with name `status`) field
            if field.status {
                status_field = Some(field);
            }
            if field_title.to_lowercase() == "status" {
                status_alt_field = Some(field);
            }
        }

        // Set status_field to status_alt if no explicit `status` set
        if status_alt_field.is_some() && status_field.is_none() {
            status_field = status_alt_field;
        }
        // Construct code for the `status` trait method for single struct and vec
        let (item_status, struct_status) = match status_field {
            Some(field) => {
                let field_ident = field.ident.as_ref().unwrap();

                match (field.optional, field.serialize) {
                    (true, false) => (
                        quote!(
                            fn status(&self) -> ::std::vec::Vec<Option<::std::string::String>> {
                                    self.iter().map(|item| item. #field_ident .clone().map(|val| val.to_string())).collect()
                        }),
                        quote!(
                            fn status(&self) -> ::std::vec::Vec<Option<::std::string::String>> {
                                    Vec::from([self. #field_ident .clone().map(|val| val.to_string())])
                        }),
                    ),
                    (false, false) => (
                        quote!(
                            fn status(&self) -> ::std::vec::Vec<Option<::std::string::String>> {
                                    self.iter().map(|item| Some(item. #field_ident .to_string())).collect()
                        }),
                        quote!(
                            fn status(&self) -> ::std::vec::Vec<Option<::std::string::String>> {
                                    Vec::from([Some(self. #field_ident .to_string())])
                        }),
                    ),
                    (true, true) => (
                        quote!(
                            fn status(&self) -> ::std::vec::Vec<Option<::std::string::String>> {
                                    self.iter().map(|item| item. #field_ident .clone().map(|val| serde_json::to_string(&val).map(|x| x.trim_matches('"').to_string()).unwrap_or_else(|_| String::from("<ERROR SERIALIZING>")))).collect()
                        }),
                        quote!(
                            fn status(&self) -> ::std::vec::Vec<Option<::std::string::String>> {
                                    Vec::from([self. #field_ident .clone().map(|val| serde_json::to_string(&val).map(|x| x.trim_matches('"').to_string()).unwrap_or_else(|_| String::from("<ERROR SERIALIZING>")))])
                        }),
                    ),
                    (false, true) => (
                        quote!(
                            fn status(&self) -> ::std::vec::Vec<Option<::std::string::String>> {
                                    self.iter().map(|item| Some(serde_json::to_string(&item. #field_ident).map(|x| x.trim_matches('"').to_string()).unwrap_or_else(|_| String::from("<ERROR SERIALIZING>")))).collect()
                        }),
                        quote!(
                            fn status(&self) -> ::std::vec::Vec<Option<::std::string::String>> {
                                    Vec::from([Some(serde_json::to_string(&self. #field_ident).map(|x| x.trim_matches('"').to_string()).unwrap_or_else(|_| String::from("<ERROR SERIALIZING>")))])
                        }),
                    ),
                }
            }
            _ => (
                quote!(
                    fn status(&self) -> ::std::vec::Vec<Option<::std::string::String>> {
                        return std::iter::repeat_with(|| None).take(self.len()).collect();
                    }
                ),
                quote!(
                    fn status(&self) -> ::std::vec::Vec<Option<::std::string::String>> {
                        Vec::from([None])
                    }
                ),
            ),
        };

        tokens.extend(quote! {
            impl #imp StructTable for #ident #ty #wher {
                fn build(&self, options: &OutputConfig) -> (::std::vec::Vec<::std::string::String>, ::std::vec::Vec<::std::vec::Vec<::std::string::String>>) {
                    let headers: Vec<String> = Vec::from(["Attribute".to_string(), "Value".to_string()]);
                    let mut res: Vec<Vec<String>> = Vec::new();
                    #(#struct_fields)*
                    (headers, res)
                }

                #struct_status
            }

            impl #imp StructTable for Vec<#ident> #ty #wher {
                fn build(&self, options: &OutputConfig) -> (::std::vec::Vec<::std::string::String>, ::std::vec::Vec<::std::vec::Vec<::std::string::String>>) {
                    let mut res: Vec<Vec<String>> = Vec::new();
                    let mut headers: Vec<String> = Vec::new();

                    #(#vec_struct_headers)*

                    for x in self.iter() {
                        let mut row: Vec<String> = Vec::new();
                        #(#vec_struct_fields)*
                        res.push(row);
                    }
                    (headers, res)
                }

                #item_status
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_default() {
        let input = quote! {
            #[derive(StructTable)]
            struct FooSpec {
                #[structable(title="foo")]
                foo: String,
                #[structable(wide)]
                bar: String,
            }
        };
        let input = syn::parse2(input).unwrap();
        TableStructInputReceiver::from_derive_input(&input).unwrap();
    }

    #[test]
    fn test_parse_pretty() {
        let input = quote! {
            #[derive(StructTable)]
            struct FooSpec {
                #[structable(pretty)]
                foo: Value,
                #[structable(optional, pretty)]
                bar: Option<Value>,
            }
        };
        let input = syn::parse2(input).unwrap();
        TableStructInputReceiver::from_derive_input(&input).unwrap();
    }
}
