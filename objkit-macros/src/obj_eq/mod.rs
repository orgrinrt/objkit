//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

// Equality over trait objects is a `PartialEq` on `&dyn Trait` and on `Box<dyn Trait>`,
// resolved through an auxiliary trait every implementor gets for free. The comparison
// recovers the concrete type on the other side with `Any::downcast_ref`, which is the
// `TypeId` check and the cast as one operation, and compares with the implementor's own
// `PartialEq` when it is the same type. Different types are never equal.

mod aux_trait;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemTrait};

pub fn obj_eq(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let original_trait = parse_macro_input!(item as ItemTrait);

    let output = aux_trait::generate(original_trait);

    output.into()
}

pub(crate) const OBJ_EQ_TRAIT_NAME: &str = "ObjEq";

fn auxiliary_trait_name(trait_name: Option<&syn::Ident>) -> syn::Ident {
    crate::auxiliary_trait_name(trait_name, OBJ_EQ_TRAIT_NAME)
}
