//! Models for reading spec manifests.

use std::collections::HashMap;
use std::ffi::OsStr;

use serde::{Deserialize, Serialize};

/// Config file format for declaring the set of ruby/spec suites to run.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// Specs by family.
    pub specs: Specs,
}

impl Config {
    /// Construct a new, empty `Config`.
    pub const fn new() -> Self {
        Self { specs: Specs::new() }
    }

    /// Lookup a suite.
    pub fn suites_for_family(&self, family: &OsStr) -> Option<&HashMap<String, Suite>> {
        match family.to_str()? {
            "lanugage" => self.specs.language.as_ref(),
            "core" => self.specs.core.as_ref(),
            "library" => self.specs.library.as_ref(),
            _ => None,
        }
    }
}

/// The set of all ruby/specs to run, by top-level directory.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Specs {
    pub language: Option<HashMap<String, Suite>>,
    pub core: Option<HashMap<String, Suite>>,
    pub library: Option<HashMap<String, Suite>>,
    pub command_line: Option<HashMap<String, Suite>>,
    pub security: Option<HashMap<String, Suite>>,
    pub optional: Option<HashMap<String, Suite>>,
}

impl Specs {
    /// Construct a new, empty `Specs`.
    pub const fn new() -> Self {
        Self {
            language: None,
            core: None,
            library: None,
            command_line: None,
            security: None,
            optional: None,
        }
    }
}

/// The specs to run for a suite or API group.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "include")]
#[serde(rename_all = "snake_case")]
pub enum Suite {
    /// Execute all specs, optionally skipping some.
    All(All),
    /// Execute no specs.
    None,
    /// Execute a set of enumerated specs.
    Set(Set),
}

impl Default for Suite {
    fn default() -> Self {
        Self::new()
    }
}

impl Suite {
    /// Construct a new, empty `Suite` that executes no specs.
    pub const fn new() -> Self {
        Self::None
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct All {
    /// List of specs to always skip because they are known to fail.
    pub skip: Option<Vec<String>>,
}

impl All {
    /// Construct a new `All` that executes all specs in a `Suite`.
    pub const fn new() -> Self {
        Self { skip: None }
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct Set {
    /// List of specs to include.
    pub specs: Vec<String>,
}
