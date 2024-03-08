# memonitor

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Documentation](https://docs.rs/memonitor/badge.svg)](https://docs.rs/memonitor/)
[![Crate](https://img.shields.io/crates/v/memonitor.svg)](https://crates.io/crates/memonitor)

---

**Memonitor** is a lightweight library that allows querying information from various CPU and GPU devices.
The main purpose is the ability to query memory related information, like how much local memory a device has and how
much is currently available to be allocated.

This is achieved by dynamically loading, if present, various device APIs found in the system, and querying them
directly.
At the moment, the following backends are supported:

- [x] [sysinfo](https://crates.io/crates/sysinfo) (CPU only)
- [x] Vulkan
- [ ] ~~CUDA (NVIDIA devices only)~~ (planned [#1](https://github.com/edgenai/memonitor/issues/1))
- [ ] ~~Metal (macOS only)~~ (planned [#2](https://github.com/edgenai/memonitor/issues/2))

## Platforms

* Linux
* Windows
* macOS

## Dependencies

* Clang
* CMake
