#![no_std]

#[cfg(not(feature = "no_std"))]
pub mod io;
mod queued_runtime;
#[cfg(not(feature = "no_std"))]
mod timer;
#[cfg(not(feature = "no_std"))]
mod defer;

pub use queued_runtime::*;
#[cfg(not(feature = "no_std"))]
pub use timer::*;
#[cfg(not(feature = "no_std"))]
pub use defer::*;
