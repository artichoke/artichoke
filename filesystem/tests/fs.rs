extern crate filesystem;

use std::io::ErrorKind;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use filesystem::UnixFileSystem;
use filesystem::{DirEntry, FakeFileSystem, FileSystem, OsFileSystem, TempDir, TempFileSystem};

macro_rules! make_test {
    ($test:ident, $fs:expr) => {
        #[test]
        fn $test() {
            let fs = $fs();
            let temp_dir = fs.temp_dir("test").unwrap();

            super::$test(&fs, temp_dir.path());
        }
    };
}

macro_rules! test_fs {
    ($name:ident, $fs:expr) => {
        mod $name {
            use super::*;

            make_test!(set_current_dir_fails_if_node_does_not_exists, $fs);
            make_test!(set_current_dir_fails_if_node_is_a_file, $fs);

            make_test!(is_dir_returns_true_if_node_is_dir, $fs);
            make_test!(is_dir_returns_false_if_node_is_file, $fs);
            make_test!(is_dir_returns_false_if_node_does_not_exist, $fs);

            make_test!(is_file_returns_true_if_node_is_file, $fs);
            make_test!(is_file_returns_false_if_node_is_dir, $fs);
            make_test!(is_file_returns_false_if_node_does_not_exist, $fs);

            make_test!(create_dir_creates_new_dir, $fs);
            make_test!(create_dir_fails_if_dir_already_exists, $fs);
            make_test!(create_dir_fails_if_parent_does_not_exist, $fs);

            make_test!(create_dir_all_creates_dirs_in_path, $fs);
            make_test!(create_dir_all_still_succeeds_if_any_dir_already_exists, $fs);

            make_test!(remove_dir_deletes_dir, $fs);
            make_test!(remove_dir_does_not_affect_parent, $fs);
            make_test!(remove_dir_fails_if_node_does_not_exist, $fs);
            make_test!(remove_dir_fails_if_node_is_a_file, $fs);
            make_test!(remove_dir_fails_if_dir_is_not_empty, $fs);

            make_test!(remove_dir_all_removes_dir_and_contents, $fs);
            make_test!(remove_dir_all_fails_if_node_is_a_file, $fs);

            make_test!(read_dir_returns_dir_entries, $fs);
            make_test!(read_dir_fails_if_node_does_not_exist, $fs);
            make_test!(read_dir_fails_if_node_is_a_file, $fs);

            make_test!(write_file_writes_to_new_file, $fs);
            make_test!(write_file_overwrites_contents_of_existing_file, $fs);
            make_test!(write_file_fails_if_file_is_readonly, $fs);
            make_test!(write_file_fails_if_node_is_a_directory, $fs);

            make_test!(overwrite_file_overwrites_contents_of_existing_file, $fs);
            make_test!(overwrite_file_fails_if_node_does_not_exist, $fs);
            make_test!(overwrite_file_fails_if_file_is_readonly, $fs);
            make_test!(overwrite_file_fails_if_node_is_a_directory, $fs);

            make_test!(read_file_returns_contents_as_bytes, $fs);
            make_test!(read_file_fails_if_file_does_not_exist, $fs);

            make_test!(read_file_to_string_returns_contents_as_string, $fs);
            make_test!(read_file_to_string_fails_if_file_does_not_exist, $fs);
            make_test!(read_file_to_string_fails_if_contents_are_not_utf8, $fs);

            make_test!(read_file_into_writes_bytes_to_buffer, $fs);
            make_test!(read_file_into_fails_if_file_does_not_exist, $fs);

            make_test!(create_file_writes_writes_to_new_file, $fs);
            make_test!(create_file_fails_if_file_already_exists, $fs);

            make_test!(remove_file_removes_a_file, $fs);
            make_test!(remove_file_fails_if_file_does_not_exist, $fs);
            make_test!(remove_file_fails_if_node_is_a_directory, $fs);

            make_test!(copy_file_copies_a_file, $fs);
            make_test!(copy_file_overwrites_destination_file, $fs);
            make_test!(copy_file_fails_if_original_file_does_not_exist, $fs);
            make_test!(copy_file_fails_if_destination_file_is_readonly, $fs);
            make_test!(copy_file_fails_if_original_node_is_directory, $fs);
            make_test!(copy_file_fails_if_destination_node_is_directory, $fs);

            make_test!(rename_renames_a_file, $fs);
            make_test!(rename_renames_a_directory, $fs);
            make_test!(rename_overwrites_destination_file, $fs);
            make_test!(rename_overwrites_empty_destination_directory, $fs);
            make_test!(rename_renames_all_descendants, $fs);
            make_test!(rename_fails_if_original_path_does_not_exist, $fs);
            make_test!(
                rename_fails_if_original_and_destination_are_different_types,
                $fs
            );
            make_test!(rename_fails_if_destination_directory_is_not_empty, $fs);

            make_test!(readonly_returns_write_permission, $fs);
            make_test!(readonly_fails_if_node_does_not_exist, $fs);

            make_test!(set_readonly_toggles_write_permission_of_file, $fs);
            make_test!(set_readonly_toggles_write_permission_of_dir, $fs);
            make_test!(set_readonly_fails_if_node_does_not_exist, $fs);

            make_test!(len_returns_size_of_file, $fs);
            make_test!(len_returns_size_of_directory, $fs);
            make_test!(len_returns_0_if_node_does_not_exist, $fs);

            #[cfg(unix)]
            make_test!(mode_returns_permissions, $fs);
            #[cfg(unix)]
            make_test!(mode_fails_if_node_does_not_exist, $fs);

            #[cfg(unix)]
            make_test!(set_mode_sets_permissions, $fs);
            #[cfg(unix)]
            make_test!(set_mode_fails_if_node_does_not_exist, $fs);

            make_test!(temp_dir_creates_tempdir, $fs);
            make_test!(temp_dir_creates_unique_dir, $fs);
        }
    };
}

