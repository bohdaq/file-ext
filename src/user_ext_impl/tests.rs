use crate::file_ext_impl::FileExtImpl;
use crate::user_ext_impl::UserExtImpl;

#[test]
fn current_user() {
    let boxed_user = UserExtImpl::get_current_user();
    assert!(boxed_user.is_ok());

    let path = "current-user.log";

    FileExtImpl::create_file(path).unwrap();
    FileExtImpl::write_file(path, boxed_user.unwrap().as_bytes()).unwrap();

    FileExtImpl::delete_file(path).unwrap();
}

#[test]
#[cfg(target_family = "windows")]
fn current_user_domain() {
    let boxed_user_domain = UserExtImpl::get_current_user_domain();
    assert!(boxed_user_domain.is_ok());

    let path = "current-user-domain.log";

    FileExtImpl::create_file(path).unwrap();
    FileExtImpl::write_file(path, boxed_user_domain.unwrap().as_bytes()).unwrap();
    FileExtImpl::delete_file(path).unwrap();
}