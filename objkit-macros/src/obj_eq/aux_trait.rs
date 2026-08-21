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

    let trait_impl = quote! {
        #vis trait #aux_trait_name #decl_generics #where_clause {
            fn dyn_eq(&self, other: &dyn #aux_trait_name #ty_generics) -> bool;
            #[doc(hidden)]
            fn type_hash(&self) -> u64;
            #[doc(hidden)]
            fn as_eq_object(&self) -> &dyn #aux_trait_name #ty_generics;
        }
    };

    original_trait
        .supertraits
        .push(parse_quote!(#aux_trait_name #ty_generics));

    #[cfg(feature = "std")]
    let type_hash_impl = quote! {
        #[doc(hidden)]
        #[inline]
        fn type_hash(&self) -> u64 {
            use ::std::hash::{Hash, Hasher};
            let mut hasher = ::std::collections::hash_map::DefaultHasher::new();
            ::std::any::TypeId::of::<#internal_generic>().hash(&mut hasher);
            hasher.finish()
        }
    };
    #[cfg(not(feature = "std"))]
    let type_hash_impl = quote! {
        #[doc(hidden)]
        #[inline]
        fn type_hash(&self) -> u64 {
            use ::core::hash::{Hash, Hasher};

            // `core::any::TypeId`, which has been in core since 1.0 and needs no crate at
            // all. This used to name `::typeable::TypeId`, which never resolved: that item
            // is a private re-import of `std::any::TypeId` inside a crate that is itself
            // `std`, so the whole point of reaching for it was defeated twice over. Nothing
            // caught it because no test ever compiled a `no_std` consumer that used
            // `obj_eq`, which is the only place the expansion lands.
            //
            // The hasher does need a crate, and it comes through objkit's own re-export
            // rather than being named here, for the same reason: `::siphasher` resolves in
            // the consumer only if the consumer happens to depend on it under that name.
            let mut hasher = ::objkit::__objkit_siphasher::sip::SipHasher24::new_with_keys(0, 0);
            ::core::any::TypeId::of::<#internal_generic>().hash(&mut hasher);
            hasher.finish()
        }
    };

    let obj_eq_impl = quote! {
        impl #merged_impl_generics #aux_trait_name #ty_generics for #internal_generic
        #aux_where
        {
            #[inline]
            fn dyn_eq(&self, other: &dyn #aux_trait_name #ty_generics) -> bool {
                if self.type_hash() != other.type_hash() {
                    return false;
                }
                // SAFETY: the type_hash check above establishes that `other` is a
                // `#internal_generic`, which is what makes this cast the identity.
                let other_t = unsafe {
                    &*(other.as_eq_object() as *const _ as *const #internal_generic)
                };
                self == other_t
            }

            #type_hash_impl

            #[doc(hidden)]
            #[inline]
            fn as_eq_object(&self) -> &dyn #aux_trait_name #ty_generics {
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
                #aux_trait_name::dyn_eq(self.as_eq_object(), other.as_eq_object())
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
                #aux_trait_name::dyn_eq(
                    self.as_eq_object() as &dyn #aux_trait_name #ty_generics,
                    other.as_eq_object() as &dyn #aux_trait_name #ty_generics,
                )
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
