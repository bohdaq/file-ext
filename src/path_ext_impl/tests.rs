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
