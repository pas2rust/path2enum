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
    assert_eq!(ProjectPaths::SrcノLibRs.to_str(), "src/lib.rs");
    assert_eq!(
        ProjectPaths::TestsノAssetsノArrowLeftSvg.to_str(),
        "tests/assets/arrow-left.svg"
    );
    assert_eq!(ProjectPaths::CargoToml.to_str(), "Cargo.toml");
}

#[magic(path = "tests/assets", ext = "svg", prefix = "icons")]
pub enum Icons {}

#[test]
fn icons() {
    use crate::Icons;
    assert_eq!(Icons::IconsノHomeSvg.to_str(), "icons/home.svg");
    assert_eq!(
        Icons::Iconsノ_11Testノ_11Svg.to_str(),
        "icons/11-test/11.svg"
    );
    assert_eq!(
        Icons::IconsノNestedDirノDeepDirノDeepIconSvg.to_str(),
        "icons/nested_dir/deep_dir/deep-icon.svg"
    );
}
