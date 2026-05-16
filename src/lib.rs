// Copyright (c) 2026 Illia Diadenchuk
// SPDX-License-Identifier: Zlib

mod string_builder_safe;
mod string_builder_unsafe;

pub mod safe_variant {
    pub use crate::string_builder_safe::*;
}

pub mod unsafe_variant {
    pub use crate::string_builder_unsafe::*;
}

#[cfg(feature = "safe")]
pub use safe_variant::*;

#[cfg(not(feature = "safe"))]
pub use unsafe_variant::*;

#[cfg(test)]
mod tests;
