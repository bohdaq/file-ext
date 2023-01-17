use std::fs::File;
use crate::directory_ext_impl::DirectoryExtImpl;
use crate::file_ext_impl::FileExtImpl;
use crate::FileExt;
use crate::path_ext_impl::PathExtImpl;
use crate::symlink_ext_impl::SymlinkExtImpl;

#[test]
fn symlink_check() {
    let path = PathExtImpl::build_path(&["test", "index_rewrite"]);
    create_rewrite_index_symlink();

    let is_symlink = SymlinkExtImpl::is_symlink(path.as_str()).unwrap();
    assert!(is_symlink);

    FileExtImpl::delete_file(path.as_str()).unwrap();
}

#[test]
fn not_symlink_check() {
    let path = PathExtImpl::build_path(&["test", "index.html"]);
    let is_symlink = SymlinkExtImpl::is_symlink(path.as_str()).unwrap();
    assert!(!is_symlink);
}

#[test]
fn file_exists() {
    let working_directory = FileExt::get_static_filepath("").unwrap();
    let absolute_path = PathExtImpl::build_path(&[working_directory.as_str(), "test", "index_rewrite"]);
    create_rewrite_index_symlink();

    let exists = SymlinkExtImpl::does_symlink_exist(absolute_path.as_str());
    assert!(exists);

    FileExtImpl::delete_file(absolute_path.as_str()).unwrap_or_default();

}

#[test]
fn symlink_creation() {
    let symlink_path = ["test", "index-link"].join(FileExt::get_path_separator().as_str());

    if SymlinkExtImpl::does_symlink_exist(symlink_path.as_str()) {
        FileExtImpl::delete_file(symlink_path.as_str()).unwrap();
    }

    let path = [FileExt::get_static_filepath("").unwrap(), "test".to_string()].join(PathExtImpl::get_path_separator().as_str());
    let points_to = [path.to_string(), "index.html".to_string()].join(PathExtImpl::get_path_separator().as_str());

    // FileExt::create_file("out.log").unwrap();

    let boxed_symlink = SymlinkExtImpl::create_symlink(
        path.as_str(),
        "index-link",
        points_to.as_str());


    assert!(boxed_symlink.is_ok());

    let symlink_created = SymlinkExtImpl::does_symlink_exist(symlink_path.as_str());
    assert!(symlink_created);

    let actual_points_to = SymlinkExtImpl::symlink_points_to(symlink_path.as_str()).unwrap();
    assert_eq!(points_to, actual_points_to);

    FileExtImpl::delete_file(symlink_path.as_str()).unwrap();
}

#[test]
fn link_points_to() {
    let symlink_path = ["test", "index_rewrite2"].join(PathExtImpl::get_path_separator().as_str());

    if SymlinkExtImpl::does_symlink_exist(symlink_path.as_str()) {
        FileExtImpl::delete_file(symlink_path.as_str()).unwrap();
    }

    let points_to =
        [
            FileExt::get_static_filepath("").unwrap(),
            "test".to_string(),
            "index.html".to_string()
        ].join(PathExtImpl::get_path_separator().as_str());


    let symlink_dir = "test";
    let path_prefix =
        [
            FileExt::get_static_filepath("").unwrap(),
            symlink_dir.to_string()
        ].join(PathExtImpl::get_path_separator().as_str());

    let boxed_symlink = SymlinkExtImpl::create_symlink(
        path_prefix.as_str(),
        "index_rewrite2",
        points_to.as_str());


    assert!(boxed_symlink.is_ok());

    let symlink_created = SymlinkExtImpl::does_symlink_exist(symlink_path.as_str());
    assert!(symlink_created);

    let actual_points_to = SymlinkExtImpl::symlink_points_to(symlink_path.as_str()).unwrap();
    assert_eq!(points_to, actual_points_to);

    FileExtImpl::delete_file(symlink_path.as_str()).unwrap();

}

fn create_rewrite_index_symlink() {
    let symlink_path = ["test", "index_rewrite"].join(PathExtImpl::get_path_separator().as_str());

    if SymlinkExtImpl::does_symlink_exist(symlink_path.as_str()) {
        FileExtImpl::delete_file(symlink_path.as_str()).unwrap();
    }

    let path = "test";
    let absolute_path = FileExt::get_static_filepath("").unwrap();
    let path_prefix = [absolute_path, path.to_string()].join(PathExtImpl::get_path_separator().as_str());
    let points_to = [path_prefix.to_string(), "index.html".to_string()].join(PathExtImpl::get_path_separator().as_str());

    let boxed_symlink = SymlinkExtImpl::create_symlink(
        path_prefix.as_str(),
        "index_rewrite",
        points_to.as_str());


    assert!(boxed_symlink.is_ok());

    let symlink_created = SymlinkExtImpl::does_symlink_exist(symlink_path.as_str());
    assert!(symlink_created);
}

