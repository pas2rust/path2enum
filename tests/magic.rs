#![allow(mixed_script_confusables)]

use path2enum::magic;

#[magic(path = "path2enum/tests/assets", ext = "svg,toml")]
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

#[magic(ext = "toml")]
pub enum ProjectPaths {}

#[test]
fn magic() {
    use crate::ProjectPaths;

    assert_eq!(
        ProjectPaths::DebugノDebuggerノCargoToml.to_str(),
        "debug/debugger/Cargo.toml"
    );
    assert_eq!(
        ProjectPaths::ClientノLeptosUiノCargoToml.to_str(),
        "client/leptos_ui/Cargo.toml"
    );
    assert_eq!(ProjectPaths::CargoToml.to_str(), "Cargo.toml");
}
