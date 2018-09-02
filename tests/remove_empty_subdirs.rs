extern crate tempfile;
extern crate walkdir;

extern crate remove_empty_subdirs;

use std::fs::{self, create_dir_all, File};

use tempfile::tempdir;
use walkdir::WalkDir;

use remove_empty_subdirs::remove_empty_subdirs;

#[test]
fn normal() {
    // Prepare temporary directory.
    let dir = tempdir().unwrap();

    // Prepare sub-directories.
    let dir_1 = dir.path().join("empty_1").join("empty_2").join("empty_3");
    create_dir_all(dir_1).unwrap();
    let dir_2 = dir.path().join("empty_1").join("empty_21").join("empty_31");
    create_dir_all(dir_2).unwrap();
    let dir_3 = dir
        .path()
        .join("not_empty_1")
        .join("not_empty_21")
        .join("empty_31");
    create_dir_all(dir_3).unwrap();

    // Prepare files.
    let file_path = dir
        .path()
        .join("not_empty_1")
        .join("not_empty_21")
        .join("new-file.txt");
    let file = File::create(file_path).unwrap();
    drop(file);

    // Assert all sub-directories and files are created.
    let mut subpaths: Vec<String> = WalkDir::new(&dir)
        .into_iter()
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let mut expected_subpaths: Vec<String> = [
        dir.path().to_path_buf(),
        dir.path().join("empty_1"),
        dir.path().join("empty_1").join("empty_2"),
        dir.path().join("empty_1").join("empty_2").join("empty_3"),
        dir.path().join("empty_1").join("empty_21"),
        dir.path().join("empty_1").join("empty_21").join("empty_31"),
        dir.path().join("not_empty_1"),
        dir.path().join("not_empty_1").join("not_empty_21"),
        dir.path()
            .join("not_empty_1")
            .join("not_empty_21")
            .join("empty_31"),
        dir.path()
            .join("not_empty_1")
            .join("not_empty_21")
            .join("new-file.txt"),
    ].into_iter()
        .map(|entry| entry.to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    subpaths.sort_unstable();
    expected_subpaths.sort_unstable();
    assert_eq!(subpaths, expected_subpaths);

    // Remove empty sub-directories.
    remove_empty_subdirs(dir.path()).unwrap();

    // Assert all empty sub-directories are removed.
    let mut subpaths: Vec<String> = WalkDir::new(&dir)
        .into_iter()
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let mut expected_subpaths: Vec<String> = [
        dir.path().to_path_buf(),
        dir.path().join("not_empty_1"),
        dir.path().join("not_empty_1").join("not_empty_21"),
        dir.path()
            .join("not_empty_1")
            .join("not_empty_21")
            .join("new-file.txt"),
    ].into_iter()
        .map(|entry| entry.to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    subpaths.sort_unstable();
    expected_subpaths.sort_unstable();
    assert_eq!(subpaths, expected_subpaths);

    dir.close().unwrap();
}

#[test]
fn hidden_dir() {
    // Prepare temporary directory.
    let dir = tempdir().unwrap();

    // Prepare sub-directories.
    let dir_1 = dir.path().join("not_empty").join(".git");
    create_dir_all(dir_1).unwrap();
    let dir_2 = dir.path().join("empty_1").join("empty_2");
    create_dir_all(dir_2).unwrap();

    // Assert all sub-directories are created.
    let mut subpaths: Vec<String> = WalkDir::new(&dir)
        .into_iter()
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let mut expected_subpaths: Vec<String> = [
        dir.path().to_path_buf(),
        dir.path().join("not_empty"),
        dir.path().join("not_empty").join(".git"),
        dir.path().join("empty_1"),
        dir.path().join("empty_1").join("empty_2"),
    ].into_iter()
        .map(|entry| entry.to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    subpaths.sort_unstable();
    expected_subpaths.sort_unstable();
    assert_eq!(subpaths, expected_subpaths);

    // Remove empty sub-directories.
    remove_empty_subdirs(dir.path()).unwrap();

    // Assert all empty sub-directories are removed.
    let mut subpaths: Vec<String> = WalkDir::new(&dir)
        .into_iter()
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let mut expected_subpaths: Vec<String> = [
        dir.path().to_path_buf(),
        dir.path().join("not_empty"),
        dir.path().join("not_empty").join(".git"),
    ].into_iter()
        .map(|entry| entry.to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    subpaths.sort_unstable();
    expected_subpaths.sort_unstable();
    assert_eq!(subpaths, expected_subpaths);

    // Closes and removes the temporary directory.
    dir.close().unwrap();
}

#[test]
fn nothing() {
    // Prepare temporary directory.
    let dir = tempdir().unwrap();

    // Assert nothing in the temporary directory.
    let mut subpaths: Vec<String> = WalkDir::new(&dir)
        .into_iter()
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let mut expected_subpaths: Vec<String> = [dir.path().to_path_buf()]
        .into_iter()
        .map(|entry| entry.to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    subpaths.sort_unstable();
    expected_subpaths.sort_unstable();
    assert_eq!(subpaths, expected_subpaths);

    // Remove empty sub-directories.
    remove_empty_subdirs(dir.path()).unwrap();

    // Assert still nothing in the temporary directory.
    let mut subpaths: Vec<String> = WalkDir::new(&dir)
        .into_iter()
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let mut expected_subpaths: Vec<String> = [dir.path().to_path_buf()]
        .into_iter()
        .map(|entry| entry.to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    subpaths.sort_unstable();
    expected_subpaths.sort_unstable();
    assert_eq!(subpaths, expected_subpaths);

    // Closes and removes the temporary directory.
    dir.close().unwrap();
}

#[cfg(unix)]
#[test]
fn permission_denied_dir() {
    // std::fs::Permissions::set_mode is only supported in Unix.
    use std::os::unix::fs::PermissionsExt;

    // Prepare temporary directory.
    let dir = tempdir().unwrap();

    // Prepare sub-directories.
    let dir_1 = dir
        .path()
        .join("normal_dir")
        .join("permission_denied")
        .join("permission_denied_1")
        .join("empty_3");
    create_dir_all(dir_1).unwrap();
    let dir_2 = dir
        .path()
        .join("normal_dir")
        .join("empty_1")
        .join("empty_2");
    create_dir_all(dir_2).unwrap();

    // Assert all sub-directories are created.
    let mut subpaths: Vec<String> = WalkDir::new(&dir)
        .into_iter()
        .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let mut expected_subpaths: Vec<String> = [
        dir.path().to_path_buf(),
        dir.path().join("normal_dir"),
        dir.path().join("normal_dir").join("permission_denied"),
        dir.path()
            .join("normal_dir")
            .join("permission_denied")
            .join("permission_denied_1"),
        dir.path()
            .join("normal_dir")
            .join("permission_denied")
            .join("permission_denied_1")
            .join("empty_3"),
        dir.path().join("normal_dir").join("empty_1"),
        dir.path()
            .join("normal_dir")
            .join("empty_1")
            .join("empty_2"),
    ].into_iter()
        .map(|entry| entry.to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    subpaths.sort_unstable();
    expected_subpaths.sort_unstable();
    assert_eq!(subpaths, expected_subpaths);

    // Change directory to permission denied.
    let dir_permission_denied_1 = dir
        .path()
        .join("normal_dir")
        .join("permission_denied")
        .join("permission_denied_1");
    let mut permissions = dir_permission_denied_1.metadata().unwrap().permissions();
    permissions.set_mode(0o40644);
    fs::set_permissions(&dir_permission_denied_1, permissions).unwrap();

    // Assert directory permission denied.
    let dir_permission_denied_1_clone = dir_permission_denied_1.clone();
    let permissions = dir_permission_denied_1_clone
        .metadata()
        .unwrap()
        .permissions();
    assert_eq!(permissions.mode(), 0o40644);

    // Remove empty sub-directories.
    remove_empty_subdirs(dir.path()).unwrap();

    // Assert all empty sub-directories are removed.
    let mut subpaths: Vec<String> = WalkDir::new(&dir)
        .into_iter()
        .filter_map(|err| err.ok())
        .map(|entry| entry.path().to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let mut expected_subpaths: Vec<String> = [
        dir.path().to_path_buf(),
        dir.path().join("normal_dir"),
        dir.path().join("normal_dir").join("permission_denied"),
        dir.path()
            .join("normal_dir")
            .join("permission_denied")
            .join("permission_denied_1"),
        dir.path()
            .join("normal_dir")
            .join("permission_denied")
            .join("permission_denied_1")
            .join("empty_3"),
    ].into_iter()
        .map(|entry| entry.to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    subpaths.sort_unstable();
    expected_subpaths.sort_unstable();
    assert_eq!(subpaths, expected_subpaths);

    // Recover directory from permission denied.
    let dir_permission_denied_1_clone = dir_permission_denied_1.clone();
    let mut permissions = dir_permission_denied_1_clone
        .metadata()
        .unwrap()
        .permissions();
    permissions.set_mode(0o40755);
    fs::set_permissions(&dir_permission_denied_1, permissions).unwrap();

    // Closes and removes the temporary directory.
    dir.close().unwrap();
}
