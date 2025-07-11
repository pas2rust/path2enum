#![allow(mixed_script_confusables)]

use path2enum::magic;

#[magic(path = "tests/assets", ext = "svg,toml")]
pub enum PublicPaths {}

#[test]
fn magic_generation() {
    use crate::PublicPaths;

    assert_eq!(PublicPaths::ArrowLeft・svg.to_str(), "arrow-left.svg");
    assert_eq!(
        PublicPaths::NestedDirノIcon・svg.to_str(),
        "nested_dir/icon.svg"
    );
    assert_eq!(
        PublicPaths::NestedDirノDeepDirノDeepIcon・svg.to_str(),
        "nested_dir/deep_dir/deep-icon.svg"
    );
}

#[magic(ext = "rs,svg,toml")]
pub enum ProjectPaths {}

#[test]
fn magic() {
    use crate::ProjectPaths;
    assert_eq!(ProjectPaths::SrcノLib・rs.to_str(), "src/lib.rs");
    assert_eq!(
        ProjectPaths::TestsノAssetsノArrowLeft・svg.to_str(),
        "tests/assets/arrow-left.svg"
    );
    assert_eq!(ProjectPaths::Cargo・toml.to_str(), "Cargo.toml");
}

#[magic(path = "tests/assets", ext = "svg", prefix = "assets")]
pub enum Icons {}


#[test]
fn icons() {
    use crate::Icons;

    assert_eq!(Icons::AssetsノHome・svg.to_str(), "assets/home.svg");
    assert_eq!(
        Icons::Assetsノ_11Testノ_11・svg.to_str(),
        "assets/11-test/11.svg"
    );
    assert_eq!(
        Icons::AssetsノNestedDirノDeepDirノDeepIcon・svg.to_str(),
        "assets/nested_dir/deep_dir/deep-icon.svg"
    );
}
