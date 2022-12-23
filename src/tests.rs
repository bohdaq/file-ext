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