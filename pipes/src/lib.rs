// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A collection of pipes used in 'signalo' umbrella crate.

#![cfg_attr(feature = "missing_mpl", feature(plugin))]
#![cfg_attr(feature = "missing_mpl", plugin(missing_mpl))]
#![cfg_attr(feature = "missing_mpl", deny(missing_mpl))]

#![warn(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate core as std;

extern crate signalo_traits;
extern crate signalo_filters;

pub mod source;
pub mod filter;
pub mod sink;

/// The crate's prelude.
pub mod prelude {
    pub use {
        source::{
            // macros::*,
            UnitPipe as SourceUnitPipe,
            Pipe as SourcePipe
        },
        filter::{
            macros::*,
            UnitPipe as FilterUnitPipe,
            Pipe as FilterPipe
        },
        sink::{
            // macros::*,
            UnitPipe as SinkUnitPipe,
            Pipe as SinkPipe
        },
    };
}
