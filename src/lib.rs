#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! The `fermium` crate is a set of bindings to SDL2, currently 2.0.9.
//!
//! `bindgen` is used to generate the bindings from the official SDL2 include
//! files. At the moment we include the following:
//!
//! * `SDL2.h`
//! * `SDL_syswm.h`
//! * `SDL_vulkan.h`
//!
//! However, `SDL_syswm.h` in particular pulls in a bunch of extra code and it
//! overwhelms the generated bindings. To avoid this, we only keep the following
//! whitelist of items:
//!
//! * `SDL_` (functions, types, and vars)
//! * `SDLK_` (vars)
//! * `AUDIO_` (vars)
//! * Anything that one of these depends on also gets pulled in.
//!
//! It is thought that this will expose all needed functionality, but if you
//! think something should be added to the whitelist please [submit an
//! issue](https://github.com/Lokathor/fermium/issues).

// Note(Lokathor): Only one of the following `include!` directives should end up
// being used in any given build.

#[cfg(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(
  all(target_os = "windows", target_arch = "x86_64"),
  not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))
))]
include!("bindings_win32_msvc_x64.rs");

#[cfg(all(
  all(target_os = "linux", target_arch = "x86_64"),
  not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))
))]
include!("bindings_linux_x64.rs");

#[cfg(all(
  all(target_os = "linux", target_arch = "arm", target_env = "gnu"),
  not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))
))]
include!("bindings_rpi3.rs");

#[cfg(all(
  all(target_os = "macos", target_arch = "x86_64"),
  not(any(feature = "use_bindgen_bin", feature = "use_bindgen_lib"))
))]
include!("bindings_mac_x64.rs");

// Note(Lokathor): Bindgen doesn't parse all things properly on its own, so we
// define a few more things here "by hand".

/// Used as the device ID for mouse events simulated with touch input
///
/// `SDL_touch.h`, line 53
pub const SDL_TOUCH_MOUSEID: u32 = -1i32 as u32;
