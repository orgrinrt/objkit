//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

// TODO: the idea I suppose is something like this:
//
// ```
// #[derive(Proxy)] // Our custom macro
// struct LoggingService<T: Service> {
//     inner: T,
//     logger: Logger,
//
//     #[before_call]
//     fn log_before(&self, method: &str, args: &[&dyn Debug]) {
//         self.logger.log(format!("Calling {method} with {args:?}"));
//     }
//
//     #[after_call]
//     fn log_after(&self, method: &str, result: &Result<(), Error>) {
//         self.logger.log(format!("Result of {method}: {result:?}"));
//     }
// }
// ```
// BUT this has to be applicable to trait objects (as well? or only?)
