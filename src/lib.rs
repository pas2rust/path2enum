use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use std::{
    fs,
    path::{Path, PathBuf},
};
use syn::{Attribute, ItemEnum, parse_macro_input};

/// Trait that converts a string path into a valid Rust enum identifier
trait ToValidIdent {
    fn to_valid_rust_ident_with_no(&self) -> String;
}

impl ToValidIdent for str {
    fn to_valid_rust_ident_with_no(&self) -> String {
        let parts: Vec<&str> = self.split('/').collect();

        let mut segments = Vec::new();

        for part in parts {
            let replaced = part.replace('&', "And");
            let replaced = replaced.replace('.', "・");
            // Convert to PascalCase
            let pascal = replaced
                .split(&['-', '_', '.', ' '][..])
                .filter(|s| !s.is_empty())
                .map(|word| {
                    let mut chars = word.chars();
                    if let Some(first_char) = chars.next() {
                        let mut s = String::new();

                        // Prefix digits with '_'
                        if first_char.is_ascii_digit() {
                            s.push('_');
                            s.push(first_char);
                        } else {
                            s.push(first_char.to_ascii_uppercase());
                        }

                        s.push_str(chars.as_str());
                        s
                    } else {
                        String::new()
                    }
                })
                .collect::<String>();

            segments.push(pascal);
        }

        // Join segments using 'ノ' instead of '/'
        segments.join("ノ")
    }
}

