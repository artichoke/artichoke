#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::restriction)]

use std::env;
use std::ffi::OsString;
use std::fmt;
use std::process::Command;
use std::str;

use tz::UtcDateTime;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl From<UtcDateTime> for Date {
    fn from(date: UtcDateTime) -> Self {
        Self {
            year: date.year(),
            month: date.month().into(),
            day: date.month_day().into(),
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

pub fn build_release_metadata() {
    let version = env::var_os("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION was not set in build.rs");
    let version = version
        .to_str()
        .expect("CARGO_PKG_VERSION was not a valid UTF-8 String");
    let birth_date = birthdate();
    let build_date = build_date();
    let release_date = build_date;
    let revision_count = revision_count();
    let platform = platform();
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
    emit("ARTICHOKE_COMPILER_VERSION", compiler_version().unwrap_or_default());
}

fn emit<T>(env: &str, value: T)
where
    T: fmt::Display,
{
    println!("cargo:rustc-env={env}={value}");
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
    UtcDateTime::from_timespec(time, 0)
        .expect("Could not construct datetime from birthdate")
        .into()
}

fn build_date() -> Date {
    // Enable reproducibility for `RUBY_RELEASE_DATE` and friends by respecting
    // the `SOURCE_DATE_EPOCH` env variable.
    //
    // https://reproducible-builds.org/docs/source-date-epoch/
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    let datetime = if let Some(timestamp) = env::var_os("SOURCE_DATE_EPOCH") {
        let seconds_since_epoch = timestamp
            .into_string()
            .expect("SOURCE_DATE_EPOCH was not valid UTF-8")
            .parse::<i64>()
            .expect("SOURCE_DATE_EPOCH was not a valid integer");
        UtcDateTime::from_timespec(seconds_since_epoch, 0)
            .expect("Could not construct datetime from SOURCE_DATE_EPOCH")
    } else {
        UtcDateTime::now().expect("Could not retreive current timestamp")
    };
    datetime.into()
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

fn platform() -> String {
    env::var_os("TARGET")
        .expect("cargo-provided TARGET env var not set")
        .to_str()
        .expect("cargo-provided TARGET env var was not valid UTF-8")
        .to_owned()
}

fn copyright(birth_date: Date, build_date: Date) -> String {
    match build_date.year {
        build_date_year if build_date_year == birth_date.year => format!(
            "artichoke - Copyright (c) {} Ryan Lopopolo <rjl@hyperbo.la>",
            build_date.year
        ),
        build_date_year if build_date_year > birth_date.year => format!(
            "artichoke - Copyright (c) {}-{} Ryan Lopopolo <rjl@hyperbo.la>",
            birth_date.year, build_date.year
        ),
        _ => format!(
            "artichoke - Copyright (c) {} Ryan Lopopolo <rjl@hyperbo.la>",
            birth_date.year
        ),
    }
}

fn description(version: &str, release_date: Date, revision_count: Option<usize>, platform: &str) -> String {
    if let Some(revision_count) = revision_count {
        format!("artichoke {version} ({release_date} revision {revision_count}) [{platform}]",)
    } else {
        format!("artichoke {version} ({release_date}) [{platform}]")
    }
}

fn compiler_version() -> Option<String> {
    let cmd = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let compiler_version = Command::new(cmd).arg("-V").output().ok()?;
    let compiler_version = String::from_utf8(compiler_version.stdout).ok()?;
    let mut compiler_version = compiler_version.trim().to_owned();
    if let Some(cc) = artichoke_backend::sys::CC_VERSION {
        compiler_version.push_str(" / ");
        compiler_version.push_str(cc);
    }
    if let Ok(compiler_host) = env::var("HOST") {
        compiler_version.push_str(" on ");
        compiler_version.push_str(&compiler_host);
    }
    Some(compiler_version)
}

fn main() {
    build_release_metadata();
}
