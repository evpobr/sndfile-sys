# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2021-07-17

### Added

- New constants for MP3 (MPEG-1/2/2.5 Layer 3) support added in
  libsndfile 1.1.0-beta1:
  - `SF_FORMAT_MPEG`
  - `SF_FORMAT_MPEG_LAYER_I`
  - `SF_FORMAT_MPEG_LAYER_II`
  - `SF_FORMAT_MPEG_LAYER_III`
  - `SFC_GET_BITRATE_MODE`
  - `SFC_SET_BITRATE_MODE`

## [0.2.2] - 2021-02-15

### Added

- `SFC_SET_OGG_PAGE_LATENCY_MS` constant
- `SFC_SET_OGG_PAGE_LATENCY` constant
- `pkg-config` method to search native library in build script

### Fixed

- libc deprecation warnings

## [0.2.1] - 2019-04-04

### Fixed

- Linking error (thanks to @tuxzz)

## [0.2.0] - 2019-02-16

### Added

- Opus codec support:
  - `SF_FORMAT_OPUS` constant
  - `SFC_GET_ORIGINAL_SAMPLERATE` constant
  - `SFC_SET_ORIGINAL_SAMPLERATE` constant

### Fixed

- Unreleased link in CHANGELOG.md

## [0.1.2] - 2018-11-11

### Added

- [Vcpkg](https://github.com/Microsoft/vcpkg) support with MSVC toolchain. See [README.md](README.md) for details.

## [0.1.1] - 2018-10-26

## Fixed

- Build warning about reduntant linker flag
- Build warning about unused import on non-Windows platforms

## [0.1.0] - 2018-10-17

### Added

- libsndfile API up to [v1.0.29pre1@81a71e0](https://github.com/erikd/libsndfile/commit/81a71e08c09b20b0255aa66e40fce293008b9525)
- [Travis CI tests](https://travis-ci.org/evpobr/sndfile-sys)

[Unreleased]: https://github.com/evpobr/sndfile-sys/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/evpobr/sndfile-sys/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/evpobr/sndfile-sys/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/evpobr/sndfile-sys/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/evpobr/sndfile-sys/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/evpobr/sndfile-sys/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/evpobr/sndfile-sys/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/evpobr/sndfile-sys/compare/f008519...v0.1.0
