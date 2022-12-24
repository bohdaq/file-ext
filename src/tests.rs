use crate::FileExt;

#[test]
fn symlink_check() {
    let path = "test/index_rewrite";
    let is_symlink = FileExt::is_symlink(path).unwrap();
    assert!(is_symlink);
}

#[test]
fn not_symlink_check() {
    let path = "test/index.html";
    let is_symlink = FileExt::is_symlink(path).unwrap();
    assert!(!is_symlink);
}

#[test]
fn link_points_to() {
    let path = "test/index_rewrite";
    let points_to = FileExt::symlink_points_to(path).unwrap();
    assert_eq!("index.html", points_to);
}

#[test]
fn does_not_exist() {
    let path = "test/non_existing_file";
    let exists = FileExt::does_file_exist(path);
    assert!(!exists);
}

#[test]
fn file_exists() {
    let path = "test/index_rewrite";
    let exists = FileExt::does_file_exist(path);
    assert!(exists);
}