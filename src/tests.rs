use std::{thread, time};
use crate::FileExt;

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
fn absolute_path_to_working_directory() {
    let boxed_path = FileExt::get_static_filepath(FileExt::get_path_separator().as_str());
    assert!(boxed_path.is_ok());
}


