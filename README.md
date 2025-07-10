# path2enum

[![Crates.io](https://img.shields.io/crates/v/path2enum.svg)](https://crates.io/crates/path2enum)
[![Docs.rs](https://docs.rs/path2enum/badge.svg)](https://docs.rs/path2enum)
[![License](https://img.shields.io/crates/l/path2enum.svg)](https://github.com/pas2rust/path2enum/blob/master/LICENSE.md)

`path2enum` is a procedural macro for Rust that generates enums based on your project's real file paths. It enables type-safe, autocomplete-friendly access to static assets and configuration files, reducing errors and improving developer experience.

## Features

- Automatically generates enums from file system directories.
- Supports nested directories with intuitive enum variants.
- Provides a `.to_str()` method returning the file path as a string.
- Supports filtering by file extension (e.g., `svg`, `toml`).
- Uses a unique delimiter (`ノ`) to separate nested directories in enum variant names.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
path2enum = "0.1.0"
```
 
 
## Usage

Import the magic macro and apply it to an empty enum to automatically generate variants representing files in your project directories. You can optionally specify the directory path (path) and file extension filter (ext).

```rust
 #![allow(mixed_script_confusables)]

  use path2enum::magic;

  #[magic(path = "tests/assets", ext = "svg,toml")]
  pub enum PublicPaths {}

  #[test]
  fn magic_generation() {
      use crate::PublicPaths;

      assert_eq!(PublicPaths::ArrowLeftSvg.to_str(), "arrow-left.svg");
      assert_eq!(
          PublicPaths::NestedDirノIconSvg.to_str(),
          "nested_dir/icon.svg"
      );
      assert_eq!(
          PublicPaths::NestedDirノDeepDirノDeepIconSvg.to_str(),
          "nested_dir/deep_dir/deep-icon.svg"
      );
  }

  #[magic(ext = "rs,svg,toml")]
  pub enum ProjectPaths {}

  #[test]
  fn magic() {
      use crate::ProjectPaths;

      assert_eq!(
          ProjectPaths::SrcノLibRs.to_str(),
          "src/lib.rs"
      );
      assert_eq!(
          ProjectPaths::TestsノAssetsノArrowLeftSvg.to_str(),
          "tests/assets/arrow-left.svg"
      );
      assert_eq!(ProjectPaths::CargoToml.to_str(), "Cargo.toml");
  }
```