test_fs!(os, OsFileSystem::new);
test_fs!(fake, FakeFileSystem::new);

fn set_current_dir_fails_if_node_does_not_exists<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("does_not_exist");

    let result = fs.set_current_dir(path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn set_current_dir_fails_if_node_is_a_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("file");

    fs.create_file(&path, "").unwrap();

    let result = fs.set_current_dir(path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);
}

fn is_dir_returns_true_if_node_is_dir<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("new_dir");

    fs.create_dir(&path).unwrap();

    assert!(fs.is_dir(&path));
}

fn is_dir_returns_false_if_node_is_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("new_dir");

    fs.create_file(&path, "").unwrap();

    assert!(!fs.is_dir(&path));
}

fn is_dir_returns_false_if_node_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    assert!(!fs.is_dir(parent.join("does_not_exist")));
}

fn is_file_returns_true_if_node_is_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("new_file");

    fs.create_file(&path, "").unwrap();

    assert!(fs.is_file(&path));
}

fn is_file_returns_false_if_node_is_dir<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("new_dir");

    fs.create_dir(&path).unwrap();

    assert!(!fs.is_file(&path));
}

fn is_file_returns_false_if_node_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    assert!(!fs.is_file(parent.join("does_not_exist")));
}

fn create_dir_creates_new_dir<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("new_dir");

    let result = fs.create_dir(&path);

    assert!(result.is_ok());
    assert!(fs.is_dir(path));
}

fn create_dir_fails_if_dir_already_exists<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("new_dir");

    fs.create_dir(&path).unwrap();

    let result = fs.create_dir(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::AlreadyExists);
}

fn create_dir_fails_if_parent_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("parent/new_dir");

    let result = fs.create_dir(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn create_dir_all_creates_dirs_in_path<T: FileSystem>(fs: &T, parent: &Path) {
    let result = fs.create_dir_all(parent.join("a/b/c"));

    assert!(result.is_ok());
    assert!(fs.is_dir(parent.join("a")));
    assert!(fs.is_dir(parent.join("a/b")));
    assert!(fs.is_dir(parent.join("a/b/c")));
}

fn create_dir_all_still_succeeds_if_any_dir_already_exists<T: FileSystem>(fs: &T, parent: &Path) {
    fs.create_dir_all(parent.join("a/b")).unwrap();

    let result = fs.create_dir_all(parent.join("a/b/c"));

    assert!(result.is_ok());
    assert!(fs.is_dir(parent.join("a")));
    assert!(fs.is_dir(parent.join("a/b")));
    assert!(fs.is_dir(parent.join("a/b/c")));
}

fn remove_dir_deletes_dir<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("dir");

    fs.create_dir(&path).unwrap();

    let result = fs.remove_dir(&path);

    assert!(result.is_ok());
    assert!(!fs.is_dir(&path));
}

