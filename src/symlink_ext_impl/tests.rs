use crate::file_ext_impl::FileExtImpl;
use crate::FileExt;
use crate::path_ext_impl::PathExtImpl;
use crate::symbol::SYMBOL;
use crate::symlink_ext_impl::SymlinkExtImpl;

#[test]
fn symlink_check() {
    let path = ["test", "index_rewrite"].join(PathExtImpl::get_path_separator().as_str());
    create_rewrite_index_symlink();

    let is_symlink = SymlinkExtImpl::is_symlink(path.as_str()).unwrap();
    assert!(is_symlink);

    FileExtImpl::delete_file(path.as_str()).unwrap();
}

#[test]
fn not_symlink_check() {
    let path = "test/index.html";
    let is_symlink = SymlinkExtImpl::is_symlink(path).unwrap();
    assert!(!is_symlink);
}

#[test]
fn file_exists() {
    let path = ["test", "index_rewrite"].join(PathExtImpl::get_path_separator().as_str());
    create_rewrite_index_symlink();

    let exists = SymlinkExtImpl::does_symlink_exist(path.as_str());
    assert!(exists);

    FileExtImpl::delete_file(path.as_str()).unwrap();

}

#[test]
fn symlink_creation() {
    let symlink_path = ["test", "index-link"].join(FileExt::get_path_separator().as_str());

    if SymlinkExtImpl::does_symlink_exist(symlink_path.as_str()) {
        FileExtImpl::delete_file(symlink_path.as_str()).unwrap();
    }

    let path = [SYMBOL.empty_string, "test", SYMBOL.empty_string].join(PathExtImpl::get_path_separator().as_str());
    let path_prefix = FileExt::get_static_filepath(path.as_str()).unwrap();
    let points_to = [path_prefix.to_string(), "index.html".to_string()].join("");

    let boxed_symlink = SymlinkExtImpl::create_symlink(
        path_prefix.as_str(),
        "index-link",
        points_to.as_str());


    assert!(boxed_symlink.is_ok());

    let symlink_created = SymlinkExtImpl::does_symlink_exist(symlink_path.as_str());
    assert!(symlink_created);

    let actual_points_to = SymlinkExtImpl::symlink_points_to(symlink_path.as_str()).unwrap();
    assert_eq!(points_to, actual_points_to);

    FileExtImpl::delete_file(symlink_path.as_str()).unwrap();
}

#[test]
fn link_points_to() {
    let symlink_path = ["test", "index_rewrite2"].join(PathExtImpl::get_path_separator().as_str());

    if SymlinkExtImpl::does_symlink_exist(symlink_path.as_str()) {
        FileExtImpl::delete_file(symlink_path.as_str()).unwrap();
    }

    let path = [SYMBOL.empty_string, "test",  "index.html"].join(FileExt::get_path_separator().as_str());
    let points_to = FileExt::get_static_filepath(path.as_str()).unwrap();

    let symlink_dir = [SYMBOL.empty_string, "test", SYMBOL.empty_string].join(FileExt::get_path_separator().as_str());
    let path_prefix = FileExt::get_static_filepath(symlink_dir.as_str()).unwrap();

    let boxed_symlink = SymlinkExtImpl::create_symlink(
        path_prefix.as_str(),
        "index_rewrite2",
        points_to.as_str());


    assert!(boxed_symlink.is_ok());

    let symlink_created = SymlinkExtImpl::does_symlink_exist(symlink_path.as_str());
    assert!(symlink_created);

    let actual_points_to = SymlinkExtImpl::symlink_points_to(symlink_path.as_str()).unwrap();
    assert_eq!(points_to, actual_points_to);

    FileExtImpl::delete_file(symlink_path.as_str()).unwrap();

}

fn create_rewrite_index_symlink() {
    let symlink_path = ["test", "index_rewrite"].join(PathExtImpl::get_path_separator().as_str());

    if SymlinkExtImpl::does_symlink_exist(symlink_path.as_str()) {
        FileExtImpl::delete_file(symlink_path.as_str()).unwrap();
    }

    let path = [SYMBOL.empty_string, "test", SYMBOL.empty_string].join(PathExtImpl::get_path_separator().as_str());
    let path_prefix = FileExt::get_static_filepath(path.as_str()).unwrap();
    let points_to = [path_prefix.to_string(), "index.html".to_string()].join("");

    let boxed_symlink = SymlinkExtImpl::create_symlink(
        path_prefix.as_str(),
        "index_rewrite",
        points_to.as_str());


    assert!(boxed_symlink.is_ok());

    let symlink_created = SymlinkExtImpl::does_symlink_exist(symlink_path.as_str());
    assert!(symlink_created);
}