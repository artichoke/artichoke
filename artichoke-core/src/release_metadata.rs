//! Information about an Artichoke build.
//!
//! Release metadata allows populating Ruby constants that describe the build,
//! like `RUBY_COPYRIGHT` for copyright information or `RUBY_PLATFORM` for
//! target architecture.

/// Information about an Artichoke build.
///
/// Implementations of this trait are used to set global Ruby constants that
/// describe the current build.
pub trait ReleaseMetadata {
    /// Copyright information.
    ///
    /// This value will populate the `RUBY_COPYRIGHT` constant.
    ///
    /// # Example
    ///
    /// > Copyright (c) 2019-2020 Ryan Lopopolo <rjl@hyperbo.la>
    fn ruby_copyright(&self) -> &str;

    /// A description of the current build.
    ///
    /// This value will populate the `RUBY_DESCRIPTION` constant.
    ///
    /// # Example
    ///
    /// > artichoke 0.1.0 (2020-06-05) [x86_64-darwin]
    fn ruby_description(&self) -> &str;

    /// The engine, or VM, used in the current build.
    ///
    /// This value will populate the `RUBY_ENGINE` constant.
    ///
    /// # Example
    ///
    /// > artichoke-mruby
    fn ruby_engine(&self) -> &str;

    /// The version of the engine, or VM, used in the current build.
    ///
    /// This value will populate the `RUBY_ENGINE_VERSION` constant.
    ///
    /// # Example
    ///
    /// > 0.1.0
    fn ruby_engine_version(&self) -> &str;

    /// The patch level the current build.
    ///
    /// This value will populate the `RUBY_PATCHLEVEL` constant.
    fn ruby_patchlevel(&self) -> &str;

    /// The target triple of the platform this build targets.
    ///
    /// The platform will be a [Rust or LLVM target triple][triple].
    ///
    /// This value will populate the `RUBY_PLATFORM` constant.
    ///
    /// [triple]: https://forge.rust-lang.org/release/platform-support.html
    fn ruby_platform(&self) -> &str;

    /// The build date of this release.
    ///
    /// This value will populate the `RUBY_RELEASE_DATE` constant.
    fn ruby_release_date(&self) -> &str;

    /// The revision count of the Artichoke git repo used for this build.
    ///
    /// This value will populate the `RUBY_REVISION` constant.
    fn ruby_revision(&self) -> &str;

    /// The target MRI Ruby version for this build.
    ///
    /// This value will populate the `RUBY_VERSION` constant.
    ///
    /// # Example
    ///
    /// > 2.6.3
    fn ruby_version(&self) -> &str;

    /// A description of the compiler used to build Artichoke.
    ///
    /// This value will populate the `ARTICHOKE_COMPILER_VERSION` constant.
    ///
    /// # Example
    ///
    /// > rustc 1.44.0 (49cae5576 2020-06-01)
    fn artichoke_compiler_version(&self) -> Option<&str>;
}
