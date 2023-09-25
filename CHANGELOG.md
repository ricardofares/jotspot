# Changelog

## [Unreleased]

### Changed

- `if annotations.is_empty()` instead of `if annotations.len() == 0`.
- `fold` instead of `for` loop to create the SelectView.
- `&annotations` instead of `annotations` int the function signature of `build_annotations_layout`.
- `expect` instead of `unwrap` to provide more information in case of an error.
- `if let` instead of `match` to handle the `None` case.
- `?` instead of `unwrap` to handle the `None` case.
- `let` bindings instad of chaining method calls to improve readability.
- `annotate(&content)` instead of `annotate(content.to_string())` to avoid unnecessary string copying.
- `chrono` crate instead of `std::time` to format the timestamp difference.

## [1.0.0] - 2023-09-25

### Added

- Initial release of the Annotation Tool.

[Unreleased]: https://github.com/username/repo/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/username/repo/releases/tag/v1.0.0