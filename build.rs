#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::restriction)]

use std::env;
use std::ffi::OsString;
use std::fmt;
use std::process::Command;
use std::str::{self, FromStr};

use chrono::prelude::*;
use target_lexicon::Triple;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl From<DateTime<Utc>> for Date {
    fn from(date: DateTime<Utc>) -> Self {
        Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

pub fn build_release_metadata(target: &Triple) {
    let version = env::var_os("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION was not set in build.rs");
    let version = version
        .to_str()
        .expect("CARGO_PKG_VERSION was not a valid UTF-8 String");
    let birth_date = birthdate();
    let build_date = build_date();
    let release_date = build_date;
    let revision_count = revision_count();
    let platform = platform(target);
    let copyright = copyright(birth_date, build_date);
    let description = description(version, release_date, revision_count, platform.as_str());

    emit("RUBY_RELEASE_DATE", release_date);
    emit("RUBY_RELEASE_YEAR", build_date.year);
    emit("RUBY_RELEASE_MONTH", build_date.month);
    emit("RUBY_RELEASE_DAY", build_date.day);
    emit("RUBY_REVISION", revision_count.unwrap_or(0));
    emit("RUBY_PLATFORM", platform);
    emit("RUBY_COPYRIGHT", copyright);
    emit("RUBY_DESCRIPTION", description);
    emit(
        "ARTICHOKE_COMPILER_VERSION",
        compiler_version().unwrap_or_else(String::new),
    );
}

fn emit<T>(env: &str, value: T)
where
    T: fmt::Display,
{
    println!("cargo:rustc-env={}={}", env, value);
}

fn birthdate() -> Date {
    // ```console
    // $ git rev-list --format=%B --max-parents=0 trunk
    // commit db318759dad41686be679c87c349fcb5ff0a396c
    // Initial commit
    // $ git show -s --format="%ct" db318759dad41686be679c87c349fcb5ff0a396c
    // 1554600621
    // $ git show -s --format="%ci" db318759dad41686be679c87c349fcb5ff0a396c
    // 2019-04-06 18:30:21 -0700
    // ```
    let time = 1_554_600_621;
    Utc.timestamp(time, 0).into()
}

fn build_date() -> Date {
    // Enable reproducibility for `RUBY_RELEASE_DATE` and friends by respecting
    // the `SOURCE_DATE_EPOCH` env variable.
    //
    // https://reproducible-builds.org/docs/source-date-epoch/
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    if let Some(timestamp) = env::var_os("SOURCE_DATE_EPOCH") {
        let epoch = timestamp
            .into_string()
            .expect("SOURCE_DATE_EPOCH was not valid UTF-8")
            .parse::<i64>()
            .expect("SOURCE_DATE_EPOCH was not a valid integer");
        Date::from(Utc.timestamp(epoch, 0))
    } else {
        Date::from(Utc::now())
    }
}

fn revision_count() -> Option<usize> {
    let revision_count = Command::new("git")
        .arg("rev-list")
        .arg("--count")
        .arg("HEAD")
        .output()
        .ok()?;
    let output = String::from_utf8(revision_count.stdout).ok()?;
    output.trim().parse::<usize>().ok()
}

fn platform(target: &Triple) -> String {
    target.to_string()
}

fn copyright(birth_date: Date, build_date: Date) -> String {
    if birth_date.year == build_date.year {
        format!(
            "artichoke - Copyright (c) {} Ryan Lopopolo <rjl@hyperbo.la>",
            birth_date.year
        )
    } else {
        format!(
            "artichoke - Copyright (c) {}-{} Ryan Lopopolo <rjl@hyperbo.la>",
            birth_date.year, build_date.year
        )
    }
}

fn description(version: &str, release_date: Date, revision_count: Option<usize>, platform: &str) -> String {
    if let Some(revision_count) = revision_count {
        format!(
            "artichoke {} ({} revision {}) [{}]",
            version, release_date, revision_count, platform
        )
    } else {
        format!("artichoke {} ({}) [{}]", version, release_date, platform)
    }
}

fn compiler_version() -> Option<String> {
    let cmd = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let compiler_version = Command::new(cmd).arg("-V").output().ok()?;
    let compiler_version = String::from_utf8(compiler_version.stdout).ok()?;
    let mut compiler_version = compiler_version.trim().to_owned();
    if let Ok(compiler_host) = env::var("HOST") {
        compiler_version.push_str(" on ");
        compiler_version.push_str(&compiler_host);
    }
    Some(compiler_version)
}

fn main() {
    let target = env::var_os("TARGET").expect("TARGET not set in build.rs");
    let target = target.to_str().expect("TARGET was not a valid UTF-8 String");
    let target = Triple::from_str(target).unwrap_or_else(|_| panic!("Invalid TARGET triple: {}", target));
    build_release_metadata(&target);
}