#[test]
fn resolve_symlink_path() {
    let root = PathExtImpl::root();
    let folder_up = PathExtImpl::folder_up();

    let symlink_directory_path_node_list =
        [
            root.as_str(),
            "home",
            "someuser",
            "folder",
            "subfolder",
            "subsubfolder",
        ];


    let symlink_points_to_node_list =
        [
            folder_up.as_str(),
            folder_up.as_str(),
            "subfolder2",
            "subsubfolder2",
        ];
    let symlink_points_to = PathExtImpl::build_path(&symlink_points_to_node_list);
    let symlink_directory = PathExtImpl::build_path(&symlink_directory_path_node_list);
    let actual_path = SymlinkExtImpl::resolve_symlink_path(symlink_directory.as_str(), symlink_points_to.as_str()).unwrap();


    let expected_path_node_list =
        [
            root.as_str(),
            "home",
            "someuser",
            "folder",
            "subfolder2",
            "subsubfolder2",
        ];
    let expected_path = PathExtImpl::build_path(&expected_path_node_list);


    assert_eq!(expected_path, actual_path);
}

#[test]
fn resolve_symlink_back_and_forth() {
    let root = PathExtImpl::root();

    let base_dir_node_path = [
        root.as_str(),
        "home",
        "someuser",
        "folder",
        "subfolder",
        "subsubfolder",
    ];
    let base_dir = PathExtImpl::build_path(&base_dir_node_path);

    let symlink_points_to_node_path = [
        "..",
        "..",
        "subfolder2",
        "subsubfolder2",
        "..",
        "..",
        "subfolder",
        "subsubfolder",
    ];
    let symlink_points_to = PathExtImpl::build_path(&symlink_points_to_node_path);


    let expected_node_path = [
        root.as_str(),
        "home",
        "someuser",
        "folder",
        "subfolder",
        "subsubfolder",
    ];
    let expected_path = PathExtImpl::build_path(&expected_node_path);


    let actual_path = SymlinkExtImpl::resolve_symlink_path(&base_dir, &symlink_points_to).unwrap();

    assert_eq!(expected_path, actual_path);
}

#[test]
fn resolve_symlink_back_and_forth_starts_from_subdir() {
    let root = PathExtImpl::root();

    let base_dir_node_path = [
        root.as_str(),
        "home",
        "someuser",
        "folder",
        "subfolder",
    ];
    let base_dir = PathExtImpl::build_path(&base_dir_node_path);

    let symlink_points_to_node_path = [
        "subsubsubfolder",
        "..",
        "..",
        "subfolder2",
        "subsubfolder2",
        "..",
        "..",
        "subfolder",
        "subsubfolder",
    ];
    let symlink_points_to = PathExtImpl::build_path(&symlink_points_to_node_path);

    let expected_node_path = [
        root.as_str(),
        "home",
        "someuser",
        "folder",
        "subfolder",
        "subsubfolder",
    ];
    let expected_path = PathExtImpl::build_path(&expected_node_path);
    let actual_path = SymlinkExtImpl::resolve_symlink_path(&base_dir, &symlink_points_to).unwrap();

    assert_eq!(expected_path, actual_path);
}

#[test]
fn resolve_symlink_path_subdirectory() {
    let root = PathExtImpl::root();

    let base_dir_node_path = [
        root.as_str(),
        "home",
        "someuser",
        "folder",
        "subfolder",
        "subsubfolder",
    ];
    let base_dir = PathExtImpl::build_path(&base_dir_node_path);

    let symlink_points_to_node_path = [
        "subsubsubfolder",
        "subsubsubsubfolder",
    ];
    let symlink_points_to = PathExtImpl::build_path(&symlink_points_to_node_path);

    let expected_node_path = [
        root.as_str(),
        "home",
        "someuser",
        "folder",
        "subfolder",
        "subsubfolder",
        "subsubsubfolder",
        "subsubsubsubfolder",
    ];
    let expected_path = PathExtImpl::build_path(&expected_node_path);

    let actual_path = SymlinkExtImpl::resolve_symlink_path(&base_dir, &symlink_points_to).unwrap();

    assert_eq!(expected_path, actual_path);
}

#[test]
fn resolve_symlink_path_root() {
    let base_dir_node_path =
        [
            "someuser",
            "folder",
            "subfolder",
            "subsubfolder",
        ];
    let base_dir= PathExtImpl::build_path(&base_dir_node_path);
    let symlink_points_to = PathExtImpl::get_temp_folder_path().unwrap();

    let expected_path = PathExtImpl::get_temp_folder_path().unwrap();
    let actual_path = SymlinkExtImpl::resolve_symlink_path(&base_dir, &symlink_points_to).unwrap();

    assert_eq!(expected_path, actual_path);
}

