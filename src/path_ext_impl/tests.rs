use crate::path_ext_impl::PathExtImpl;

#[test]
#[cfg(target_family = "windows")]
fn temp_folder() {
    let temp_folder_path = PathExtImpl::get_temp_folder_path().unwrap();
    assert!(temp_folder_path.starts_with("C:\\Users\\"));
    assert!(temp_folder_path.ends_with("\\AppData\\Local\\Temp"));
}

#[test]
#[cfg(target_family = "unix")]
fn temp_folder() {
    let temp_folder_path = PathExtImpl::get_temp_folder_path().unwrap();
    assert_eq!(temp_folder_path, "/tmp")
}

#[test]
#[cfg(target_family = "unix")]
fn build_path() {
    let root = PathExtImpl::root();
    let folder_up = PathExtImpl::folder_up();

    let node_list =
        [
            root.as_str(),
            "home",
            "someuser",
            "folder",
            "subfolder",
            "subsubfolder",
        ];

    let another_node_list =
        [
            folder_up.as_str(),
            folder_up.as_str(),
            "subfolder2",
            "subsubfolder2",
        ];
    let path = PathExtImpl::build_path(&node_list);
    let another_path = PathExtImpl::build_path(&another_node_list);

    assert_eq!("/home/someuser/folder/subfolder/subsubfolder", path);
    assert_eq!("../../subfolder2/subsubfolder2", another_path);
}


#[test]
#[cfg(target_family = "windows")]
fn build_path() {
    let root = PathExtImpl::root();

    let node_list =
        [
            root.as_str(),
            "Users",
            "someuser",
            "folder",
            "subfolder",
            "subsubfolder",
        ];

    let path = PathExtImpl::build_path(&node_list);

    assert_eq!("C:\\Users\\someuser\\folder\\subfolder\\subsubfolder", path);
}

#[test]
fn absolute_path_to_working_directory() {
    let boxed_path = PathExtImpl::absolute_path_to_working_directory();
    assert!(boxed_path.is_ok());
    let path = boxed_path.unwrap();
}
