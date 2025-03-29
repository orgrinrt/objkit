//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use crate::obj_eq::auxiliary_trait_name;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemTrait};

pub(crate) fn generate(mut original_trait: ItemTrait) -> TokenStream {
    let trait_name = &original_trait.ident;
    let vis = &original_trait.vis;
    let aux_trait_name = auxiliary_trait_name(Some(trait_name));

    let trait_impl = quote! {
        #vis trait #aux_trait_name {
            fn dyn_eq(&self, other: &dyn #aux_trait_name) -> bool;
            // fn dyn_eq_(&self, other: &dyn #trait_name) -> bool;
            #[doc(hidden)]
            fn type_hash(&self) -> u64;
            #[doc(hidden)]
            fn as_eq_object(&self) -> &dyn #aux_trait_name;
        }
    };

    original_trait
        .supertraits
        .push(parse_quote!(#aux_trait_name));

    let obj_eq_impl = quote! {
        impl<T> #aux_trait_name for T
        where
            T: #trait_name + 'static + PartialEq,
        {
            #[inline]
            fn dyn_eq(&self, other: &dyn #aux_trait_name) -> bool {
                if self.type_hash() != other.type_hash() {
                    return false;
                }
                // SAFETY: the type_hash check guarantees that the underlying type is T here
                let other_t = unsafe { &*(other.as_eq_object() as *const _ as *const T) };
                self == other_t
            }

            // #[inline]
            // fn dyn_eq_(&self, other: &dyn #trait_name) -> bool
            // {
            //     (self.as_animal() as &dyn Animal) == (other as &dyn Animal)
            // }

            #[doc(hidden)]
            #[inline]
            fn type_hash(&self) -> u64 {
                use std::hash::{Hash, Hasher};
                let mut hasher = ::std::collections::hash_map::DefaultHasher::new();
                ::std::any::TypeId::of::<T>().hash(&mut hasher);
                hasher.finish()
            }

            #[doc(hidden)]
            #[inline]
            fn as_eq_object(&self) -> &dyn #aux_trait_name {
                self
            }
        }
    };

    let box_partial_eq_impl = quote! {
        impl PartialEq for Box<dyn #trait_name> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                #aux_trait_name::dyn_eq(self.as_eq_object(), other.as_eq_object())
            }
        }
        impl PartialEq for &dyn #trait_name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                #aux_trait_name::dyn_eq(self.as_eq_object() as &dyn #aux_trait_name, other.as_eq_object() as &dyn #aux_trait_name)
            }
        }
    };

    let box_eq_impl = quote! {
        impl Eq for Box<dyn #trait_name> {}
        impl Eq for &dyn #trait_name {}
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