fn remove_dir_does_not_affect_parent<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("parent/child");

    fs.create_dir_all(&path).unwrap();

    let result = fs.remove_dir(&path);

    assert!(result.is_ok());
    assert!(fs.is_dir(parent.join("parent")));
    assert!(!fs.is_dir(parent.join("child")));
}

fn remove_dir_fails_if_node_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let result = fs.remove_dir(parent.join("does_not_exist"));

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn remove_dir_fails_if_node_is_a_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("file");

    fs.create_file(&path, "").unwrap();

    let result = fs.remove_dir(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);
    assert!(fs.is_file(&path));
}

fn remove_dir_fails_if_dir_is_not_empty<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("dir");
    let child = path.join("file");

    fs.create_dir(&path).unwrap();
    fs.create_file(&child, "").unwrap();

    let result = fs.remove_dir(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);
    assert!(fs.is_dir(&path));
    assert!(fs.is_file(&child));
}

fn remove_dir_all_removes_dir_and_contents<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("dir");
    let child = path.join("file");

    fs.create_dir(&path).unwrap();
    fs.create_file(&child, "").unwrap();

    let result = fs.remove_dir_all(&path);

    assert!(result.is_ok());
    assert!(!fs.is_dir(&path));
    assert!(!fs.is_file(&child));
    assert!(fs.is_dir(parent));
}

fn remove_dir_all_fails_if_node_is_a_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("file");

    fs.create_file(&path, "").unwrap();

    let result = fs.remove_dir_all(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);
    assert!(fs.is_file(&path));
}

fn read_dir_returns_dir_entries<T: FileSystem>(fs: &T, parent: &Path) {
    let file1 = parent.join("file1");
    let file2 = parent.join("file2");
    let dir1 = parent.join("dir1");
    let dir2 = parent.join("dir2");
    let file3 = dir1.join("file3");
    let file4 = dir2.join("file4");

    fs.create_file(&file1, "").unwrap();
    fs.create_file(&file2, "").unwrap();
    fs.create_dir(&dir1).unwrap();
    fs.create_dir(&dir2).unwrap();
    fs.create_file(&file3, "").unwrap();
    fs.create_file(&file4, "").unwrap();

    let result = fs.read_dir(parent);

    assert!(result.is_ok());

    let mut entries: Vec<PathBuf> = result.unwrap().map(|e| e.unwrap().path()).collect();
    let expected_paths = &mut [file1, file2, dir1, dir2];

    entries.sort();
    expected_paths.sort();

    assert_eq!(&entries, expected_paths);
}

fn read_dir_fails_if_node_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("does_not_exist");
    let result = fs.read_dir(&path);

    assert!(result.is_err());

    match result {
        Ok(_) => panic!("should be an err"),
        Err(err) => assert_eq!(err.kind(), ErrorKind::NotFound),
    }
}

fn read_dir_fails_if_node_is_a_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("file");

    fs.create_file(&path, "").unwrap();

    let result = fs.read_dir(&path);

    assert!(result.is_err());
    match result {
        Ok(_) => panic!("should be an err"),
        Err(err) => assert_eq!(err.kind(), ErrorKind::Other),
    }
}

fn write_file_writes_to_new_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("new_file");
    let result = fs.write_file(&path, "new contents");

    assert!(result.is_ok());

    let contents = String::from_utf8(fs.read_file(path).unwrap()).unwrap();

    assert_eq!(&contents, "new contents");
}

fn write_file_overwrites_contents_of_existing_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_file");

    fs.write_file(&path, "old contents").unwrap();

    let result = fs.write_file(&path, "new contents");

    assert!(result.is_ok());

    let contents = String::from_utf8(fs.read_file(path).unwrap()).unwrap();

    assert_eq!(&contents, "new contents");
}

fn write_file_fails_if_file_is_readonly<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_file");

    fs.create_file(&path, "").unwrap();
    fs.set_readonly(&path, true).unwrap();

    let result = fs.write_file(&path, "test contents");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::PermissionDenied);
}

fn write_file_fails_if_node_is_a_directory<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_dir");

    fs.create_dir(&path).unwrap();

    let result = fs.write_file(&path, "test contents");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);
}

