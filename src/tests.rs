use std::{thread, time};
use crate::FileExt;
use crate::path_ext_impl::PathExtImpl;
use crate::symbol::{SYMBOL};

#[test]
fn write() {
    let filename = "write-test.content";
    FileExt::create_file(filename).unwrap();

    let expected_content = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <title>Title</title>\n</head>\n<body>\n\n</body>\n</html>";
    FileExt::write_file(filename, expected_content.as_bytes()).unwrap();

    let actual = FileExt::read_file(filename).unwrap();
    assert_eq!(actual, expected_content.as_bytes());

}

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
fn file_content() {
    let path = "test/index.html";
    let file_raw_bytes = FileExt::read_file(path).unwrap();
    let content = String::from_utf8(file_raw_bytes).unwrap();

    let content_escaped_newline_carriage_return = str::replace(content.as_str(), "\r\n", "\n");

    let expected_content = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <title>Title</title>\n</head>\n<body>\n\n</body>\n</html>";

    assert_eq!(expected_content, content_escaped_newline_carriage_return);
}

#[test]
fn partial_read() {
    let path = "test/index.html";
    let file_raw_bytes = FileExt::read_file_partially(path, 4, 10).unwrap();
    let content = String::from_utf8(file_raw_bytes).unwrap();

    let expected_content = "CTYPE h";

    assert_eq!(expected_content, content);
}

#[test]
fn does_not_exist() {
    let path = "test/non_existing_file";
    let exists = FileExt::does_file_exist(path);
    assert!(!exists);
}

#[test]
fn file_exists() {
    let path = ["test", "index_rewrite"].join(FileExt::get_path_separator().as_str());
    create_rewrite_index_symlink();

    let exists = FileExt::does_file_exist(path.as_str());
    assert!(exists);

    FileExt::delete_file(path.as_str()).unwrap();

}

#[test]
fn file_creation_deletion() {
    let path = "test/file-creation.txt";

    let exists = FileExt::does_file_exist(path);
    assert!(!exists);

    FileExt::create_file(path).unwrap();

    let content = FileExt::read_file(path).unwrap();
    assert_eq!(content.len(), 0);

    FileExt::delete_file(path).unwrap();

    let exists = FileExt::does_file_exist(path);
    assert!(!exists);
}

