use std::{thread, time};
use crate::FileExt;
use crate::symbol::{SYMBOL};



#[test]
fn symlink_check() {
    let path = ["test", "index_rewrite"].join(FileExt::get_path_separator().as_str());
    create_rewrite_index_symlink();

    let is_symlink = FileExt::is_symlink(path.as_str()).unwrap();
    assert!(is_symlink);

    FileExt::delete_file(path.as_str()).unwrap();
}

#[test]
fn not_symlink_check() {
    let path = "test/index.html";
    let is_symlink = FileExt::is_symlink(path).unwrap();
    assert!(!is_symlink);
}





#[test]
fn file_exists() {
    let path = ["test", "index_rewrite"].join(FileExt::get_path_separator().as_str());
    create_rewrite_index_symlink();

    let exists = FileExt::does_symlink_exist(path.as_str());
    assert!(exists);

    FileExt::delete_file(path.as_str()).unwrap();

}



#[test]
fn modification_timestamp() {

    let content = "data".as_bytes();
    let path = "modification_timestamp-test.content";

    FileExt::create_file(path).unwrap();
    FileExt::write_file(path, content).unwrap();

    let does_exist = FileExt::does_file_exist(path);
    assert!(does_exist);

    let modified_timestamp = FileExt::file_modified_utc(path).unwrap();

    let one_second = time::Duration::from_secs(1);
    thread::sleep(one_second);

    FileExt::write_file(path, "\nnewline and some data".as_bytes()).unwrap();

    let after_update_modified_timestamp = FileExt::file_modified_utc(path).unwrap();
    assert!(after_update_modified_timestamp > modified_timestamp);


    FileExt::delete_file(path).unwrap();
    let doesnt_exist = !FileExt::does_file_exist(path);
    assert!(doesnt_exist);
}

#[test]
fn symlink_creation() {
    let symlink_path = ["test", "index-link"].join(FileExt::get_path_separator().as_str());

    if FileExt::does_symlink_exist(symlink_path.as_str()) {
        FileExt::delete_file(symlink_path.as_str()).unwrap();
    }

    let path = [SYMBOL.empty_string, "test", SYMBOL.empty_string].join(FileExt::get_path_separator().as_str());
    let path_prefix = FileExt::get_static_filepath(path.as_str()).unwrap();
    let points_to = [path_prefix.to_string(), "index.html".to_string()].join("");

    let boxed_symlink = FileExt::create_symlink(
        path_prefix.as_str(),
        "index-link",
        points_to.as_str());


    assert!(boxed_symlink.is_ok());

    let symlink_created = FileExt::does_symlink_exist(symlink_path.as_str());
    assert!(symlink_created);

    let actual_points_to = FileExt::symlink_points_to(symlink_path.as_str()).unwrap();
    assert_eq!(points_to, actual_points_to);

    FileExt::delete_file(symlink_path.as_str()).unwrap();
}

#[test]
fn link_points_to() {
    let symlink_path = ["test", "index_rewrite2"].join(FileExt::get_path_separator().as_str());

    if FileExt::does_symlink_exist(symlink_path.as_str()) {
        FileExt::delete_file(symlink_path.as_str()).unwrap();
    }

    let path = [SYMBOL.empty_string, "test",  "index.html"].join(FileExt::get_path_separator().as_str());
    let points_to = FileExt::get_static_filepath(path.as_str()).unwrap();

    let symlink_dir = [SYMBOL.empty_string, "test", SYMBOL.empty_string].join(FileExt::get_path_separator().as_str());
    let path_prefix = FileExt::get_static_filepath(symlink_dir.as_str()).unwrap();

    let boxed_symlink = FileExt::create_symlink(
        path_prefix.as_str(),
        "index_rewrite2",
        points_to.as_str());


    assert!(boxed_symlink.is_ok());

    let symlink_created = FileExt::does_symlink_exist(symlink_path.as_str());
    assert!(symlink_created);

    let actual_points_to = FileExt::symlink_points_to(symlink_path.as_str()).unwrap();
    assert_eq!(points_to, actual_points_to);

    FileExt::delete_file(symlink_path.as_str()).unwrap();

}

fn create_rewrite_index_symlink() {
    let symlink_path = ["test", "index_rewrite"].join(FileExt::get_path_separator().as_str());

    if FileExt::does_symlink_exist(symlink_path.as_str()) {
        FileExt::delete_file(symlink_path.as_str()).unwrap();
    }

    let path = [SYMBOL.empty_string, "test", SYMBOL.empty_string].join(FileExt::get_path_separator().as_str());
    let path_prefix = FileExt::get_static_filepath(path.as_str()).unwrap();
    let points_to = [path_prefix.to_string(), "index.html".to_string()].join("");

    let boxed_symlink = FileExt::create_symlink(
        path_prefix.as_str(),
        "index_rewrite",
        points_to.as_str());


    assert!(boxed_symlink.is_ok());

    let symlink_created = FileExt::does_symlink_exist(symlink_path.as_str());
    assert!(symlink_created);
}

#[test]
fn current_user() {
    let boxed_user = FileExt::get_current_user();
    assert!(boxed_user.is_ok());

    let path = "current-user.log";

    FileExt::create_file(path).unwrap();
    FileExt::write_file(path, boxed_user.unwrap().as_bytes()).unwrap();

    FileExt::delete_file(path).unwrap();
}

#[test]
#[cfg(target_family = "windows")]
fn current_user_domain() {
    let boxed_user_domain = FileExt::get_current_user_domain();
    assert!(boxed_user_domain.is_ok());

    let path = "current-user-domain.log";

    FileExt::create_file(path).unwrap();
    FileExt::write_file(path, boxed_user_domain.unwrap().as_bytes()).unwrap();
}

#[test]
#[cfg(target_family = "windows")]
fn temp_folder() {
    let temp_folder_path = FileExt::get_temp_folder_path().unwrap();
    assert!(temp_folder_path.starts_with("C:\\Users\\"));
    assert!(temp_folder_path.ends_with("\\AppData\\Local\\Temp"));
}

#[test]
#[cfg(target_family = "unix")]
fn temp_folder() {
    let temp_folder_path = FileExt::get_temp_folder_path().unwrap();
    assert_eq!(temp_folder_path, "/tmp")
}

#[test]
fn absolute_path_to_working_directory() {
    let boxed_path = FileExt::get_static_filepath(FileExt::get_path_separator().as_str());
    assert!(boxed_path.is_ok());
}


