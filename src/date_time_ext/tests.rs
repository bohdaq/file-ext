use std::time::SystemTime;
use crate::date_time_ext::DateTimeExt;
use crate::FileExt;


#[test]
fn system_to_nanos() {
    let now = SystemTime::now();
    let nanos = DateTimeExt::_system_time_to_unix_nanos(now);
    assert_ne!(nanos, 0);
}

#[test]
fn now_as_nanos() {
    let nanos = DateTimeExt::_now_unix_epoch_nanos();
    assert_ne!(nanos, 0);
}

#[test]
fn new_directory_create_delete() {
    let path = "new_directory";

    let boxed_create = FileExt::create_directory(path);
    assert!(boxed_create.is_ok());

    assert!(FileExt::does_directory_exist(path));

    let boxed_delete = FileExt::delete_directory(path);
    assert!(boxed_delete.is_ok());
}