use std::ffi::OsStr;
use walkdir::{DirEntry, WalkDir};

const MRUBY_SRC_DIR: &str = "mruby-2.0.1/src";
const MRUBY_INCLUDE_DIR: &str = "mruby-2.0.1/include";
const LIBMRUBY: &str = "libmruby.a";

fn keep_entry(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() || is_c_source(entry)
}

fn is_c_source(entry: &DirEntry) -> bool {
    entry.path().extension().and_then(OsStr::to_str) == Some("c")
}

fn main() {
    let mut config = cc::Build::new();
    for entry in WalkDir::new(MRUBY_SRC_DIR).into_iter().filter_entry(keep_entry) {
        let entry = entry.unwrap();
        if is_c_source(&entry) {
            config.file(entry.path());
        }
    }
    config.include(MRUBY_INCLUDE_DIR).compile(LIBMRUBY);
}
