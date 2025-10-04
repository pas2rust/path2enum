# path2enum
[![Crates.io](https://img.shields.io/crates/v/path2enum.svg)](https://crates.io/crates/path2enum)
[![Docs.rs](https://docs.rs/path2enum/badge.svg)](https://docs.rs/path2enum)
[![License](https://img.shields.io/crates/l/path2enum.svg)](https://github.com/pas2rust/path2enum/blob/main/LICENSE)
![GitHub top language](https://img.shields.io/github/languages/top/pas2rust/path2enum?color=orange&logo=rust&style=flat&logoColor=white)
![GitHub stars](https://img.shields.io/github/stars/pas2rust/path2enum?color=success&style=flat&logo=github)
![GitHub forks](https://img.shields.io/github/forks/pas2rust/path2enum?color=orange&logo=Furry%20Network&style=flat&logoColor=white)
![Tests](https://raw.githubusercontent.com/pas2rust/badges/main/path2enum-tests.svg)
![Crates.io downloads](https://img.shields.io/crates/d/path2enum.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/pas2rust/path2enum?color=ff69b4&label=update&logo=git&style=flat&logoColor=white)

`path2enum` is a Rust procedural macro that automatically generates enums from your project’s real file paths. It provides **type-safe**, **autocomplete-friendly** access to static assets, config files, or any resources in your filesystem, reducing errors and boosting developer productivity.

---

## 🔨 Features 

- Generate Rust enums directly from directory structures, including nested folders.
- Filter files by extension (e.g., `svg`, `toml`, `rs`).
- Variant names are auto-converted to valid Rust identifiers with readable formatting.
- Uses the unique Japanese character `ノ` to visually separate nested directory names in enum variants.
- Provides `.to_str()` method to get the original file path as a string.
- Supports optional prefixing for virtual namespaces or folder grouping.

---

## ⚙️ Installation 

Add `path2enum` to your `Cargo.toml` dependencies:

```bash
cargo add path2enum
```
 
## 🚀 Usage 

Import the magic macro and apply it to an empty enum to automatically generate variants representing files in your project directories. You can optionally specify the directory path (path) and file extension filter (ext).


```rust
#![allow(mixed_script_confusables)]
use path2enum::magic;

#[magic(path = "tests/assets", ext = "svg,toml")]
pub enum PublicPaths {}
assert_eq!(PublicPaths::ArrowLeft・svg.to_str(), "arrow-left.svg");
assert_eq!(PublicPaths::NestedDirノIcon・svg.to_str(), "nested_dir/icon.svg");
assert_eq!(PublicPaths::NestedDirノDeepDirノDeepIcon・svg.to_str(), "nested_dir/deep_dir/deep-icon.svg");
assert_eq!(PublicPaths::NestedDir.to_str(), "nested_dir");
assert_eq!(PublicPaths::NestedDirノDeepDir.to_str(), "nested_dir/deep_dir");

#[magic(ext = "rs,svg,toml")]
pub enum ProjectPaths {}
assert_eq!(ProjectPaths::SrcノLib・rs.to_str(), "src/lib.rs");
assert_eq!(ProjectPaths::TestsノAssetsノArrowLeft・svg.to_str(), "tests/assets/arrow-left.svg");
assert_eq!(ProjectPaths::Cargo・toml.to_str(), "Cargo.toml");
assert_eq!(ProjectPaths::TestsノAssets.to_str(), "tests/assets");

#[magic(path = "tests/assets", ext = "svg", prefix = "assets")]
pub enum Icons {}
assert_eq!(Icons::AssetsノHome・svg.to_str(), "assets/home.svg");
assert_eq!(Icons::Assetsノ_11Testノ_11・svg.to_str(),"assets/11-test/11.svg");
assert_eq!(Icons::AssetsノNestedDirノDeepDirノDeepIcon・svg.to_str(),"assets/nested_dir/deep_dir/deep-icon.svg");
assert_eq!(Icons::Assetsノ_11Test.to_str(), "assets/11-test");
assert_eq!(Icons::AssetsノNestedDirノDeepDir.to_str(), "assets/nested_dir/deep_dir");
```

---

<h2 align="center">
  <strong>❤️ Donate</strong>
</h2>

<p align="center">
  <a href="https://github.com/pas2rust/pas2rust/blob/main/pas-monero-donate.png" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Monero%20QR-FF6600?style=flat&logo=monero&logoColor=white" alt="Monero QR"/>
  </a>
  <a href="https://github.com/pas2rust/pas2rust/blob/main/pas-bitcoin-donate.png" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/BTC%20QR-EAB300?style=flat&logo=bitcoin&logoColor=white" alt="BTC QR"/>
  </a>
  <a href="https://revolut.me/pas2rust" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Revolut%20QR-Blue?style=flat&logo=revolut&logoColor=white" alt="Revolut QR"/>
  </a>
  <a href="https://wise.com/pay/me/pedroaugustos99" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Wise%20QR-1CA0F2?style=flat&logo=wise&logoColor=white" alt="Wise QR"/>
  </a>
</p>


---