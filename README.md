# sndfile-sys

[![Build Status](https://travis-ci.org/evpobr/sndfile-sys.svg?branch=master)](https://travis-ci.org/evpobr/sndfile-sys)

Rust FFI bindings to [libsndfile](https://github.com/erikd/libsndfile).

## Notes

### MSVC toolchain

Starting from version v0.1.2 `sndfile-sys` can use [Vcpkg package manager](https://github.com/Microsoft/vcpkg) to search for
native `libsndfile` library (thanks to [Vcpkg](https://github.com/mcgoo/vcpkg-rs) crate):

* Install `Vcpkg`
* install static libsndfile library (x86|x64): `vcpkg install libsndfile:x64-windows-static`
* Add `Vcpkg` path (e.g. `d:\vcpkg`) to `VCPKG_ROOT` environment variable
* Add `-Ctarget-feature=+crt-static` to `RUSTFLAGS` environment variable

This search method affects `MSVC` toolchain only. You can disable it completely with environment varible `VCPKGRS_DISABLE` set to `1`.

Use can link to dynamic `libsndfile` (not recommended for `MSVC` toolchain):

* Install dynamic `libsndfile` library with command: `vcpkg install libsndfile:x64-windows`
* Delete `RUSTFLAGS` environment variable
* Set `VCPKGRS_DYNAMIC` environment variable to `1`

Starting from version v0.2.2 `sndfile-sys` can use [PkgConfig](git://anongit.freedesktop.org/pkg-config) to search for native
`libsndfile` library (thanks to [pkg-config-rs](https://github.com/rust-lang/pkg-config-rs) crate):
