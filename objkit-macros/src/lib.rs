//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

mod downcast;
mod eq;
mod obj_send_sync;
mod obj_visitor;
mod proxy;

include_proc_macro::macros! {
    attribute -> clone_box:: clone_box,
}
