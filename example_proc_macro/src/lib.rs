use migration_repr::Migration;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use syn::{parse_macro_input, DeriveInput};
use migration_repr::Migratable;

#[proc_macro]
pub fn make_migrated_types(_input: TokenStream) -> TokenStream {
    let expanded = Migration::new(example_legacy_types::LegacyBar::get_ty())
        .to_token_stream();
    TokenStream::from(expanded)
}
