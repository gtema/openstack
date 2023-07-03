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

        for field in fields.into_iter() {
            let field_ident = field.ident.as_ref().unwrap();
            let field_title = field.title.clone().unwrap_or(field_ident.to_string());

            // Build field rows for <T> impl
            let struct_row = match field.wide {
                false => match field.optional {
                    false => quote!(
                      if options.fields.is_empty() || options.fields.contains(#field_title) {
                          res.push(Vec::from([#field_title.to_string(), self. #field_ident .to_string()]));
                      }
                    ),
                    true => quote!(
                      if self. #field_ident .is_some() && (options.fields.is_empty() || options.fields.contains(#field_title)) {
                          res.push(Vec::from([#field_title.to_string(), self. #field_ident .clone().unwrap().to_string()]));
                      }
                    ),
                },

                true => match field.optional {
                    false => quote!(
                        if (options.fields.is_empty() || options.fields.contains(#field_title)) && options.wide  {
                            res.push(Vec::from([#field_title.to_string(), self. #field_ident .to_string()]));
                        }
                    ),
                    true => quote!(
                        if (options.fields.is_empty() || options.fields.contains(#field_title)) && options.wide  {
                            res.push(Vec::from([#field_title.to_string(), self. #field_ident .as_ref().map(|x| x.to_string()).unwrap_or("".to_string())]));
                        }
                    ),
                },
            };
            // Build field values processing for Vec<T> impl
            let vec_struct_row = match field.wide {
                false => match field.optional {
                    false => quote!(
                        if options.fields.is_empty() || options.fields.contains(#field_title) {
                            row.push(x. #field_ident .to_string());
                        }
                    ),
                    true => quote!(
                        if options.fields.is_empty() || options.fields.contains(#field_title) {
                            row.push(x. #field_ident .as_ref().map(|x| x.to_string()).unwrap_or("".to_string()));
                        }
                    ),
                },
                true => match field.optional {
                    false => quote!(
                        if (options.fields.is_empty() || options.fields.contains(#field_title)) && options.wide  {
                            row.push(x. #field_ident .to_string());
                        }
                    ),
                    true => quote!(
                        if (options.fields.is_empty() || options.fields.contains(#field_title)) && options.wide  {
                          row.push(x. #field_ident .as_ref().map(|v| v.to_string()).unwrap_or("".to_string()));
                        }
                    ),
                },
            };
            // Build field headers processing for the Vec<T> impl
            let vec_struct_header_row = match field.wide {
                false => quote!(
                    if options.fields.is_empty() || options.fields.contains(#field_title) {
                        headers.push(#field_title .to_string());
                    }
                ),
                true => quote!(
                    if (options.fields.is_empty() || options.fields.contains(#field_title)) && options.wide  {
                        headers.push(#field_title .to_string());
                    }
                ),
            };

            struct_fields.push(struct_row);
            vec_struct_fields.push(vec_struct_row);
            vec_struct_headers.push(vec_struct_header_row);
        }

        tokens.extend(quote! {
            impl #imp StructTable for #ident #ty #wher {
                fn build(&self, options: &OutputConfig) -> (::std::vec::Vec<::std::string::String>, ::std::vec::Vec<::std::vec::Vec<::std::string::String>>) {
                    let headers: Vec<String> = Vec::from(["Attribute".to_string(), "Value".to_string()]);
                    let mut res: Vec<Vec<String>> = Vec::new();
                    #(#struct_fields)*
                    (headers, res)
                }
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
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO Unit test `derive`

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
}
