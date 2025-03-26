//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

#[path = "direct.rs"]
mod direct_ext;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemTrait};

pub fn clone_box(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_trait = parse_macro_input!(item as ItemTrait);
    let original_trait = input_trait.clone();

    let output = direct_ext::generate(original_trait);

    output.into()
}

pub(crate) const CLONE_BOXED_TRAIT_NAME: &str = "CloneBoxed";

fn auxiliary_trait_name(trait_name: Option<&syn::Ident>) -> syn::Ident {
    let ident_string = match trait_name {
        Some(ident) => format!("{}{}", ident, CLONE_BOXED_TRAIT_NAME),
        None => CLONE_BOXED_TRAIT_NAME.to_string(),
    };
    syn::Ident::new(
        &ident_string,
        trait_name
            .map(|ident| ident.span())
            .unwrap_or_else(proc_macro2::Span::call_site),
    )
}