fn overwrite_file_overwrites_contents_of_existing_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_file");

    fs.write_file(&path, "old contents").unwrap();

    let result = fs.overwrite_file(&path, "new contents");

    assert!(result.is_ok());

    let contents = String::from_utf8(fs.read_file(path).unwrap()).unwrap();

    assert_eq!(&contents, "new contents");
}

fn overwrite_file_fails_if_node_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("new_file");
    let result = fs.overwrite_file(&path, "new contents");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn overwrite_file_fails_if_file_is_readonly<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_file");

    fs.create_file(&path, "").unwrap();
    fs.set_readonly(&path, true).unwrap();

    let result = fs.overwrite_file(&path, "test contents");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::PermissionDenied);
}

fn overwrite_file_fails_if_node_is_a_directory<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_dir");

    fs.create_dir(&path).unwrap();

    let result = fs.overwrite_file(&path, "test contents");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);
}

fn read_file_returns_contents_as_bytes<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test.txt");

    fs.write_file(&path, "test text").unwrap();

    let result = fs.read_file(&path);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), br"test text");
}

fn read_file_fails_if_file_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test.txt");
    let result = fs.read_file(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn read_file_to_string_returns_contents_as_string<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test.txt");

    fs.write_file(&path, "test text").unwrap();

    let result = fs.read_file_to_string(&path);

    assert!(result.is_ok());
    assert_eq!(&result.unwrap(), "test text");
}

fn read_file_to_string_fails_if_file_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test.txt");
    let result = fs.read_file_to_string(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn read_file_to_string_fails_if_contents_are_not_utf8<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test.txt");

    fs.write_file(&path, &[0, 159, 146, 150]).unwrap();

    let result = fs.read_file_to_string(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidData);
}

fn read_file_into_writes_bytes_to_buffer<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test.txt");
    let text = "test text";

    fs.write_file(&path, text).unwrap();
    let mut buf = Vec::new();

    let result = fs.read_file_into(&path, &mut buf);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), text.as_bytes().len());
    assert_eq!(buf, br"test text");
}

fn read_file_into_fails_if_file_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test.txt");

    let result = fs.read_file_into(&path, &mut Vec::new());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn create_file_writes_writes_to_new_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_file");
    let result = fs.create_file(&path, "new contents");

    assert!(result.is_ok());

    let contents = String::from_utf8(fs.read_file(path).unwrap()).unwrap();

    assert_eq!(&contents, "new contents");
}

fn create_file_fails_if_file_already_exists<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_file");

    fs.create_file(&path, "contents").unwrap();

    let result = fs.create_file(&path, "new contents");

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::AlreadyExists);
}

fn remove_file_removes_a_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_file");

    fs.create_file(&path, "").unwrap();

    let result = fs.remove_file(&path);

    assert!(result.is_ok());

    let result = fs.read_file(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn remove_file_fails_if_file_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let result = fs.remove_file(parent.join("does_not_exist"));

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn remove_file_fails_if_node_is_a_directory<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_dir");

    fs.create_dir(&path).unwrap();

    let result = fs.remove_file(&path);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);
}

fn copy_file_copies_a_file<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");

    fs.create_file(&from, "test").unwrap();

    let result = fs.copy_file(&from, &to);

    assert!(result.is_ok());

    let result = fs.read_file(&to);

    assert!(result.is_ok());
    assert_eq!(&result.unwrap(), b"test");
}

fn copy_file_overwrites_destination_file<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");

    fs.create_file(&from, "expected").unwrap();
    fs.create_file(&to, "should be overwritten").unwrap();

    let result = fs.copy_file(&from, &to);

    assert!(result.is_ok());

    let result = fs.read_file(&to);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"expected");
}

fn copy_file_fails_if_original_file_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");

    let result = fs.copy_file(&from, &to);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
    assert!(!fs.is_file(&to));
}

fn copy_file_fails_if_destination_file_is_readonly<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");

    fs.create_file(&from, "test").unwrap();
    fs.create_file(&to, "").unwrap();
    fs.set_readonly(&to, true).unwrap();

    let result = fs.copy_file(&from, &to);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::PermissionDenied);
}

fn copy_file_fails_if_original_node_is_directory<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");

    fs.create_dir(&from).unwrap();

    let result = fs.copy_file(&from, &to);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
}

fn copy_file_fails_if_destination_node_is_directory<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");

    fs.create_file(&from, "").unwrap();
    fs.create_dir(&to).unwrap();

    let result = fs.copy_file(&from, &to);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);
}

