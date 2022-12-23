#![no_std]

#[cfg(not(feature = "no_std"))]
pub mod io;
mod queued_runtime;
#[cfg(not(feature = "no_std"))]
mod timer;

pub use queued_runtime::*;
#[cfg(not(feature = "no_std"))]
pub use timer::*;
