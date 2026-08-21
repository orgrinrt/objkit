//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use crate::{appended_generics, aux_where_clause};
use crate::as_any::auxiliary_trait_name;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, ItemTrait};

pub(crate) fn generate(mut original_trait: ItemTrait) -> TokenStream {
    let trait_name = original_trait.ident.clone();
    let trait_generics = original_trait.generics.clone();
    let vis = original_trait.vis.clone();
    let aux_trait_name = auxiliary_trait_name(Some(&trait_name));

    // A declaration position takes the parameters with their bounds; an argument position
    // takes the names alone. Rendering the whole `Generics` into both put `<A: Clone>`
    // where only `<A>` is legal, which rustc reports as `associated type bounds are
    // unstable` pointing at the caller's own trait.
    let (decl_generics, ty_generics, where_clause) = trait_generics.split_for_impl();

    let internal_generic: syn::Ident = format_ident!("__U__");
    let merged = appended_generics(&trait_generics, &internal_generic);
    let (merged_impl_generics, _, _) = merged.split_for_impl();

    let aux_where = aux_where_clause(
        &trait_generics,
        parse_quote!(#internal_generic: #trait_name #ty_generics + 'static),
    );

    original_trait
        .supertraits
        .push(parse_quote!(#aux_trait_name #ty_generics));

    // The auxiliary trait carrying `as_any`.
    let aux_trait = quote! {
        #vis trait #aux_trait_name #decl_generics #where_clause {
            fn as_any(&self) -> &dyn ::core::any::Any;
        }
    };

    let aux_impl = quote! {
        impl #merged_impl_generics #aux_trait_name #ty_generics for #internal_generic
        #aux_where
        {
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
        }
    };

    quote! {
        #original_trait
        #aux_trait
        #aux_impl
    }
}
