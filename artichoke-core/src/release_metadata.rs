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
    /// # Examples
    ///
    /// > artichoke - Copyright (c) 2019-2020 Ryan Lopopolo \<rjl@hyperbo.la\>
    fn ruby_copyright(&self) -> &str;

    /// A description of the current build.
    ///
    /// This value will populate the `RUBY_DESCRIPTION` constant.
    ///
    /// # Examples
    ///
    /// > artichoke 0.1.0-pre.0 (2021-01-12 revision 4009) [x86_64-apple-darwin]
    fn ruby_description(&self) -> &str;

    /// The engine, or VM, used in the current build.
    ///
    /// This value will populate the `RUBY_ENGINE` constant.
    ///
    /// # Examples
    ///
    /// > artichoke-mruby
    fn ruby_engine(&self) -> &str;

    /// The version of the engine, or VM, used in the current build.
    ///
    /// This value will populate the `RUBY_ENGINE_VERSION` constant.
    ///
    /// # Examples
    ///
    /// > 0.1.0-pre.0
    fn ruby_engine_version(&self) -> &str;

    /// The patch level the current build.
    ///
    /// This value will populate the `RUBY_PATCHLEVEL` constant.
    ///
    /// # Examples
    ///
    /// > 0
    fn ruby_patchlevel(&self) -> &str;

    /// The target triple of the platform this build targets.
    ///
    /// The platform will be a [Rust or LLVM target triple][triple].
    ///
    /// This value will populate the `RUBY_PLATFORM` constant.
    ///
    /// # Examples
    ///
    /// > x86_64-apple-darwin
    ///
    /// [triple]: https://forge.rust-lang.org/release/platform-support.html
    fn ruby_platform(&self) -> &str;

    /// The build date of this release.
    ///
    /// This value will populate the `RUBY_RELEASE_DATE` constant.
    ///
    /// # Examples
    ///
    /// > 2021-01-12
    fn ruby_release_date(&self) -> &str;

    /// The revision count of the Artichoke git repository used for this build.
    ///
    /// This value will populate the `RUBY_REVISION` constant.
    ///
    /// # Examples
    ///
    /// > 4009
    fn ruby_revision(&self) -> &str;

    /// The target MRI Ruby version for this build.
    ///
    /// This value will populate the `RUBY_VERSION` constant.
    ///
    /// # Examples
    ///
    /// > 3.1.2
    fn ruby_version(&self) -> &str;

    /// A description of the compiler used to build Artichoke.
    ///
    /// This value will populate the `ARTICHOKE_COMPILER_VERSION` constant.
    ///
    /// # Examples
    ///
    /// > rustc 1.49.0 (e1884a8e3 2020-12-29) on x86_64-apple-darwin
    fn artichoke_compiler_version(&self) -> Option<&str>;
}
