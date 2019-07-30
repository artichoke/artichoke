use artichoke_backend::sys;

pub fn build_info() -> String {
    format!(
        "Artichoke [{} rustc {} {} {} {}]",
        sys::mruby_sys_version(true),
        env!("TARGET"),
        env!("RUSTC_VERSION"),
        env!("RUSTC_COMMIT_HASH"),
        env!("RUSTC_COMMIT_DATE")
    )
}
