# path2enum

[![Crates.io](https://img.shields.io/crates/v/path2enum.svg)](https://crates.io/crates/path2enum)  
[![Docs.rs](https://docs.rs/path2enum/badge.svg)](https://docs.rs/path2enum)  
[![License](https://img.shields.io/crates/l/path2enum.svg)](https://github.com/pas2rust/path2enum/blob/master/LICENSE.md)

`path2enum` is a Rust procedural macro that automatically generates enums from your project’s real file paths. It provides **type-safe**, **autocomplete-friendly** access to static assets, config files, or any resources in your filesystem, reducing errors and boosting developer productivity.

---

## Features

- Generate Rust enums directly from directory structures, including nested folders.
- Filter files by extension (e.g., `svg`, `toml`, `rs`).
- Variant names are auto-converted to valid Rust identifiers with readable formatting.
- Uses the unique Japanese character `ノ` to visually separate nested directory names in enum variants.
- Provides `.to_str()` method to get the original file path as a string.
- Supports optional prefixing for virtual namespaces or folder grouping.

---

## Installation

Add `path2enum` to your `Cargo.toml` dependencies:

```toml
[dependencies]
path2enum = "0.3.0"
```
 
## Usage

Import the magic macro and apply it to an empty enum to automatically generate variants representing files in your project directories. You can optionally specify the directory path (path) and file extension filter (ext).


```rust
#![allow(mixed_script_confusables)]
use path2enum::magic;

#[magic(path = "tests/assets", ext = "svg,toml")]
pub enum PublicPaths {}
assert_eq!(PublicPaths::ArrowLeft・svg.to_str(), "arrow-left.svg");
assert_eq!(PublicPaths::NestedDirノIcon・svg.to_str(), "nested_dir/icon.svg");
assert_eq!(PublicPaths::NestedDirノDeepDirノDeepIcon・svg.to_str(), "nested_dir/deep_dir/deep-icon.svg");

#[magic(ext = "rs,svg,toml")]
pub enum ProjectPaths {}
assert_eq!(ProjectPaths::SrcノLib・rs.to_str(), "src/lib.rs");
assert_eq!(ProjectPaths::TestsノAssetsノArrowLeft・svg.to_str(), "tests/assets/arrow-left.svg");
assert_eq!(ProjectPaths::Cargo・toml.to_str(), "Cargo.toml");

#[magic(path = "tests/assets", ext = "svg", prefix = "icons")]
pub enum Icons {}
assert_eq!(Icons::IconsノHome・svg.to_str(), "icons/home.svg");
assert_eq!(Icons::Iconsノ_11Testノ_11・svg.to_str(), "icons/11-test/11.svg");
assert_eq!(Icons::IconsノNestedDirノDeepDirノDeepIcon・svg.to_str(), "icons/nested_dir/deep_dir/deep-icon.svg");
```