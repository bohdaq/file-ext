use crate::directory_ext_impl::DirectoryExtImpl;
use crate::FileExt;
use crate::path_ext_impl::PathExtImpl;

#[test]
fn new_directory_recursively_create_delete() {
    let path = ["directory", "subdirectory"].join(PathExtImpl::get_path_separator().as_str());

    if DirectoryExtImpl::does_directory_exist("directory") {
        FileExt::delete_directory("directory").unwrap();
    }

    let boxed_create = DirectoryExtImpl::create_directory(path.as_str());
    assert!(boxed_create.is_ok());

    assert!(DirectoryExtImpl::does_directory_exist(path.as_str()));

    let boxed_delete = DirectoryExtImpl::delete_directory("directory");
    assert!(boxed_delete.is_ok());

}

#[test]
fn new_directory_create_recursively_additional_subdirectory() {
    let path = ["recursive_directory_creation", "subdirectory", "subsubdirectory"].join(PathExtImpl::get_path_separator().as_str());

    if FileExt::does_directory_exist("recursive_directory_creation") {
        FileExt::delete_directory("recursive_directory_creation").unwrap();
    }

    let boxed_create = DirectoryExtImpl::create_directory(path.as_str());
    assert!(boxed_create.is_ok());

    assert!(DirectoryExtImpl::does_directory_exist(path.as_str()));

    let boxed_delete = DirectoryExtImpl::delete_directory("recursive_directory_creation");
    assert!(boxed_delete.is_ok());
}

#[test]
fn new_directory_create_non_recursively() {
    let path = "dir".to_string();

    if DirectoryExtImpl::does_directory_exist(path.as_str()) {
        DirectoryExtImpl::delete_directory(path.as_str()).unwrap();
    }

    let boxed_create = DirectoryExtImpl::create_directory(path.as_str());
    assert!(boxed_create.is_ok());

    assert!(DirectoryExtImpl::does_directory_exist(path.as_str()));

    let boxed_delete = DirectoryExtImpl::delete_directory(path.as_str());
    assert!(boxed_delete.is_ok());
}

#[test]
fn new_directory_create_recursively_additional_subdirectory_removal() {
    let path = ["recursive_directory", "subdirectory", "subsubdirectory"].join(PathExtImpl::get_path_separator().as_str());

    if FileExt::does_directory_exist("recursive_directory") {
        FileExt::delete_directory("recursive_directory").unwrap();
    }

    let boxed_create = DirectoryExtImpl::create_directory(path.as_str());
    assert!(boxed_create.is_ok());

    assert!(DirectoryExtImpl::does_directory_exist(path.as_str()));

    let boxed_delete = DirectoryExtImpl::delete_directory(path.as_str());
    assert!(boxed_delete.is_ok());

    assert!(!DirectoryExtImpl::does_directory_exist(path.as_str()));

    let boxed_delete = DirectoryExtImpl::delete_directory("recursive_directory");
    assert!(boxed_delete.is_ok());

    assert!(!DirectoryExtImpl::does_directory_exist("recursive_directory"));
}