#[proc_macro_attribute]
/// Procedural macro `magic` to generate Rust enums from filesystem files.
///
/// Generates enum variants whose names are derived from real file paths,
/// transformed into valid Rust identifiers, while preserving the original path for access.
///
/// # Macro attributes (parameters):
///
/// - `path: &str`  
///   Root directory where the macro will scan files.  
///   Default: `"."` (project root).
///
/// - `ext: &str`  
///   Allowed file extensions, comma-separated (e.g. `"rs,svg,toml"`).  
///   Default: `"svg"`.
///
/// - `prefix: &str`  
///   Optional prefix added to the returned path from `.to_str()`.  
///   Useful for virtual namespaces or folders.
///
/// # Behavior
///
/// - Recursively scans the `path` directory.
/// - For each file with an extension in `ext`, generates an enum variant.
/// - Variant names are derived from the file path, transformed to valid Rust identifiers:
///   - `/` is replaced by `ノ` (katakana no)
///   - `-`, `_`, `.` (dot), and spaces are treated as separators for PascalCase words
///   - The `.` character is replaced by the Japanese middle dot `・` (U+30FB) in the identifier,
///     so file extensions appear visually separated but valid in Rust identifiers.
///   - Invalid characters replaced (e.g., digit prefixes get a leading underscore)
///   - `&` is replaced by `And`
///
/// - The `.to_str()` method returns the original file path as-is,
///   including hyphens, underscores, dots, and prefix (if any).
///
/// # Examples
///
/// ```rust
/// use path2enum::magic;
///
/// #[magic(path = "tests/assets", ext = "svg,toml")]
/// pub enum PublicPaths {}
///
/// assert_eq!(PublicPaths::ArrowLeft・svg.to_str(), "arrow-left.svg");
/// assert_eq!(PublicPaths::NestedDirノIcon・svg.to_str(), "nested_dir/icon.svg");
/// assert_eq!(PublicPaths::NestedDirノDeepDirノDeepIcon・svg.to_str(), "nested_dir/deep_dir/deep-icon.svg");
///
/// #[magic(ext = "rs,svg,toml")]
/// pub enum ProjectPaths {}
///
/// assert_eq!(ProjectPaths::SrcノLib・rs.to_str(), "src/lib.rs");
/// assert_eq!(ProjectPaths::TestsノAssetsノArrowLeft・svg.to_str(), "tests/assets/arrow-left.svg");
/// assert_eq!(ProjectPaths::Cargo・toml.to_str(), "Cargo.toml");
///
/// #[magic(path = "tests/assets", ext = "svg", prefix = "assets")]
/// pub enum Icons {}
/// assert_eq!(Icons::AssetsノHome・svg.to_str(), "assets/home.svg");
/// assert_eq!(Icons::Assetsノ_11Testノ_11・svg.to_str(),"assets/11-test/11.svg");
/// assert_eq!(Icons::AssetsノNestedDirノDeepDirノDeepIcon・svg.to_str(),"assets/nested_dir/deep_dir/deep-icon.svg");
/// ```
///
/// # Notes
///
/// - The generated enum derives common traits (`Debug, Clone, Copy, PartialEq, Eq`).
/// - The `.to_str()` method returns the original file path for runtime usage.
/// - Generated variant identifiers follow Rust naming rules even for special characters or digit-starting names.
/// - The Japanese middle dot `・` improves readability of extensions inside identifiers without breaking Rust syntax.
///
/// # Typical use case
///
/// Useful for embedding static assets, config files, or resources accessible via enums at compile time,
/// avoiding hardcoded string literals.
///
/// # Requirements
///
/// This macro depends on the `path2enum` crate which should be added as a dependency.
///
///
/// ```ignore
/// // Simplified usage example
/// #[magic(path = "assets/icons", ext = "svg", prefix = "icons")]
/// pub enum Icons {}
///
/// fn main() {
///     println!("{}", Icons::IconsノHome・svg.to_str());
/// }
/// ```
///
pub fn magic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_enum = parse_macro_input!(item as ItemEnum);

    let attr_ts2: TokenStream2 = attr.into();
    let attr: Attribute = syn::parse_quote!(#[magic(#attr_ts2)]);

    let mut root = None;
    let mut ext: Option<Vec<String>> = None;
    let mut prefix = String::new();

    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("path") {
            let value = meta.value()?.parse::<syn::LitStr>()?;
            root = Some(value.value());
            Ok(())
        } else if meta.path.is_ident("ext") {
            let value = meta.value()?.parse::<syn::LitStr>()?;
            let exts = value
                .value()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            ext = Some(exts);
            Ok(())
        } else if meta.path.is_ident("prefix") {
            let value = meta.value()?.parse::<syn::LitStr>()?;
            prefix = value.value();
            Ok(())
        } else {
            Err(meta.error("Only `path`, `ext`, and `prefix` are supported"))
        }
    });

    // Default to project root if no path is provided
    let root = root.unwrap_or_else(|| ".".to_string());
    let ext = ext.unwrap_or_else(|| vec!["svg".to_string()]);
    let root_path = PathBuf::from(&root);

    let enum_ident = &input_enum.ident;

    let mut variants = Vec::new();
    collect_paths(&root_path, &ext, &mut variants, "", &prefix);

    variants.sort_by(|a, b| a.0.cmp(&b.0));

    let variant_defs = variants.iter().map(|(ident, _)| quote! { #ident, });

    let match_arms = variants.iter().map(|(ident, original_path)| {
        let lit = syn::LitStr::new(original_path, Span::call_site());
        quote! {
            Self::#ident => #lit,
        }
    });

    let expanded = quote! {
        #[allow(mixed_script_confusables)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #enum_ident {
            #(#variant_defs)*
        }

        impl #enum_ident {
            pub fn to_str(&self) -> &'static str {
                match self {
                    #(#match_arms)*
                    _ => unreachable!("Unrecognized variant in generated enum {}", stringify!(#enum_ident)),
                }
            }


            pub fn to_string(&self) -> String {
                self.to_str().to_string()
            }
        }
    };

    TokenStream::from(expanded)
}

fn collect_paths(
    dir: &Path,
    allowed_exts: &[String],
    variants: &mut Vec<(proc_macro2::Ident, String)>,
    current_rel_path: &str,
    logical_prefix: &str,
) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        let rel_path = if current_rel_path.is_empty() {
            name.to_string()
        } else {
            format!("{}/{}", current_rel_path, name)
        };

        if path.is_dir() {
            collect_paths(&path, allowed_exts, variants, &rel_path, logical_prefix);
        } else if path.is_file() && has_allowed_extension(&name, allowed_exts) {
            let logical_path = if logical_prefix.is_empty() {
                rel_path.clone()
            } else {
                format!("{}/{}", logical_prefix, rel_path)
            };

            let variant_ident = format_ident!("{}", logical_path.to_valid_rust_ident_with_no());
            variants.push((variant_ident, logical_path));
        }
    }
}


fn has_allowed_extension(file_name: &str, allowed_exts: &[String]) -> bool {
    allowed_exts
        .iter()
        .any(|ext| file_name.ends_with(&format!(".{}", ext)))
}