#[test]
fn resolve_symlink_path_not_valid() {
    let base_dir = [PathExtImpl::root(), "home".to_string(), "someuser".to_string()].join(PathExtImpl::get_path_separator().as_str());
    let symlink_points_to = ["..", "..", "..", "..", "tmp", "folder"].join(PathExtImpl::get_path_separator().as_str());

    let boxed_resolve = SymlinkExtImpl::resolve_symlink_path(base_dir.as_str(), symlink_points_to.as_str());
    let is_err = boxed_resolve.is_err();

    assert!(is_err);

    let expected_error = "not valid path for the symlink";
    let actual_error = boxed_resolve.err().unwrap();
    assert_eq!(expected_error, actual_error);
}

#[test]
fn symlink_inside_subdirectory_test() {
    // FileExt::create_file("out.log").unwrap();

    let folder_up = PathExtImpl::folder_up();

    let test_dir = "symlink_resolve";
    if DirectoryExtImpl::does_directory_exist(test_dir) {
        DirectoryExtImpl::delete_directory(test_dir).unwrap();
    }

    DirectoryExtImpl::create_directory(test_dir).unwrap();

    let points_to = PathExtImpl::build_path(&[folder_up.as_str(),"test", "index.html"]);
    SymlinkExtImpl::create_symlink(test_dir, "index-rewrite", points_to.as_str()).unwrap();

    let symlink_path = ["symlink_resolve", "index-rewrite"].join(PathExtImpl::get_path_separator().as_str());
    let exists = SymlinkExtImpl::does_symlink_exist(symlink_path.as_str());
    assert!(exists);

    let actual_points_to = SymlinkExtImpl::symlink_points_to(symlink_path.as_str()).unwrap();
    let resolved_points_to = SymlinkExtImpl::resolve_symlink_path(test_dir, actual_points_to.as_str()).unwrap();

    let working_directory = FileExt::get_static_filepath("").unwrap();
    let absolute_path_symlink_points_to_node_path =
        [
            working_directory.as_str(),
            "test",
            "index.html"
        ];
    let absolute_path_symlink_points_to = PathExtImpl::build_path(&absolute_path_symlink_points_to_node_path);

    assert_eq!(absolute_path_symlink_points_to, resolved_points_to);


    DirectoryExtImpl::delete_directory(test_dir).unwrap();
}

#[test]
fn actual_symlinks_test_same_folder() {
    let symlink_dir = "symlink_resolve2";
    let symlink_name = "index-rewrite";
    let points_to = "index.html";
    let expected_file_content = "1234";

    if DirectoryExtImpl::does_directory_exist(symlink_dir) {
        DirectoryExtImpl::delete_directory(symlink_dir).unwrap();
    }

    DirectoryExtImpl::create_directory(symlink_dir).unwrap();

    let file_node_list_path = [symlink_dir, points_to];
    let file_path = PathExtImpl::build_path(&file_node_list_path);
    FileExt::create_file(&file_path).unwrap();

    FileExt::write_file(&file_path, expected_file_content.as_bytes()).unwrap();
    let file_exists = FileExt::does_file_exist(&file_path);
    assert!(file_exists);

    let file_content = FileExt::read_file(&file_path).unwrap();
    assert_eq!(file_content, expected_file_content.as_bytes());

    SymlinkExtImpl::create_symlink(
        symlink_dir,
        symlink_name,
        points_to
    ).unwrap();


    let symlink_node_list_path = [symlink_dir, symlink_name];
    let symlink_path = PathExtImpl::build_path(&symlink_node_list_path);

    let exists = SymlinkExtImpl::does_symlink_exist(&symlink_path);
    assert!(exists);

    let symlink_points_to = SymlinkExtImpl::symlink_points_to(&symlink_path).unwrap();
    let resolved_points_to = SymlinkExtImpl::resolve_symlink_path(
        symlink_dir,
        &symlink_points_to
    ).unwrap();

    let working_directory = FileExt::get_static_filepath("").unwrap();
    let absolute_path_symlink_points_to_node_path =
        [
            working_directory.as_str(),
            symlink_dir,
            points_to
        ];
    let absolute_path_symlink_points_to = PathExtImpl::build_path(&absolute_path_symlink_points_to_node_path);

    let actual_content = FileExt::read_file(&absolute_path_symlink_points_to).unwrap();

    assert_eq!(absolute_path_symlink_points_to, resolved_points_to);
    assert_eq!(expected_file_content.as_bytes(), actual_content);

    DirectoryExtImpl::delete_directory(symlink_dir).unwrap();
}

