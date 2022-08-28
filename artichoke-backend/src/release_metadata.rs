//! Information about an Artichoke build.
//!
//! Release metadata allows populating Ruby constants that describe the build,
//! like `RUBY_COPYRIGHT` for copyright information or `RUBY_PLATFORM` for
//! target architecture.

use artichoke_core::release_metadata;

/// Information about an Artichoke build.
///
/// This build information is injected into `artichoke-backend` by the
/// `artichoke` crate at interpreter initialization time.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReleaseMetadata<'a> {
    /// Copyright information.
    ///
    /// This value will populate the `RUBY_COPYRIGHT` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// artichoke - Copyright (c) 2019-2020 Ryan Lopopolo \<rjl@hyperbo.la\>
    /// ```
    pub copyright: &'a str,
    /// A description of the current build.
    ///
    /// This value will populate the `RUBY_DESCRIPTION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// artichoke 0.1.0-pre.0 (2021-01-12 revision 4009) [x86_64-apple-darwin]
    /// ```
    pub description: &'a str,
    /// The engine, or VM, used in the current build.
    ///
    /// This value will populate the `RUBY_ENGINE` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// artichoke-mruby
    /// ```
    pub engine: &'a str,
    /// The version of the engine, or VM, used in the current build.
    ///
    /// This value will populate the `RUBY_ENGINE_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 0.1.0-pre.0
    /// ```
    pub engine_version: &'a str,
    /// The patch level the current build.
    ///
    /// This value will populate the `RUBY_PATCHLEVEL` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 0
    /// ```
    pub patchlevel: &'a str,
    /// The target triple of the platform this build targets.
    ///
    /// The platform will be a [Rust or LLVM target triple][triple].
    ///
    /// This value will populate the `RUBY_PLATFORM` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// x86_64-apple-darwin
    /// ```
    ///
    /// [triple]: https://forge.rust-lang.org/release/platform-support.html
    pub platform: &'a str,
    /// The build date of this release.
    ///
    /// This value will populate the `RUBY_RELEASE_DATE` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 2021-01-12
    /// ```
    pub release_date: &'a str,
    /// The target MRI Ruby version for this build.
    ///
    /// This value will populate the `RUBY_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 3.1.2
    /// ```
    pub revision: &'a str,
    /// The target MRI Ruby version for this build.
    ///
    /// This value will populate the `RUBY_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 3.1.2
    /// ```
    pub ruby_version: &'a str,
    /// A description of the compiler used to build Artichoke.
    ///
    /// This value will populate the `ARTICHOKE_COMPILER_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// rustc 1.49.0 (e1884a8e3 2020-12-29) on x86_64-apple-darwin
    /// ```
    pub compiler_version: Option<&'a str>,
}

impl<'a> Default for ReleaseMetadata<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> release_metadata::ReleaseMetadata for ReleaseMetadata<'a> {
    fn ruby_copyright(&self) -> &str {
        self.copyright
    }

    fn ruby_description(&self) -> &str {
        self.description
    }

    fn ruby_engine(&self) -> &str {
        self.engine
    }

    fn ruby_engine_version(&self) -> &str {
        self.engine_version
    }

    fn ruby_patchlevel(&self) -> &str {
        self.patchlevel
    }

    fn ruby_platform(&self) -> &str {
        self.platform
    }

    fn ruby_release_date(&self) -> &str {
        self.release_date
    }

    fn ruby_revision(&self) -> &str {
        self.revision
    }

    fn ruby_version(&self) -> &str {
        self.ruby_version
    }

    fn artichoke_compiler_version(&self) -> Option<&str> {
        self.compiler_version
    }
}

