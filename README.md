# memonitor

<p align="left">
    <img alt="GitHub" src="https://img.shields.io/github/license/edgenai/memonitor">
</p>

---

**Memonitor** is a lightweight library that allows querying information from various CPU and GPU devices.
The main purpose is the ability to query memory related information, like how much local memory a device has and how
much is currently available to be allocated.

This is achieved by dynamically loading, if present, various device APIs found in the system, and querying them
directly.
At the moment, the following backends are supported:

* INSERT CPU CRATE (CPU only)
* Vulkan
* ~~CUDA (NVIDIA devices only)~~ (planned [#1](https://github.com/edgenai/memonitor/issues/1))
* ~~Metal (macOS only)~~ (planned [#2](https://github.com/edgenai/memonitor/issues/2))

## Platforms

- [x] Linux
- [ ] Windows (needs checking)
- [ ] macOS (needs checking)

## Dependencies

* Clang
* CMake
