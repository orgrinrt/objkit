//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "alloc_box")]
extern crate alloc;

use include_proc_macro::macros;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::parse_quote;

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

#[inline]
fn box_path() -> TokenStream2 {
    #[cfg(all(not(feature = "std_box"), feature = "alloc_box"))]
    quote! {
        ::alloc::boxed::Box
    }
    #[cfg(feature = "std_box")]
    quote! {
        ::std::boxed::Box
    }
    #[cfg(all(feature = "std_box", feature = "alloc_box"))]
    compile_error!(
        "Both `std_box` and `alloc_box` features are enabled. Please enable only one of them."
    );
    #[cfg(all(not(feature = "std_box"), not(feature = "alloc_box")))]
    compile_error!(
        "Neither `std_box` nor `alloc_box` features are enabled. Please enable one of them."
    );
}

fn merged_generics(
    input_generics: &syn::Generics,
    internal_generics: &syn::Generics,
) -> syn::Generics {
    fn param_names_match(a: &syn::GenericParam, b: &syn::GenericParam) -> bool {
        match (a, b) {
            (syn::GenericParam::Type(a), syn::GenericParam::Type(b)) => a.ident == b.ident,
            (syn::GenericParam::Lifetime(a), syn::GenericParam::Lifetime(b)) => {
                a.lifetime.ident == b.lifetime.ident
            },
            (syn::GenericParam::Const(a), syn::GenericParam::Const(b)) => a.ident == b.ident,
            _ => false,
        }
    }

    let mut merged_generics = input_generics.clone();

    for param in &internal_generics.params {
        if !input_generics
            .params
            .iter()
            .any(|p| param_names_match(p, param))
        {
            merged_generics.params.push(param.clone());
        }
    }

    merged_generics
}

fn appended_generics(generics: &syn::Generics, new_param: &syn::Ident) -> syn::Generics {
    merged_generics(generics, &parse_quote!(<#new_param>))
}

macros! {
    attribute -> clone_box::clone_box,
    attribute -> as_any::as_any,
    attribute -> as_super::as_super,
    attribute -> obj_eq::obj_eq,
}
