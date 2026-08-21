//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
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
use crate::{appended_generics, aux_where_clause, box_path};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, ItemTrait};

pub(crate) fn generate(mut original_trait: ItemTrait) -> TokenStream {
    let trait_name = original_trait.ident.clone();
    let trait_generics = original_trait.generics.clone();
    let vis = original_trait.vis.clone();
    let aux_trait_name = auxiliary_trait_name(Some(&trait_name));
    let box_path = box_path();

    if let Some(refusal) = crate::refuse_associated_types(&original_trait, "clone_box") {
        return refusal;
    }

    let (decl_generics, ty_generics, where_clause) = trait_generics.split_for_impl();

    let internal_generic: syn::Ident = format_ident!("__U__");
    let merged = appended_generics(&trait_generics, &internal_generic);
    let (merged_impl_generics, _, _) = merged.split_for_impl();

    let aux_where = aux_where_clause(
        &trait_generics,
        parse_quote!(#internal_generic: #trait_name #ty_generics + Clone + 'static),
    );

    original_trait
        .supertraits
        .push(parse_quote!(#aux_trait_name #ty_generics));

    let aux_trait = quote! {
        #vis trait #aux_trait_name #decl_generics #where_clause {
            fn clone_box(&self) -> #box_path <dyn #trait_name #ty_generics>;
        }
    };

    let aux_impl = quote! {
        impl #merged_impl_generics #aux_trait_name #ty_generics for #internal_generic
        #aux_where
        {
            #[inline]
            fn clone_box(&self) -> #box_path <dyn #trait_name #ty_generics> {
                #box_path::new(self.clone())
            }
        }
    };

    // `Clone` for the boxed trait object, dispatching through the auxiliary trait. This impl
    // used to carry no generics at all, so an annotated `trait T<A>` produced
    // `impl Clone for Box<dyn T<A>>` with `A` undeclared, and the whole expansion failed
    // with `cannot find type A in this scope`. The other three macros accepted the same
    // trait, so the divergence was in this impl rather than in the shape.
    let box_clone_impl = quote! {
        impl #decl_generics Clone for #box_path <dyn #trait_name #ty_generics> #where_clause {
            #[inline]
            fn clone(&self) -> Self {
                <dyn #trait_name #ty_generics as #aux_trait_name #ty_generics>::clone_box(&**self)
            }
        }
    };

    quote! {
        #original_trait
        #aux_trait
        #aux_impl
        #box_clone_impl
    }
}
