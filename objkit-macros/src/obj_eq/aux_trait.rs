//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use crate::obj_eq::auxiliary_trait_name;
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

    if let Some(refusal) = crate::refuse_associated_types(&original_trait, "obj_eq") {
        return refusal;
    }

    let (decl_generics, ty_generics, where_clause) = trait_generics.split_for_impl();

    let internal_generic: syn::Ident = format_ident!("__U__");
    let merged = appended_generics(&trait_generics, &internal_generic);
    let (merged_impl_generics, _, _) = merged.split_for_impl();

    let aux_where = aux_where_clause(
        &trait_generics,
        parse_quote!(#internal_generic: #trait_name #ty_generics + 'static + PartialEq),
    );

    // The auxiliary trait, a supertrait of the annotated one, so both methods sit in
    // `dyn Trait`'s own vtable and neither side of a comparison has to be upcast first.
    // Upcasting `&dyn Trait` to `&dyn TraitObjEq` is what the language does since 1.86 and
    // the crate's minimum is below that, so an earlier shape carried a method for it and
    // paid a call per side; taking `&dyn Any` instead needs no upcast at all.
    //
    // `eq_any` hands out the value as `&dyn Any`, which is what lets `dyn_eq` recover the
    // concrete type on the other side with `downcast_ref`. It is not named `as_any`,
    // because `#[as_any]` is applied to the same trait below and puts a method of that
    // name on a sibling supertrait, and two supertraits offering one name makes the call
    // on `dyn Trait` ambiguous.
    let trait_impl = quote! {
        #vis trait #aux_trait_name #decl_generics #where_clause {
            fn dyn_eq(&self, other: &dyn ::core::any::Any) -> bool;
            #[doc(hidden)]
            fn eq_any(&self) -> &dyn ::core::any::Any;
        }
    };

    original_trait
        .supertraits
        .push(parse_quote!(#aux_trait_name #ty_generics));

    // Equality is one `downcast_ref`, which compares the two `TypeId`s and hands back the
    // concrete reference when they match. The check and the cast are one operation, so
    // there is no window in which the check can have passed for a type the cast then
    // gets wrong.
    //
    // It used to be two: a 64-bit hash of each side's `TypeId` compared first, then an
    // unchecked pointer cast on the strength of the hashes being equal. That paid two
    // hasher constructions and two hashes of a 128-bit id per comparison, and it made
    // a hash collision between two implementors undefined behaviour with nothing in
    // safe code to stop it. Comparing the ids themselves is both cheaper and total.
    let obj_eq_impl = quote! {
        impl #merged_impl_generics #aux_trait_name #ty_generics for #internal_generic
        #aux_where
        {
            #[inline]
            fn dyn_eq(&self, other: &dyn ::core::any::Any) -> bool {
                match other.downcast_ref::<#internal_generic>() {
                    ::core::option::Option::Some(other) => self == other,
                    ::core::option::Option::None => false,
                }
            }

            #[doc(hidden)]
            #[inline]
            fn eq_any(&self) -> &dyn ::core::any::Any {
                self
            }
        }
    };

    // The `Box` impls are a convenience over the reference ones and nothing else: comparing
    // two `&dyn Trait` is the operation, and comparing two boxed ones is the same call
    // through one more layer. Under `no_alloc` there is no `Box` to write an impl for, so
    // they are absent and the reference impls, which are the operation, stay.
    let boxed_impls_wanted = !cfg!(feature = "no_alloc");

    let box_partial_eq_impl = if boxed_impls_wanted {
        quote! {
        impl #decl_generics PartialEq for #box_path <dyn #trait_name #ty_generics> #where_clause {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                #aux_trait_name::dyn_eq(&**self, other.eq_any())
            }
        }
        }
    } else {
        quote! {}
    };

    let reference_partial_eq_impl = quote! {
        impl #decl_generics PartialEq for &dyn #trait_name #ty_generics #where_clause {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                #aux_trait_name::dyn_eq(*self, other.eq_any())
            }
        }
    };

    let box_eq_impl = if boxed_impls_wanted {
        quote! {
            impl #decl_generics Eq for #box_path <dyn #trait_name #ty_generics> #where_clause {}
        }
    } else {
        quote! {}
    };

    let reference_eq_impl = quote! {
        impl #decl_generics Eq for &dyn #trait_name #ty_generics #where_clause {}
    };

    quote! {
        #[::objkit::as_super]
        #[::objkit::as_any]
        #original_trait
        #trait_impl
        #obj_eq_impl
        #reference_partial_eq_impl
        #reference_eq_impl
        #box_partial_eq_impl
        #box_eq_impl
    }
}
