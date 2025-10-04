use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use std::{
    collections::HashSet,
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
        // Split by logical path segments
        let parts: Vec<&str> = self.split('/').collect();
        let mut segments: Vec<String> = Vec::with_capacity(parts.len());

        for part in parts {
            // keep intended replacements: '&' -> And, '.' -> ・ (middle dot)
            let replaced = part.replace('&', "And").replace('.', "・");

            // split into words on common separators (note '.' already replaced above)
            let words = replaced
                .split(&['-', '_', ' '][..])
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            // PascalCase each part (works with Unicode letters)
            let mut pascal = String::new();
            for word in words {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    // use to_uppercase() for Unicode correctness
                    let first_up = first.to_uppercase().to_string();
                    pascal.push_str(&first_up);
                    pascal.push_str(chars.as_str());
                }
            }

            // If the Pascal result is empty (rare), or starts with invalid start char,
            // ensure the segment starts with a valid identifier-start (letter or underscore).
            let safe = match pascal.chars().next() {
                Some(c) if c == '_' => pascal,
                Some(c) if c.is_alphabetic() => pascal, // allows Unicode letters (Katakana included)
                Some(c) if c.is_ascii_digit() => format!("_{}", pascal),
                Some(_) => format!("_{}", pascal),
                None => "_".to_string(),
            };

            segments.push(safe);
        }

        // Join segments with ノ to match your display style
        let joined = segments.join("ノ");

        // Final guard: make sure the overall first char is valid (if not, prefix `_`)
        let ident = match joined.chars().next() {
            Some(c) if c == '_' => joined,
            Some(c) if c.is_alphabetic() => joined,
            Some(c) if c.is_ascii_digit() => format!("_{}", joined),
            Some(_) => format!("_{}", joined),
            None => "_".to_string(),
        };

        ident
    }
}

fn collect_paths(
    dir: &Path,
    allowed_exts: &[String],
    variants: &mut Vec<(proc_macro2::Ident, String)>,
    current_rel_path: &str,
    logical_prefix: &str,
    seen: &mut HashSet<String>,
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
            // logical path for the directory itself
            let logical_dir_path = if logical_prefix.is_empty() {
                rel_path.clone()
            } else {
                format!("{}/{}", logical_prefix, rel_path)
            };

            if !seen.contains(&logical_dir_path) {
                let ident_str = logical_dir_path.to_valid_rust_ident_with_no();
                let dir_ident = format_ident!("{}", ident_str);
                variants.push((dir_ident, logical_dir_path.clone()));
                seen.insert(logical_dir_path.clone());
            }

            // recurse into directory
            collect_paths(&path, allowed_exts, variants, &rel_path, logical_prefix, seen);
        } else if path.is_file() && has_allowed_extension(&name, allowed_exts) {
            let logical_path = if logical_prefix.is_empty() {
                rel_path.clone()
            } else {
                format!("{}/{}", logical_prefix, rel_path)
            };

            if !seen.contains(&logical_path) {
                let ident_str = logical_path.to_valid_rust_ident_with_no();
                let variant_ident = format_ident!("{}", ident_str);
                variants.push((variant_ident, logical_path.clone()));
                seen.insert(logical_path);
            }
        }
    }
}

fn has_allowed_extension(file_name: &str, allowed_exts: &[String]) -> bool {
    allowed_exts
        .iter()
        .any(|ext| file_name.ends_with(&format!(".{}", ext)))
}


#[proc_macro_attribute]
/// Procedural macro `magic` — generates enums from real filesystem paths.
///
/// ## Parameters
/// - `path: &str` — root directory to scan (default: `"."`).
/// - `ext: &str` — comma-separated list of allowed extensions (e.g. `"rs,svg,toml"`; default: `"svg"`).
/// - `prefix: &str` — optional logical prefix added to the paths returned by `to_str()`.
///
/// ## Behavior
/// - Recursively scans the given `path`.
/// - Generates an enum variant **for each directory** and **for each file** matching the allowed extensions.
///   Example: both `a/b/c` (directory) and `a/b/c/file.svg` (file) will appear as distinct variants.
/// - Variants are made readable:
///   - each segment is converted to PascalCase;
///   - path separators are represented by `ノ` (katakana no);
///   - the separator between filename and extension is `・` (middle dot).
/// - If a segment starts with invalid characters for a Rust identifier (e.g. dot, digit), it is prefixed with `_`.
/// - `Enum::to_str()` always returns the original logical path (with hyphens, underscores, extension, and prefix intact).
///
/// ## Examples (doc-tests)
///
/// ```rust
/// # #![allow(mixed_script_confusables)]
/// # use path2enum::magic;
///
/// #[magic(path = "tests/assets", ext = "svg,toml")]
/// pub enum PublicPaths {}
///
/// // file variants
/// assert_eq!(PublicPaths::ArrowLeft・svg.to_str(), "arrow-left.svg");
/// assert_eq!(PublicPaths::NestedDirノIcon・svg.to_str(), "nested_dir/icon.svg");
/// assert_eq!(PublicPaths::NestedDirノDeepDirノDeepIcon・svg.to_str(), "nested_dir/deep_dir/deep-icon.svg");
///
/// // directory variants
/// assert_eq!(PublicPaths::NestedDir.to_str(), "nested_dir");
/// assert_eq!(PublicPaths::NestedDirノDeepDir.to_str(), "nested_dir/deep_dir");
///
/// #[magic(ext = "rs,svg,toml")]
/// pub enum ProjectPaths {}
///
/// assert_eq!(ProjectPaths::SrcノLib・rs.to_str(), "src/lib.rs");
/// assert_eq!(ProjectPaths::TestsノAssetsノArrowLeft・svg.to_str(), "tests/assets/arrow-left.svg");
/// assert_eq!(ProjectPaths::Cargo・toml.to_str(), "Cargo.toml");
///
/// // directory variant inside project
/// assert_eq!(ProjectPaths::TestsノAssets.to_str(), "tests/assets");
///
/// #[magic(path = "tests/assets", ext = "svg", prefix = "assets")]
/// pub enum Icons {}
///
/// assert_eq!(Icons::AssetsノHome・svg.to_str(), "assets/home.svg");
/// assert_eq!(Icons::Assetsノ_11Testノ_11・svg.to_str(), "assets/11-test/11.svg");
/// assert_eq!(Icons::AssetsノNestedDirノDeepDirノDeepIcon・svg.to_str(), "assets/nested_dir/deep_dir/deep-icon.svg");
///
/// // directory variants with prefix
/// assert_eq!(Icons::Assetsノ_11Test.to_str(), "assets/11-test");
/// assert_eq!(Icons::AssetsノNestedDirノDeepDir.to_str(), "assets/nested_dir/deep_dir");
/// ```
///
/// ## Notes
/// - Variants use `ノ` and `・` for readability in code and tests.
/// - An ASCII-only mode (e.g. `ArrowLeft_svg`) could be added as an option if needed.
/// - Use `to_str()` at runtime to retrieve the original logical path.
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
    let mut seen: HashSet<String> = HashSet::new();
    collect_paths(&root_path, &ext, &mut variants, "", &prefix, &mut seen);

    // Sort by readable ident (string form of Ident) for deterministic output
    variants.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));

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
