use crate::core;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReleaseMetadata<'a> {
    pub copyright: &'a str,
    pub description: &'a str,
    pub engine: &'a str,
    pub engine_version: &'a str,
    pub patchlevel: &'a str,
    pub platform: &'a str,
    pub release_date: &'a str,
    pub revision: &'a str,
    pub ruby_version: &'a str,
    pub compiler_version: Option<&'a str>,
}

impl<'a> Default for ReleaseMetadata<'a> {
    fn default() -> Self {
        Self {
            copyright: "Copyright (c) 2019 Ryan Lopopolo <rjl@hyperbo.la>",
            description: "Artichoke Ruby",
            engine: "artichoke-mruby",
            engine_version: env!("CARGO_PKG_VERSION"),
            patchlevel: "0",
            platform: "host",
            release_date: "",
            revision: "1",
            ruby_version: "2.6.3",
            compiler_version: Some("rustc"),
        }
    }
}

impl<'a> core::ReleaseMetadata for ReleaseMetadata<'a> {
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ruby_copyright(mut self, copyright: &'a str) -> Self {
        self.copyright = copyright;
        self
    }

    pub fn with_ruby_description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    pub fn with_ruby_engine(mut self, engine: &'a str) -> Self {
        self.engine = engine;
        self
    }

    pub fn with_ruby_engine_version(mut self, engine_version: &'a str) -> Self {
        self.engine_version = engine_version;
        self
    }

    pub fn with_ruby_patchlevel(mut self, patchlevel: &'a str) -> Self {
        self.patchlevel = patchlevel;
        self
    }

    pub fn with_ruby_platform(mut self, platform: &'a str) -> Self {
        self.platform = platform;
        self
    }

    pub fn with_ruby_release_date(mut self, release_date: &'a str) -> Self {
        self.release_date = release_date;
        self
    }

    pub fn with_ruby_revision(mut self, revision: &'a str) -> Self {
        self.revision = revision;
        self
    }

    pub fn with_ruby_version(mut self, ruby_version: &'a str) -> Self {
        self.ruby_version = ruby_version;
        self
    }

    pub fn with_artichoke_compiler_version(mut self, compiler_version: Option<&'a str>) -> Self {
        self.compiler_version = compiler_version;
        self
    }
}