#[test]
fn symlink_points_to_file_in_same_folder() {
    let symlink_dir = "symlink_resolve2";
    let symlink_name = "index-rewrite";
    let points_to = "index.html";
    let expected_file_content = "1234";

    if DirectoryExtImpl::does_directory_exist(symlink_dir) {
        DirectoryExtImpl::delete_directory(symlink_dir).unwrap();
    }

    DirectoryExtImpl::create_directory(symlink_dir).unwrap();

    let file_node_list_path = [symlink_dir, points_to];
    let file_path = PathExtImpl::build_path(&file_node_list_path);
    FileExt::create_file(&file_path).unwrap();

    FileExt::write_file(&file_path, expected_file_content.as_bytes()).unwrap();
    let file_exists = FileExt::does_file_exist(&file_path);
    assert!(file_exists);

    let file_content = FileExt::read_file(&file_path).unwrap();
    assert_eq!(file_content, expected_file_content.as_bytes());

    SymlinkExtImpl::create_symlink(
        symlink_dir,
        symlink_name,
        points_to
    ).unwrap();


    let symlink_node_list_path = [symlink_dir, symlink_name];
    let symlink_path = PathExtImpl::build_path(&symlink_node_list_path);

    let exists = SymlinkExtImpl::does_symlink_exist(&symlink_path);
    assert!(exists);

    let symlink_points_to = SymlinkExtImpl::symlink_points_to(&symlink_path).unwrap();
    let resolved_points_to = SymlinkExtImpl::resolve_symlink_path(
        symlink_dir,
        &symlink_points_to
    ).unwrap();

    let working_directory = FileExt::get_static_filepath("").unwrap();
    let absolute_path_symlink_points_to_node_path =
        [
            working_directory.as_str(),
            symlink_dir,
            points_to
        ];
    let absolute_path_symlink_points_to = PathExtImpl::build_path(&absolute_path_symlink_points_to_node_path);

    let actual_content = FileExt::read_file(&absolute_path_symlink_points_to).unwrap();

    assert_eq!(absolute_path_symlink_points_to, resolved_points_to);
    assert_eq!(expected_file_content.as_bytes(), actual_content);

    DirectoryExtImpl::delete_directory(symlink_dir).unwrap();
}

#[test]
fn symlink_points_to_file_in_subdirectory() {
    // FileExt::create_file("out.log").unwrap();
    // FileExt::write_file("out.log", "1234".as_bytes()).unwrap();

    let working_directory = FileExt::get_static_filepath("").unwrap();
    let symlink_dir = working_directory.to_string();
    let file_dir = "symlink_resolve3";
    let symlink_name = "index-rewrite";
    let points_to_filename = "index.html";
    let points_to = PathExtImpl::build_path(&
            [
                working_directory.as_str(),
                file_dir,
                points_to_filename,
            ]);
    let expected_file_content = "1234";

    if DirectoryExtImpl::does_directory_exist(file_dir) {
        DirectoryExtImpl::delete_directory(file_dir).unwrap();
    }

    DirectoryExtImpl::create_directory(file_dir).unwrap();

    let file_node_list_path = [file_dir, points_to_filename];
    let file_path = PathExtImpl::build_path(&file_node_list_path);
    FileExt::create_file(&file_path).unwrap();

    FileExt::write_file(&file_path, expected_file_content.as_bytes()).unwrap();
    let file_exists = FileExt::does_file_exist(&file_path);
    assert!(file_exists);

    let file_content = FileExt::read_file(&file_path).unwrap();
    assert_eq!(file_content, expected_file_content.as_bytes());

    // FileExt::write_file("out.log", format!("\nsymlink_dir: {}", symlink_dir).as_bytes()).unwrap();
    // FileExt::write_file("out.log", format!("\nsymlink_name: {}", symlink_name).as_bytes()).unwrap();
    // FileExt::write_file("out.log", format!("\npoints_to: {}", points_to).as_bytes()).unwrap();


    SymlinkExtImpl::create_symlink(
        &symlink_dir,
        symlink_name,
        &points_to
    ).unwrap();


    let symlink_node_list_path = [symlink_dir.as_str(), symlink_name];
    let symlink_path = PathExtImpl::build_path(&symlink_node_list_path);

    let exists = SymlinkExtImpl::does_symlink_exist(&symlink_path);
    assert!(exists);

    let symlink_points_to = SymlinkExtImpl::symlink_points_to(&symlink_path).unwrap();
    let resolved_points_to = SymlinkExtImpl::resolve_symlink_path(
        &symlink_dir,
        &symlink_points_to
    ).unwrap();


    let actual_content = FileExt::read_file(&points_to).unwrap();

    assert_eq!(points_to, resolved_points_to);
    assert_eq!(expected_file_content.as_bytes(), actual_content);

    FileExt::delete_directory(file_dir).unwrap();
    FileExt::delete_file(&symlink_path).unwrap();
}
