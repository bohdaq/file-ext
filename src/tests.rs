use crate::FileExt;
use crate::symbol::{Symbol, SYMBOL};

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
    let path = "/tmp/test.txt";

    let doesnt_exist = !FileExt::does_file_exist(path);
    assert!(doesnt_exist);

    FileExt::read_or_create_and_write(path, content).unwrap();

    let does_exist = FileExt::does_file_exist(path);
    assert!(does_exist);

    let new_content = "updated data".as_bytes();
    FileExt::read_or_create_and_write(path, new_content).unwrap();

    let file_content = FileExt::read_file(path).unwrap();
    assert_eq!(content, file_content);

    FileExt::delete_file(path).unwrap();
    let doesnt_exist = !FileExt::does_file_exist(path);
    assert!(doesnt_exist);
}

#[test]
fn modification_timestamp() {
    let expected_timestamp : u128 = 1672145538390040940;
    let modified_timestamp = FileExt::file_modified_utc("test/index.html").unwrap();
    assert_eq!(expected_timestamp, modified_timestamp);
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