impl<'a> ReleaseMetadata<'a> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            copyright: "Copyright (c) 2019 Ryan Lopopolo <rjl@hyperbo.la>",
            description: "Artichoke Ruby",
            engine: "artichoke-mruby",
            engine_version: env!("CARGO_PKG_VERSION"),
            patchlevel: "0",
            platform: "host",
            release_date: "",
            revision: "1",
            ruby_version: "3.1.2",
            compiler_version: Some("rustc"),
        }
    }

    /// Set copyright information.
    ///
    /// This value will populate the `RUBY_COPYRIGHT` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// artichoke - Copyright (c) 2019-2020 Ryan Lopopolo \<rjl@hyperbo.la\>
    /// ```
    #[must_use]
    pub fn with_ruby_copyright(mut self, copyright: &'a str) -> Self {
        self.copyright = copyright;
        self
    }

    /// Set a description of the current build.
    ///
    /// This value will populate the `RUBY_DESCRIPTION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// artichoke 0.1.0-pre.0 (2021-01-12 revision 4009) [x86_64-apple-darwin]
    /// ```
    #[must_use]
    pub fn with_ruby_description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    /// Set the engine, or VM, used in the current build.
    ///
    /// This value will populate the `RUBY_ENGINE` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// artichoke-mruby
    /// ```
    #[must_use]
    pub fn with_ruby_engine(mut self, engine: &'a str) -> Self {
        self.engine = engine;
        self
    }

    /// Set the version of the engine, or VM, used in the current build.
    ///
    /// This value will populate the `RUBY_ENGINE_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 0.1.0-pre.0
    /// ```
    #[must_use]
    pub fn with_ruby_engine_version(mut self, engine_version: &'a str) -> Self {
        self.engine_version = engine_version;
        self
    }

    /// Set the patch level the current build.
    ///
    /// This value will populate the `RUBY_PATCHLEVEL` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 0
    /// ```
    #[must_use]
    pub fn with_ruby_patchlevel(mut self, patchlevel: &'a str) -> Self {
        self.patchlevel = patchlevel;
        self
    }

    /// Set the target triple of the platform this build targets.
    ///
    /// The platform will be a [Rust or LLVM target triple][triple].
    ///
    /// This value will populate the `RUBY_PLATFORM` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// x86_64-apple-darwin
    /// ```
    ///
    /// [triple]: https://forge.rust-lang.org/release/platform-support.html
    #[must_use]
    pub fn with_ruby_platform(mut self, platform: &'a str) -> Self {
        self.platform = platform;
        self
    }

    /// Set the build date of this release.
    ///
    /// This value will populate the `RUBY_RELEASE_DATE` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 2021-01-12
    /// ```
    #[must_use]
    pub fn with_ruby_release_date(mut self, release_date: &'a str) -> Self {
        self.release_date = release_date;
        self
    }

    /// Set the target MRI Ruby version for this build.
    ///
    /// This value will populate the `RUBY_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 3.1.2
    /// ```
    #[must_use]
    pub fn with_ruby_revision(mut self, revision: &'a str) -> Self {
        self.revision = revision;
        self
    }

    /// Set the target MRI Ruby version for this build.
    ///
    /// This value will populate the `RUBY_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 3.1.2
    /// ```
    #[must_use]
    pub fn with_ruby_version(mut self, ruby_version: &'a str) -> Self {
        self.ruby_version = ruby_version;
        self
    }

    /// Set a description of the compiler used to build Artichoke.
    ///
    /// This value will populate the `ARTICHOKE_COMPILER_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// rustc 1.49.0 (e1884a8e3 2020-12-29) on x86_64-apple-darwin
    /// ```
    #[must_use]
    pub fn with_artichoke_compiler_version(mut self, compiler_version: Option<&'a str>) -> Self {
        self.compiler_version = compiler_version;
        self
    }

    /// Copyright information.
    ///
    /// This value will populate the `RUBY_COPYRIGHT` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// artichoke - Copyright (c) 2019-2020 Ryan Lopopolo \<rjl@hyperbo.la\>
    /// ```
    #[must_use]
    pub const fn ruby_copyright(&self) -> &str {
        self.copyright
    }

    /// A description of the current build.
    ///
    /// This value will populate the `RUBY_DESCRIPTION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// artichoke 0.1.0-pre.0 (2021-01-12 revision 4009) [x86_64-apple-darwin]
    /// ```
    #[must_use]
    pub const fn ruby_description(&self) -> &str {
        self.description
    }

    /// The engine, or VM, used in the current build.
    ///
    /// This value will populate the `RUBY_ENGINE` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// artichoke-mruby
    /// ```
    #[must_use]
    pub const fn ruby_engine(&self) -> &str {
        self.engine
    }

    /// The version of the engine, or VM, used in the current build.
    ///
    /// This value will populate the `RUBY_ENGINE_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 0.1.0-pre.0
    /// ```
    #[must_use]
    pub const fn ruby_engine_version(&self) -> &str {
        self.engine_version
    }

    /// The patch level the current build.
    ///
    /// This value will populate the `RUBY_PATCHLEVEL` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 0
    /// ```
    #[must_use]
    pub const fn ruby_patchlevel(&self) -> &str {
        self.patchlevel
    }

    /// The target triple of the platform this build targets.
    ///
    /// The platform will be a [Rust or LLVM target triple][triple].
    ///
    /// This value will populate the `RUBY_PLATFORM` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// x86_64-apple-darwin
    /// ```
    ///
    /// [triple]: https://forge.rust-lang.org/release/platform-support.html
    #[must_use]
    pub const fn ruby_platform(&self) -> &str {
        self.platform
    }

    /// The build date of this release.
    ///
    /// This value will populate the `RUBY_RELEASE_DATE` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 2021-01-12
    /// ```
    #[must_use]
    pub const fn ruby_release_date(&self) -> &str {
        self.release_date
    }

    /// The target MRI Ruby version for this build.
    ///
    /// This value will populate the `RUBY_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 3.1.2
    /// ```
    #[must_use]
    pub const fn ruby_revision(&self) -> &str {
        self.revision
    }

    /// The target MRI Ruby version for this build.
    ///
    /// This value will populate the `RUBY_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// 3.1.2
    /// ```
    #[must_use]
    pub const fn ruby_version(&self) -> &str {
        self.ruby_version
    }

    /// A description of the compiler used to build Artichoke.
    ///
    /// This value will populate the `ARTICHOKE_COMPILER_VERSION` constant.
    ///
    /// # Examples
    ///
    /// ```text
    /// rustc 1.49.0 (e1884a8e3 2020-12-29) on x86_64-apple-darwin
    /// ```
    #[must_use]
    pub const fn artichoke_compiler_version(&self) -> Option<&str> {
        self.compiler_version
    }
}