#[test]
fn read_or_create_and_write() {
    let content = "data".as_bytes();
    let tmp_folder = FileExt::get_temp_folder_path().unwrap();

    let path = [tmp_folder, "test.txt".to_string()].join(FileExt::get_path_separator().as_str());

    let doesnt_exist = !FileExt::does_file_exist(path.as_str());
    assert!(doesnt_exist);

    FileExt::read_or_create_and_write(path.as_str(), content).unwrap();

    let does_exist = FileExt::does_file_exist(path.as_str());
    assert!(does_exist);

    let new_content = "updated data".as_bytes();
    FileExt::read_or_create_and_write(path.as_str(), new_content).unwrap();

    let file_content = FileExt::read_file(path.as_str()).unwrap();
    assert_eq!(content, file_content);

    FileExt::delete_file(path.as_str()).unwrap();
    let doesnt_exist = !FileExt::does_file_exist(path.as_str());
    assert!(doesnt_exist);
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

    //let path = "out.log";
    //FileExt::create_file(path).unwrap();
    //FileExt::write_file(path, format!("\n\nsymlink_dir: {}", symlink_dir).as_bytes()).unwrap();
    //FileExt::write_file(path, format!("\npath_prefix: {}", path_prefix).as_bytes()).unwrap();
    //FileExt::write_file(path, format!("\npoints_to: {}", points_to).as_bytes()).unwrap();


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

//#[test]
fn new_directory_create_delete() {
    let path = "new_directory";

    let boxed_create = FileExt::create_directory(path);
    assert!(boxed_create.is_ok());

    assert!(FileExt::does_directory_exist(path));

    let boxed_delete = FileExt::delete_directory(path);
    assert!(boxed_delete.is_ok());
}

//#[test]
fn new_directory_create_recursively_delete() {
    let path = ["directory", "subdirectory"].join(PathExtImpl::get_path_separator().as_str());

    if FileExt::does_directory_exist(path.as_str()) {
        FileExt::delete_directory(path.as_str()).unwrap();
    }

    let name = "new_directory_create_recursively_delete.log";
    FileExt::create_file(name).unwrap();

    let boxed_create = recursive_call("", path.as_str(),name);
    assert!(boxed_create.is_ok());

    // assert!(FileExt::does_directory_exist(path.as_str()));

    // let boxed_delete = FileExt::delete_directory(path.as_str());
    // assert!(boxed_delete.is_ok());
}

//#[test]
fn new_directory_create_recursively() {
    let path = ["recursive_directory_creation", "subdirectory", "subsubdirectory"].join(PathExtImpl::get_path_separator().as_str());

    if FileExt::does_directory_exist(path.as_str()) {
        FileExt::delete_directory(path.as_str()).unwrap();
    }

    let name = "new_directory_create_recursively.log";
    FileExt::create_file(name).unwrap();

    let boxed_create = recursive_call("", path.as_str(), name);
    assert!(boxed_create.is_ok());

    // assert!(FileExt::does_directory_exist(path.as_str()));

    // let boxed_delete = FileExt::delete_directory(path.as_str());
    // assert!(boxed_delete.is_ok());
}

//#[test]
fn new_directory_create_non_recursively() {
    let path = "dir".to_string();

    if FileExt::does_directory_exist(path.as_str()) {
       FileExt::delete_directory(path.as_str()).unwrap();
    }

    let name = "new_directory_create_non_recursively.log";
    FileExt::create_file(name).unwrap();

    let boxed_create = recursive_call("", path.as_str(), name);
    assert!(boxed_create.is_ok());

    // assert!(FileExt::does_directory_exist(path.as_str()));

    // let boxed_delete = FileExt::delete_directory(path.as_str());
    // assert!(boxed_delete.is_ok());
}

fn recursive_call(processed_path: &str, remaining_path: &str, log_filename: &str) -> Result<(), String> {
    let name = log_filename;
    FileExt::write_file(name, format!("\n\nprocessed path: {}", processed_path).as_bytes()).unwrap();
    FileExt::write_file(name, format!("\nremaining path: {}", remaining_path).as_bytes()).unwrap();

    let boxed_split = remaining_path.split_once(PathExtImpl::get_path_separator().as_str());
    if boxed_split.is_none() {
        let mut folder_path = remaining_path.to_string();
        if processed_path.chars().count() != 0 {
            folder_path = [processed_path, remaining_path].join(FileExt::get_path_separator().as_str());
        }

        FileExt::write_file(name, format!("\nfolder path: {}", folder_path).as_bytes()).unwrap();
        FileExt::write_file(name, format!("\nremaining path: {}", remaining_path).as_bytes()).unwrap();

        let boxed_create_folder = FileExt::create_directory(folder_path.as_str());
        if boxed_create_folder.is_err() {
            let message = boxed_create_folder.err().unwrap();
            return Err(message)
        }

        return Ok(());
    }
    let (folder, remaining_path) = boxed_split.unwrap();

    let mut  folder_path = folder.to_string();
    if processed_path.chars().count() != 0 {
        folder_path = [processed_path, folder].join(FileExt::get_path_separator().as_str());
    }

    FileExt::write_file(name, format!("\nfolder path: {}", folder_path).as_bytes()).unwrap();
    FileExt::write_file(name, format!("\nremaining path: {}", remaining_path).as_bytes()).unwrap();

    let boxed_create_folder = FileExt::create_directory(folder_path.as_str());
    if boxed_create_folder.is_err() {
        let message = boxed_create_folder.err().unwrap();
        return Err(message)
    }
    let mut _processed_path = folder.to_string();
    if processed_path.chars().count() != 0 {
        _processed_path = [processed_path, folder].join(FileExt::get_path_separator().as_str());
    }
    recursive_call(_processed_path.as_str(), remaining_path, name)
}