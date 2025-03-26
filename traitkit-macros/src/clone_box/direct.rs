//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------
//! This module implements the clone_box attribute macro.
//!
//! When a trait is annotated with `#[clone_box]`, the macro expands it to generate a unique
//! auxiliary trait (by appending `CloneBox` to the trait name). This auxiliary trait provides a
//! `clone_box` method to enable cloning of trait objects.
//!
//! This approach achieves:
//!
//! - \*Performance\*: By generating a unique auxiliary trait for every annotated trait, the direct
//!   variant allows the compiler to optimize more effectively through inlining, resulting in a near
//!   zero-cost abstraction aside from the unavoidable dynamic dispatch call.
//!
//! - *Zero-cost abstraction\*: Although dynamic dispatch is used (and is inherent to the pattern
//!   anyway, so that comes with the domain even if manually writing), the additional overhead is minimal,
//!   and in optimized builds the compiler can often remove any unnecessary indirection. This design
//!   maximizes performance while retaining flexibility.
//!
//! - \*Object safety\*: The macro modifies the original trait to extend the auxiliary trait ensuring that
//!   trait objects (for example, `Box<dyn Animal>`) implement the auxiliary trait. This guarantees the
//!   correct resolution of the `clone_box` method when calling `<dyn Trait as AuxiliaryTrait>::clone_box`.
//!
//! For more details, see the inline code comments and the repository documentation.
//!

use crate::clone_box::auxiliary_trait_name;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemTrait};

pub(crate) fn generate(mut original_trait: ItemTrait) -> TokenStream {
    let trait_name = &original_trait.ident;
    let trait_generics = &original_trait.generics;
    let vis = &original_trait.vis;
    let aux_trait_name = auxiliary_trait_name(Some(trait_name));

    // modify the trait to extend the aux trait
    original_trait
        .supertraits
        .push(parse_quote!(#aux_trait_name));

    // generate the auxiliary trait (unique for this trait)
    let aux_trait = quote! {
        #vis trait #aux_trait_name {
            fn clone_box(&self) -> Box<dyn #trait_name #trait_generics>;
        }
    };

    // provide a blanket impl for the auxiliary trait
    let aux_impl = quote! {
        impl<T> #aux_trait_name for T
        where
            T: #trait_name #trait_generics + Clone + 'static,
        {
            #[inline]
            fn clone_box(&self) -> Box<dyn #trait_name #trait_generics> {
                Box::new(self.clone())
            }
        }
    };

    // implement Clone for Box<dyn Trait> by dispatching via the aux trait
    let box_clone_impl = quote! {
        impl Clone for Box<dyn #trait_name #trait_generics> {
            #[inline]
            fn clone(&self) -> Self {
                <dyn #trait_name #trait_generics as #aux_trait_name>::clone_box(&**self)
            }
        }
    };

    let expanded = quote! {
        #original_trait
        #aux_trait
        #aux_impl
        #box_clone_impl
    };

    expanded
}
