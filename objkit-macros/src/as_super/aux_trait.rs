//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use crate::as_super::auxiliary_trait_name;
use crate::{appended_generics, aux_where_clause};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, ItemTrait};

pub(crate) fn generate(mut original_trait: ItemTrait) -> TokenStream {
    let trait_name = original_trait.ident.clone();
    let trait_generics = original_trait.generics.clone();
    let vis = original_trait.vis.clone();
    let aux_trait_name = auxiliary_trait_name(Some(&trait_name));

    if let Some(refusal) = crate::refuse_associated_types(&original_trait, "as_super") {
        return refusal;
    }

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

    let as_super_inlined_name = super_inlined_name(&trait_name);

    let aux_trait = quote! {
        #vis trait #aux_trait_name #decl_generics #where_clause {
            fn #as_super_inlined_name(&self) -> &dyn #trait_name #ty_generics;
        }
    };

    let aux_impl = quote! {
        impl #merged_impl_generics #aux_trait_name #ty_generics for #internal_generic
        #aux_where
        {
            #[inline]
            fn #as_super_inlined_name(&self) -> &dyn #trait_name #ty_generics {
                self as &dyn #trait_name #ty_generics
            }
        }
    };

    quote! {
        #original_trait
        #aux_trait
        #aux_impl
    }
}

pub(crate) fn super_inlined_name(trait_name: &syn::Ident) -> syn::Ident {
    format_ident!("as_{}", trait_name.to_string().to_lowercase())
}
