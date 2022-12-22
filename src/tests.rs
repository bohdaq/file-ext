use std::fs;
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
    let path_buff = fs::read_link(path).unwrap();
    let points_to = path_buff.as_path().to_str().unwrap();
    assert_eq!("index.html", points_to);
}