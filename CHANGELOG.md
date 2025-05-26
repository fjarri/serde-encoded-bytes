# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.2.1] - 2025-05-26


### Added

- `Base64Url` encoding. ([#4])


[#4]: https://github.com/fjarri/serde-encoded-bytes/pull/4


## [0.2.0] - 2025-02-01

### Changed

- Removed proxy traits `TryFromSliceRef` and `TryFromArray`. ([#3])


### Added

- `GenericArray014` container type to support `generic-array=0.14`. ([#3])
- `BorrowedSliceLike` container type. ([#3])


[#3]: https://github.com/fjarri/serde-encoded-bytes/pull/3


## [0.1.1] - 2025-01-05

### Fixed

- Fixed a possible panic when decoding hex strings ([#2])


### Internal

- Bumped `base64` to 0.22. ([#1])


[#1]: https://github.com/fjarri/serde-encoded-bytes/pull/1
[#2]: https://github.com/fjarri/serde-encoded-bytes/pull/2


## [0.1.0] - 2024-10-16

Initial release.


[0.1.0]: https://github.com/fjarri/serde-encoded-bytes/releases/tag/v0.1.0
[0.1.1]: https://github.com/fjarri/serde-encoded-bytes/releases/tag/v0.1.1
[0.2.0]: https://github.com/fjarri/serde-encoded-bytes/releases/tag/v0.2.0
[0.2.1]: https://github.com/fjarri/serde-encoded-bytes/releases/tag/v0.2.1
