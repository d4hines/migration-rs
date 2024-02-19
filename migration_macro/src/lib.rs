use std::collections::BTreeMap;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Migration_repr)]
pub fn migration_repr(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let ident_name = format_ident!("{}", input.ident);
    let fields_ident = format_ident!("fields"); // not sure if this is neede because of hygiene or
                                                // not
    let expanded = match input.data {
        syn::Data::Struct(struct_) => {
            let fields: Vec<_> = match struct_.fields {
                syn::Fields::Named(fields) => fields
                    .named
                    .into_iter()
                    .map(|field| {
                        let field_name = field.ident.unwrap().to_string();
                        println!("=========={},", field_name);
                        let ty = match field.ty {
                            syn::Type::Array(_) => todo!("implement arrays"),
                            syn::Type::BareFn(_) => todo!("implement barefn"),
                            syn::Type::Group(_) => todo!("implement group"),
                            syn::Type::ImplTrait(_) => todo!("implement impltraits"),
                            syn::Type::Infer(_) => todo!("implement infer"),
                            syn::Type::Macro(_) => todo!("implement macro"),
                            syn::Type::Never(_) => todo!("implement never"),
                            syn::Type::Paren(_) => todo!("implement Paren"),
                            syn::Type::Path(path) => quote! { #path::get_ty() },
                            syn::Type::Ptr(_) => todo!("implement ptr"),
                            syn::Type::Reference(_) => todo!("implement reference"),
                            syn::Type::Slice(_) => todo!("implement slice"),
                            syn::Type::TraitObject(_) => todo!("implement trait object"),
                            syn::Type::Tuple(_) => todo!("implement tuple"),
                            syn::Type::Verbatim(_) => todo!("implement verbatim"),
                            _ => panic!("unexpected ty '_'"),
                        };
                        quote! {
                            #fields_ident.insert(#field_name.to_string(), Box::new(#ty));
                        }
                    })
                    .collect(),
                syn::Fields::Unnamed(_) => todo!(),
                syn::Fields::Unit => todo!(),
            };
            quote! {
                impl migration_repr::Migratable for #ident_name {
                    fn get_ty() -> migration_repr::Ty {
                      let mut fields : std::collections::BTreeMap<String, Box<migration_repr::Ty>>
                            = std::collections::BTreeMap::new();
                      #( #fields )*
                      migration_repr::Ty::Struct(fields)
                    }
                }
            }
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
