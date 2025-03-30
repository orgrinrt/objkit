//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use crate::obj_eq::auxiliary_trait_name;
use crate::{appended_generics, box_path, merged_generics};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::Parser;
use syn::{parse2, parse_quote, Generics, ItemTrait};

pub(crate) fn generate(mut original_trait: ItemTrait) -> TokenStream {
    let trait_name = &original_trait.ident;
    let trait_generics = &original_trait.generics;
    let vis = &original_trait.vis;
    let aux_trait_name = auxiliary_trait_name(Some(trait_name));
    let box_path = box_path();

    let trait_impl = quote! {
        #vis trait #aux_trait_name #trait_generics {
            fn dyn_eq(&self, other: &dyn #aux_trait_name #trait_generics) -> bool;
            // fn dyn_eq_(&self, other: &dyn #trait_name) -> bool;
            #[doc(hidden)]
            fn type_hash(&self) -> u64;
            #[doc(hidden)]
            fn as_eq_object(&self) -> &dyn #aux_trait_name #trait_generics;
        }
    };

    original_trait
        .supertraits
        .push(parse_quote!(#aux_trait_name #trait_generics));

    let internal_generic: syn::Ident = format_ident!("__U__");
    let merged_generics = appended_generics(trait_generics, &internal_generic);

    #[cfg(feature = "std")]
    let type_hash_impl = quote! {
        #[doc(hidden)]
        #[inline]
        fn type_hash(&self) -> u64 {
            use std::hash::{Hash, Hasher};
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
            use core::hash::{Hash, Hasher};
            use siphasher::sip::{SipHasher, SipHasher24};

            let mut hasher = SipHasher24::new_with_keys(0, 0);
            typeable::TypeId::of::<#internal_generic>().hash(&mut hasher);
            hasher.finish()
        }
    };

    let obj_eq_impl = quote! {
        impl #merged_generics #aux_trait_name #trait_generics for #internal_generic
        where
            #internal_generic: #trait_name #trait_generics + 'static + PartialEq,
        {
            #[inline]
            fn dyn_eq(&self, other: &dyn #aux_trait_name #trait_generics) -> bool {
                if self.type_hash() != other.type_hash() {
                    return false;
                }
                // SAFETY: the type_hash check guarantees that the underlying type is T here
                let other_t = unsafe { &*(other.as_eq_object() as *const _ as *const #internal_generic) };
                self == other_t
            }

            // #[inline]
            // fn dyn_eq_(&self, other: &dyn #trait_name) -> bool
            // {
            //     (self.as_animal() as &dyn Animal) == (other as &dyn Animal)
            // }

            #type_hash_impl

            #[doc(hidden)]
            #[inline]
            fn as_eq_object(&self) -> &dyn #aux_trait_name #trait_generics {
                self
            }
        }
    };

    let box_partial_eq_impl = quote! {
        impl #trait_generics PartialEq for #box_path <dyn #trait_name  #trait_generics> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                #aux_trait_name::dyn_eq(self.as_eq_object(), other.as_eq_object())
            }
        }
        impl #trait_generics PartialEq for &dyn #trait_name  #trait_generics {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                #aux_trait_name::dyn_eq(self.as_eq_object() as &dyn #aux_trait_name  #trait_generics, other.as_eq_object() as &dyn #aux_trait_name  #trait_generics)
            }
        }
    };

    let box_eq_impl = quote! {
        impl  #trait_generics Eq for #box_path <dyn #trait_name  #trait_generics> {}
        impl  #trait_generics Eq for &dyn #trait_name  #trait_generics {}
    };

    let expanded = quote! {
        #[::objkit::as_super]
        #[::objkit::as_any]
        #original_trait
        #trait_impl
        #obj_eq_impl
        #box_partial_eq_impl
        #box_eq_impl
    };

    expanded
}
