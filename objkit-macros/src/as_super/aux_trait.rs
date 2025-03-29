//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use crate::as_super::auxiliary_trait_name;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, ItemTrait};

pub(crate) fn generate(mut original_trait: ItemTrait) -> TokenStream {
    let trait_name = &original_trait.ident;
    let trait_generics = &original_trait.generics;
    let vis = &original_trait.vis;
    let aux_trait_name = auxiliary_trait_name(Some(trait_name));

    // Modify the original trait to extend the auxiliary trait.
    original_trait
        .supertraits
        .push(parse_quote!(#aux_trait_name));

    let as_super_inlined_name = super_inlined_name(trait_name);
    // let as_super_box_inlined_name = super_inlined_name_box(trait_name);

    let aux_trait = quote! {
        #vis trait #aux_trait_name {
            fn #as_super_inlined_name(&self) -> &dyn #trait_name;
            // fn #as_super_box_inlined_name(&self) -> Box<dyn #trait_name>;
        }
    };

    // Provide a blanket implementation for the auxiliary trait.
    let aux_impl = quote! {
        impl<T> #aux_trait_name for T
        where
            T: #trait_name #trait_generics + 'static,
        {
            #[inline]
            fn #as_super_inlined_name(&self) -> &dyn #trait_name #trait_generics {
                self as &dyn #trait_name #trait_generics
            }
            // #[inline]
            // fn #as_super_box_inlined_name(&self) -> Box<dyn #trait_name #trait_generics> {
            //     Box::new(self)
            // }
        }
    };

    // Combine the modified original trait, auxiliary trait, and the blanket impl.
    let expanded = quote! {
        #original_trait
        #aux_trait
        #aux_impl
    };

    expanded
}

pub(crate) fn super_inlined_name(trait_name: &syn::Ident) -> syn::Ident {
    format_ident!("as_{}", trait_name.to_string().to_lowercase())
}

// pub(crate) fn super_inlined_name_box(trait_name: &syn::Ident) -> syn::Ident {
//     format_ident!("as_{}_box", trait_name.to_string().to_lowercase())
// }
