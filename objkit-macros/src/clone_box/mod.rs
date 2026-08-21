//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

#[path = "aux_trait.rs"]
mod aux_trait;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemTrait};

pub fn clone_box(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_trait = parse_macro_input!(item as ItemTrait);

    let output = aux_trait::generate(original_trait);

    output.into()
}

pub(crate) const CLONE_BOXED_TRAIT_NAME: &str = "CloneBoxed";

fn auxiliary_trait_name(trait_name: Option<&syn::Ident>) -> syn::Ident {
    crate::auxiliary_trait_name(trait_name, CLONE_BOXED_TRAIT_NAME)
}
