use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use std::{
    fs,
    path::{Path, PathBuf},
};
use syn::{Attribute, ItemEnum, parse_macro_input};

#[proc_macro_attribute]
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
            Err(meta.error("Only `path`, `ext` and `prefix` are supported"))
        }
    });

    // Se não passou path, usa raiz do projeto "."
    let root = root.unwrap_or_else(|| ".".to_string());
    let ext = ext.unwrap_or_else(|| vec!["svg".to_string()]);
    let root_path = PathBuf::from(&root);

    let enum_ident = &input_enum.ident;

    let mut variants = vec![];
    collect_paths(&root_path, &ext, &mut variants, "", &prefix);

    variants.sort_by(|a, b| a.0.cmp(&b.0));

    let variant_defs = variants.iter().map(|(ident, _)| quote! { #ident, });

    // Para to_str e to_string: convertem variante de volta para path com barras normais
    let match_arms = variants.iter().map(|(ident, original_path)| {
        let lit = syn::LitStr::new(original_path, Span::call_site());
        quote! {
            Self::#ident => #lit,
        }
    });

    let expanded = quote! {#[allow(mixed_script_confusables)]#[allow(mixed_script_confusables)]#[allow(mixed_script_confusables)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #enum_ident {
            #(#variant_defs)*
        }

        impl #enum_ident {
            pub fn to_str(&self) -> &'static str {
                match self {
                    #(#match_arms)*
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
    ext: &[String],
    variants: &mut Vec<(proc_macro2::Ident, String)>,
    relative: &str,
    prefix: &str,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            let full_rel_path = if relative.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", relative, name)
            };

            if path.is_dir() {
                collect_paths(&path, ext, variants, &full_rel_path, prefix);
            } else if path.is_file() {
                if ext.iter().any(|e| name.ends_with(&format!(".{}", e))) {
                    let name_for_enum = if prefix.is_empty() {
                        full_rel_path.clone()
                    } else {
                        format!("{}/{}", prefix, full_rel_path)
                    };

                    // Transforma para ident Rust válido, trocando '/' por ノ (katakana no)
                    let enum_ident =
                        format_ident!("{}", to_valid_rust_ident_with_no(&name_for_enum));
                    variants.push((enum_ident, full_rel_path));
                }
            }
        }
    }
}

/// Converte caminho em um identificador Rust válido, cuidando de caracteres especiais:
/// - Substitui '/' por 'ノ' (katakana no)
/// - Converte para PascalCase cada segmento
/// - Substitui '&' por 'And'
/// - Prefixa números com '_'
fn to_valid_rust_ident_with_no(s: &str) -> String {
    // Divide pelo caminho
    let parts: Vec<&str> = s.split('/').collect();

    let mut segments = vec![];

    for part in parts {
        // Substitui & por And
        let replaced_and = part.replace('&', "And");

        // Converte para PascalCase cada parte (com separadores internos)
        let pascal = replaced_and
            .split(&['-', '_', '.', ' '][..])
            .filter(|p| !p.is_empty())
            .map(|word| {
                let mut chars = word.chars();

                if let Some(first_char) = chars.next() {
                    let mut s = String::new();

                    // Prefixa dígito com _
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

    // Junta segmentos com ノ no lugar de /
    segments.join("ノ")
}