fn rename_renames_a_file<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");

    fs.create_file(&from, "contents").unwrap();

    let result = fs.rename(&from, &to);

    assert!(result.is_ok());
    assert!(!fs.is_file(&from));

    let result = fs.read_file_to_string(&to);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "contents");
}

fn rename_renames_a_directory<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");
    let child = from.join("child");

    fs.create_dir(&from).unwrap();
    fs.create_file(&child, "child").unwrap();

    let result = fs.rename(&from, &to);

    assert!(result.is_ok());
    assert!(!fs.is_dir(&from));

    let result = fs.read_file_to_string(to.join("child"));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "child");
}

fn rename_overwrites_destination_file<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");

    fs.create_file(&from, "from").unwrap();
    fs.create_file(&to, "to").unwrap();

    let result = fs.rename(&from, &to);

    assert!(result.is_ok());
    assert!(!fs.is_file(&from));

    let result = fs.read_file_to_string(&to);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "from");
}

fn rename_overwrites_empty_destination_directory<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");
    let child = from.join("child");

    fs.create_dir(&from).unwrap();
    fs.create_dir(&to).unwrap();
    fs.create_file(&child, "child").unwrap();

    let result = fs.rename(&from, &to);

    assert!(result.is_ok(), "err: {:?}", result);
    assert!(!fs.is_dir(&from));

    let result = fs.read_file_to_string(to.join("child"));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "child");
}

fn rename_renames_all_descendants<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");
    let child_file = from.join("child_file");
    let child_dir = from.join("child_dir");
    let grandchild = child_dir.join("grandchild");

    fs.create_dir(&from).unwrap();
    fs.create_file(&child_file, "child_file").unwrap();
    fs.create_dir(&child_dir).unwrap();
    fs.create_file(&grandchild, "grandchild").unwrap();

    let result = fs.rename(&from, &to);

    assert!(result.is_ok());
    assert!(!fs.is_dir(&from));

    let result = fs.read_file_to_string(to.join("child_file"));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "child_file");

    let result = fs.read_file_to_string(to.join("child_dir").join("grandchild"));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "grandchild");
}

fn rename_fails_if_original_path_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");

    let result = fs.rename(&from, &to);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn rename_fails_if_original_and_destination_are_different_types<T: FileSystem>(
    fs: &T,
    parent: &Path,
) {
    let file = parent.join("file");
    let dir = parent.join("dir");

    fs.create_file(&file, "").unwrap();
    fs.create_dir(&dir).unwrap();

    let result = fs.rename(&file, &dir);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);

    let result = fs.rename(&dir, &file);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);
}

fn rename_fails_if_destination_directory_is_not_empty<T: FileSystem>(fs: &T, parent: &Path) {
    let from = parent.join("from");
    let to = parent.join("to");
    let child = to.join("child");

    fs.create_dir(&from).unwrap();
    fs.create_dir(&to).unwrap();
    fs.create_file(&child, "child").unwrap();

    let result = fs.rename(&from, &to);

    assert!(result.is_err());
}

fn readonly_returns_write_permission<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_file");

    fs.create_file(&path, "").unwrap();

    let result = fs.readonly(&path);

    assert!(result.is_ok());
    assert!(!result.unwrap());

    fs.set_readonly(&path, true).unwrap();

    let result = fs.readonly(&path);

    assert!(result.is_ok());
    assert!(result.unwrap());
}

fn readonly_fails_if_node_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let result = fs.readonly(parent.join("does_not_exist"));

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn set_readonly_toggles_write_permission_of_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_file");

    fs.create_file(&path, "").unwrap();

    let result = fs.set_readonly(&path, true);

    assert!(result.is_ok());
    assert!(fs.write_file(&path, "readonly").is_err());

    let result = fs.set_readonly(&path, false);

    assert!(result.is_ok());
    assert!(fs.write_file(&path, "no longer readonly").is_ok());
}

fn set_readonly_toggles_write_permission_of_dir<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("test_dir");

    fs.create_dir(&path).unwrap();

    let result = fs.set_readonly(&path, true);

    assert!(result.is_ok());
    assert!(fs.write_file(&path.join("file"), "").is_err());

    let result = fs.set_readonly(&path, false);

    assert!(result.is_ok());
    assert!(fs.write_file(&path.join("file"), "").is_ok());
}

