//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

pub mod aux_trait;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemTrait};

pub fn as_any(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_trait = parse_macro_input!(item as ItemTrait);
    let original_trait = input_trait.clone();

    // #[cfg(feature = "as_any_aux_trait")]
    let output = aux_trait::generate(original_trait);

    output.into()
}

pub(crate) const AS_ANY_TRAIT_NAME: &str = "AsAny";

pub(crate) fn auxiliary_trait_name(trait_name: Option<&syn::Ident>) -> syn::Ident {
    crate::auxiliary_trait_name(trait_name, AS_ANY_TRAIT_NAME)
}
