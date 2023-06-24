use crate::file_ext_impl::FileExtImpl;
use crate::FileExt;
use crate::path_ext_impl::PathExtImpl;

#[test]
fn write() {
    let filename = "write-test.content";
    FileExtImpl::create_file(filename).unwrap();

    let expected_content = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <title>Title</title>\n</head>\n<body>\n\n</body>\n</html>";
    FileExtImpl::write_file(filename, expected_content.as_bytes()).unwrap();

    let actual = FileExtImpl::read_file(filename).unwrap();
    assert_eq!(actual, expected_content.as_bytes());

    FileExtImpl::delete_file(filename).unwrap();

}

#[test]
fn file_content() {
    let path = "test/index.html";
    let file_raw_bytes = FileExtImpl::read_file(path).unwrap();
    let content = String::from_utf8(file_raw_bytes).unwrap();

    let content_escaped_newline_carriage_return = str::replace(content.as_str(), "\r\n", "\n");

    let expected_content = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <title>Title</title>\n</head>\n<body>\n\n</body>\n</html>";

    assert_eq!(expected_content, content_escaped_newline_carriage_return);
}

#[test]
fn partial_read() {
    let path = "test/index.html";
    let file_raw_bytes = FileExtImpl::read_file_partially(path, 4, 10).unwrap();
    let content = String::from_utf8(file_raw_bytes).unwrap();

    let expected_content = "CTYPE h";

    assert_eq!(expected_content, content);
}

#[test]
fn does_not_exist() {
    let path = "test/non_existing_file";
    let exists = FileExtImpl::does_file_exist(path);
    assert!(!exists);
}

#[test]
fn read_or_create_and_write() {
    let content = "data".as_bytes();
    let tmp_folder = PathExtImpl::get_temp_folder_path().unwrap();

    let path = [tmp_folder, "test.txt".to_string()].join(PathExtImpl::get_path_separator().as_str());

    let doesnt_exist = !FileExtImpl::does_file_exist(path.as_str());
    assert!(doesnt_exist);

    FileExtImpl::read_or_create_and_write(path.as_str(), content).unwrap();

    let does_exist = FileExtImpl::does_file_exist(path.as_str());
    assert!(does_exist);

    let new_content = "updated data".as_bytes();
    FileExtImpl::read_or_create_and_write(path.as_str(), new_content).unwrap();

    let file_content = FileExtImpl::read_file(path.as_str()).unwrap();
    assert_eq!(content, file_content);

    FileExtImpl::delete_file(path.as_str()).unwrap();
    let doesnt_exist = !FileExtImpl::does_file_exist(path.as_str());
    assert!(doesnt_exist);
}

#[test]
fn file_creation_deletion() {
    let path = "test/file-creation.txt";

    let exists = FileExtImpl::does_file_exist(path);
    assert!(!exists);

    FileExtImpl::create_file(path).unwrap();

    let content = FileExtImpl::read_file(path).unwrap();
    assert_eq!(content.len(), 0);

    FileExtImpl::delete_file(path).unwrap();

    let exists = FileExtImpl::does_file_exist(path);
    assert!(!exists);
}

#[test]
fn length() {
    let expected_length: u64 = 398;
    let pwd = FileExt::working_directory().unwrap();
    let length = FileExt::file_length(vec![pwd.as_str(), "LICENSE"]).unwrap();

    assert_eq!(expected_length, length);
}


#[test]
fn copy_file() {
    let pwd = FileExt::working_directory().unwrap();
    FileExt::copy_file(vec![pwd.as_str(), "LICENSE"], vec![pwd.as_str(), "LICENSE_copy2"]).unwrap();

    let path = FileExt::build_path(vec![pwd.as_str(), "LICENSE_copy2"].as_slice());
    FileExt::delete_file(path.as_str()).unwrap();
}

#[test]
fn copy_file_with_callback_and_block_size() {
    let block_size : u64 = 1000000;
    let pwd = FileExt::working_directory().unwrap();
    let mut label = "".to_string();
    let progress_callback = |start, end, total| { label = format!("copying block {}-{} of {} bytes", start, end, total).to_string(); };
    let cancel_callback = |_start, _end, _total| { false };
    FileExt::copy_file_with_callbacks(
        vec![pwd.as_str(), "LICENSE"],
        vec![pwd.as_str(), "LICENSE_copy3"],
        Some(block_size),
        progress_callback,
        cancel_callback
    ).unwrap();

    let path = FileExt::build_path(vec![pwd.as_str(), "LICENSE_copy3"].as_slice());
    FileExt::delete_file(path.as_str()).unwrap();
}

#[test]
fn copy_file_with_callback_and_block_size_starting_from_byte() {
    let block_size : u64 = 1000000;
    let pwd = FileExt::working_directory().unwrap();
    let mut label = "".to_string();
    let progress_callback = |start, end, total| { label = format!("copying block {}-{} of {} bytes", start, end, total).to_string(); };
    let cancel_callback = |_start, _end, _total| { false };
    let starting_byte = 4;
    FileExt::copy_file_with_callbacks_starting_from_byte(
        vec![pwd.as_str(), "LICENSE"],
        vec![pwd.as_str(), "LICENSE_copy4"],
        starting_byte,
        Some(block_size),
        progress_callback,
        cancel_callback
    ).unwrap();

    let path = FileExt::build_path(vec![pwd.as_str(), "LICENSE_copy4"].as_slice());
    FileExt::delete_file(path.as_str()).unwrap();
}