fn set_readonly_fails_if_node_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let result = fs.set_readonly(parent.join("does_not_exist"), true);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);

    let result = fs.set_readonly(parent.join("does_not_exist"), true);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn len_returns_size_of_file<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("file");
    let result = fs.create_file(&path, "");

    assert!(result.is_ok());

    let len = fs.len(&path);

    assert_eq!(len, 0);

    let result = fs.write_file(&path, "contents");

    assert!(result.is_ok());

    let len = fs.len(&path);

    assert_eq!(len, 8);
}

fn len_returns_size_of_directory<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("directory");
    let result = fs.create_dir(&path);

    assert!(result.is_ok());

    let len = fs.len(&path);

    assert_ne!(len, 0);
}

fn len_returns_0_if_node_does_not_exist<T: FileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("does-not-exist");
    let len = fs.len(&path);

    assert_eq!(len, 0);
}

#[cfg(unix)]
fn mode_returns_permissions<T: FileSystem + UnixFileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("file");

    fs.create_file(&path, "").unwrap();
    fs.set_mode(&path, 0o644).unwrap();

    let result = fs.mode(&path);

    assert!(result.is_ok());
    assert_eq!(result.unwrap() % 0o100_000, 0o644);

    fs.set_mode(&path, 0o600).unwrap();

    let result = fs.mode(&path);

    assert!(result.is_ok());
    assert_eq!(result.unwrap() % 0o100_000, 0o600);

    fs.set_readonly(&path, true).unwrap();

    let result = fs.mode(&path);

    assert!(result.is_ok());
    assert_eq!(result.unwrap() % 0o100_000, 0o400);
}

#[cfg(unix)]
fn mode_fails_if_node_does_not_exist<T: UnixFileSystem>(fs: &T, parent: &Path) {
    let result = fs.mode(parent.join("does_not_exist"));

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

#[cfg(unix)]
fn set_mode_sets_permissions<T: FileSystem + UnixFileSystem>(fs: &T, parent: &Path) {
    let path = parent.join("file");

    fs.create_file(&path, "").unwrap();

    let result = fs.set_mode(&path, 0o000);

    assert!(result.is_ok());

    let readonly_result = fs.readonly(&path);

    assert!(readonly_result.is_ok());
    assert!(readonly_result.unwrap());

    let read_result = fs.read_file(&path);
    let write_result = fs.write_file(&path, "should not be allowed");

    assert!(read_result.is_err());
    assert!(write_result.is_err());
    assert_eq!(read_result.unwrap_err().kind(), ErrorKind::PermissionDenied);
    assert_eq!(
        write_result.unwrap_err().kind(),
        ErrorKind::PermissionDenied
    );

    let result = fs.set_mode(&path, 0o200);

    assert!(result.is_ok());

    let read_result = fs.read_file(&path);
    let write_result = fs.write_file(&path, "should be allowed");

    assert!(read_result.is_err());
    assert!(write_result.is_ok());
    assert_eq!(read_result.unwrap_err().kind(), ErrorKind::PermissionDenied);

    let readonly_result = fs.readonly(&path);

    assert!(readonly_result.is_ok());
    assert!(!readonly_result.unwrap());

    let result = fs.set_mode(&path, 0o644);

    assert!(result.is_ok());

    let readonly_result = fs.readonly(&path);

    assert!(readonly_result.is_ok());
    assert!(!readonly_result.unwrap());
}

#[cfg(unix)]
fn set_mode_fails_if_node_does_not_exist<T: UnixFileSystem>(fs: &T, parent: &Path) {
    let result = fs.set_mode(parent.join("does_not_exist"), 0o644);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

fn temp_dir_creates_tempdir<T: FileSystem + TempFileSystem>(fs: &T, _: &Path) {
    let path = {
        let result = fs.temp_dir("test");

        assert!(result.is_ok());

        let temp_dir = result.unwrap();

        assert!(fs.is_dir(temp_dir.path()));

        temp_dir.path().to_path_buf()
    };

    assert!(!fs.is_dir(&path));
    assert!(fs.is_dir(path.parent().unwrap()));
}

fn temp_dir_creates_unique_dir<T: FileSystem + TempFileSystem>(fs: &T, _: &Path) {
    let first = fs.temp_dir("test").unwrap();
    let second = fs.temp_dir("test").unwrap();

    assert_ne!(first.path(), second.path());
}
