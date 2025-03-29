//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

// TODO: equality with something like this:

// ```
//  trait EqObject {
//     // Type-safe equality test for trait objects
//     fn eq_object(&self, other: &dyn EqObject) -> bool;
//
//     // Optional: Add fast path with type ID comparison first
//     fn type_hash(&self) -> u64;
// }
//
// impl<T: 'static + PartialEq> EqObject for T {
//     fn eq_object(&self, other: &dyn EqObject) -> bool {
//         // Fast path: Different type hashes mean definitely not equal
//         if self.type_hash() != other.type_hash() {
//             return false;
//         }
//
//         // Only downcast when types match
//         if let Some(typed) = other.downcast_ref::<T>() {
//             self == typed
//         } else {
//             false
//         }
//         // TODO: maybe think about variation where we compare pointers or some other
//         //       way to avoid downcasting, at least try a couple of different fast paths
//         //       before downcasting
//         //       e.g. if we know that the type is a pointer, we can compare the pointers
//         //       directly, or if we know that the type is a reference, we can compare the
//         //       references directly etc.
//         //       Also maybe consider using a custom type hash function instead of std::any::TypeId(?)
//         //       to avoid the overhead of creating a TypeId object
//         //       Also maybe we can make this branchless by using a union or something
//     }
//
//     fn type_hash(&self) -> u64 {
//         // Use std::any::TypeId hash or custom type hash
//         std::any::TypeId::of::<T>().into_u64()
//     }
// }
// ```
// NOTE: maybe we can use a union to store the pointer and the vtable in the same memory location

mod aux_trait;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemTrait};

pub fn obj_eq(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_trait = parse_macro_input!(item as ItemTrait);
    let original_trait = input_trait.clone();

    let output = aux_trait::generate(original_trait);

    output.into()
}

pub(crate) const OBJ_EQ_TRAIT_NAME: &str = "ObjEq";

fn auxiliary_trait_name(trait_name: Option<&syn::Ident>) -> syn::Ident {
    crate::auxiliary_trait_name(trait_name, OBJ_EQ_TRAIT_NAME)
}
