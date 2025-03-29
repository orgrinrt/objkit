//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

// TODO: something like this:
//
// ```
// Marker traits with explicit bounds
// trait SendObject: Send {
//     // Methods that can be called across thread boundaries
//     fn process(&self);
// }
//
// trait SyncObject: Send + Sync {
//     // Methods that can be shared between threads
//     fn process_shared(&self);
// }
//
// // Generic wrapper that enforces thread safety
// struct ThreadSafe<T: ?Sized + Send + Sync>(T);
//
// ```
//
// NOTE: we probably need to have an attribute for the fns in addition to the one that gens the above?
//       e.g. #[obj_send_sync] or something like that
//       or #[obj_send_sync(unsafe)]
//       or #[obj_send_sync(unsafe, thread_safe)]
//       or maybe we don't? not sure what the best way to do this is
