#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

/// Re-export the engine crate under the root crate.
pub mod engine {
    pub use catalina_engine::*;
}

/// Re-export the BSP crate under the root crate.
pub mod bsp {
    pub use catalina_bsp::*;
}

/// Re-export the BSP crate under the root crate.
pub mod instruments {
    pub use catalina_instruments::*;
}
