//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use crate::as_any::auxiliary_trait_name;
use proc_macro2::TokenStream;
use quote::quote;
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

    // Generate the auxiliary trait which provides the as_any method.
    let aux_trait = quote! {
        #vis trait #aux_trait_name {
            fn as_any(&self) -> &dyn ::std::any::Any;
        }
    };

    // Provide a blanket implementation for the auxiliary trait.
    let aux_impl = quote! {
        impl<T> #aux_trait_name for T
        where
            T: #trait_name #trait_generics + 'static,
        {
            #[inline]
            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }
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
