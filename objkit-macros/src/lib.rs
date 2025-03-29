//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

mod downcast;
mod obj_send_sync;
mod obj_visitor;
mod proxy;

fn auxiliary_trait_name(trait_name: Option<&syn::Ident>, postfix: &str) -> syn::Ident {
    let ident_string = match trait_name {
        Some(ident) => format!("{}{}", ident, postfix),
        None => postfix.to_string(),
    };
    syn::Ident::new(
        &ident_string,
        trait_name
            .map(|ident| ident.span())
            .unwrap_or_else(proc_macro2::Span::call_site),
    )
}

include_proc_macro::macros! {
    attribute -> clone_box::clone_box,
    attribute -> as_any::as_any,
    attribute -> as_super::as_super,
    attribute -> obj_eq::obj_eq,
}
