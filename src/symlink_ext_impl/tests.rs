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

#[test]
fn resolve_symlink_path() {
    let base_dir = "/home/someuser/folder/subfolder/subsubfolder";
    let symlink_points_to = "../../subfolder2/subsubfolder2";

    let expected_path = "/home/someuser/folder/subfolder2/subsubfolder2";
    let actual_path = SymlinkExtImpl::resolve_symlink_path(base_dir, symlink_points_to).unwrap();

    assert_eq!(expected_path, actual_path);
}

#[test]
fn resolve_symlink_back_and_forth() {
    let base_dir = "/home/someuser/folder/subfolder/subsubfolder";
    let symlink_points_to = "../../subfolder2/subsubfolder2/../../subfolder/subsubfolder";

    let expected_path = "/home/someuser/folder/subfolder/subsubfolder";
    let actual_path = SymlinkExtImpl::resolve_symlink_path(base_dir, symlink_points_to).unwrap();

    assert_eq!(expected_path, actual_path);
}

#[test]
fn resolve_symlink_back_and_forth_starts_from_subdir() {
    let base_dir = "/home/someuser/folder/subfolder";
    let symlink_points_to = "subsubfolder/../../subfolder2/subsubfolder2/../../subfolder/subsubfolder";

    let expected_path = "/home/someuser/folder/subfolder/subsubfolder";
    let actual_path = SymlinkExtImpl::resolve_symlink_path(base_dir, symlink_points_to).unwrap();

    assert_eq!(expected_path, actual_path);
}

#[test]
fn resolve_symlink_path_subdirectory() {
    let base_dir = "/home/someuser/folder/subfolder/subsubfolder";
    let symlink_points_to = "subsubsubfolder/subsubsubsubfolder";

    let expected_path = "/home/someuser/folder/subfolder/subsubfolder/subsubsubfolder/subsubsubsubfolder";
    let actual_path = SymlinkExtImpl::resolve_symlink_path(base_dir, symlink_points_to).unwrap();

    assert_eq!(expected_path, actual_path);
}

#[test]
fn resolve_symlink_path_root() {
    let base_dir = "/home/someuser/folder/subfolder/subsubfolder";
    let symlink_points_to = "/tmp/folder";

    let expected_path = "/tmp/folder";
    let actual_path = SymlinkExtImpl::resolve_symlink_path(base_dir, symlink_points_to).unwrap();

    assert_eq!(expected_path, actual_path);
}

#[test]
fn resolve_symlink_path_not_valid() {
    let base_dir = "/home/someuser";
    let symlink_points_to = "../../../tmp/folder";

    let boxed_resolve = SymlinkExtImpl::resolve_symlink_path(base_dir, symlink_points_to);
    let is_err = boxed_resolve.is_err();

    assert!(is_err);

    let expected_error = "not valid path for the symlink";
    let actual_error = boxed_resolve.err().unwrap();
    assert_eq!(expected_error, actual_error);
}
