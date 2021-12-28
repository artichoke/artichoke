use insta;
use crate::run;
use std::fs;
use std::path::Path;

#[test]
fn test_ruby_apps() -> Result<(), String> {
    let binary_name = "artichoke";
    let app_paths = Path::new("./tests/integration/apps");

    let paths = fs::read_dir(app_paths).unwrap();

    for path in paths {
        let file_path = path.unwrap().path();

        let str_path = file_path.clone().into_os_string().into_string().unwrap();
        let args = vec![&*str_path];

        let snapshot_name = format!("ruby_app__{}", file_path.strip_prefix(app_paths).unwrap().display());
        insta::assert_debug_snapshot!(snapshot_name, run(binary_name, args))
    }

    Ok(())
}
