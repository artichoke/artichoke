#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
    version_sync::assert_markdown_deps_updated!("artichoke-backend/README.md");
    version_sync::assert_markdown_deps_updated!("artichoke-core/README.md");
    version_sync::assert_markdown_deps_updated!("spec-runner/README.md");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
