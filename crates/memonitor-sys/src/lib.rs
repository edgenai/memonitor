//! Automatically generated bindings for some of [memonitor]'s backends.
//!
//! # Features
//!
//! * `vulkan` - enables the Vulkan backend, enabled by default.
//!
//! [memonitor]: https://crates.io/crates/memonitor

#![warn(missing_docs)]

#[cfg(feature = "cuda")]
pub mod cuda;
#[cfg(feature = "vulkan")]
pub mod vulkan;
