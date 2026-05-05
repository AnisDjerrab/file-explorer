use std::fs::{self, DirEntry, metadata};

#[tauri::command]
pub fn get_home_directory() -> String {
    if let Some(home) = dirs::home_dir() {
        let output = home.to_string_lossy().into_owned();
        output
    } else {
        let output = "Err: home dir not found.".to_string();
        output
    }
}

fn cvt_dir_entry_str(input: &mut Vec<DirEntry>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for entry in input {
        output.push(entry.file_name().to_string_lossy().to_string());
    }
    output
}

fn sort(unsorted_list: &mut Vec<DirEntry>, sort_method: String) -> Vec<String> {
    if sort_method == "A-Z" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.file_name().to_string_lossy().into_owned();
            let b_name = b.file_name().to_string_lossy().into_owned();

            a_name.cmp(&b_name)
        });        
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method=="Z-A" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.file_name().to_string_lossy().into_owned();
            let b_name = b.file_name().to_string_lossy().into_owned();

            b_name.cmp(&a_name)
        });        
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method=="last_modified" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().modified().unwrap();
            let b_name = b.metadata().unwrap().modified().unwrap();

            a_name.cmp(&b_name)
        });        
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method=="most_recently_modified" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().modified().unwrap();
            let b_name = b.metadata().unwrap().modified().unwrap();

            b_name.cmp(&a_name)
        });        
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method=="smallest_size" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().len();
            let b_name = b.metadata().unwrap().len();

            a_name.cmp(&b_name)
        });        
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method=="biggest_size" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().len();
            let b_name = b.metadata().unwrap().len();

            b_name.cmp(&a_name)
        });        
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else if sort_method=="extension" {
        unsorted_list.sort_by(|a, b| {
            if (a.metadata().unwrap().is_dir() || a.metadata().unwrap().is_symlink() && b.metadata().unwrap().is_dir() || b.metadata().unwrap().is_symlink()) {
                let a_tmp = a.path();
                let b_tmp = b.path();
                let a_name = a.file_name().to_string_lossy().into_owned();
                let b_name = b.file_name().to_string_lossy().into_owned();

                a_name.cmp(&b_name)
            } else {
                let a_name = a.file_name().to_string_lossy().into_owned();
                let b_name = b.file_name().to_string_lossy().into_owned();

                a_name.cmp(&b_name)
            }
        });        
        let sorted_list = cvt_dir_entry_str(unsorted_list);
        sorted_list
    } else {
        let unsorted_return_list = cvt_dir_entry_str(unsorted_list);
        unsorted_return_list
    }
}

#[tauri::command]
pub fn list_folders(path: String, sort_method: String) -> (i32, Vec<String>) {
    let mut unsorted_list: Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(path).unwrap().flatten() {
        if entry.metadata().unwrap().is_dir() {
            unsorted_list.push(entry);
        }
    }
    let sorted_list = sort(&mut unsorted_list, sort_method);
    (0, sorted_list)
}   

pub fn list_files(path: String, sort_method: String) -> (i32, Vec<String>) {
    let mut unsorted_list: Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(path).unwrap().flatten() {
        if entry.metadata().unwrap().is_file() {
            unsorted_list.push(entry);
        }
    }
    let sorted_list = sort(&mut unsorted_list, sort_method);
    (0, sorted_list)
}   

pub fn list_symlinks(path: String, sort_method: String) -> (i32, Vec<String>) {
    let mut unsorted_list: Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(path).unwrap().flatten() {
        if entry.metadata().unwrap().is_file() {
            unsorted_list.push(entry);
        }
    }
    let sorted_list = sort(&mut unsorted_list, sort_method);
    (0, sorted_list)
}   

fn cvt_mimetype_iconfilename(mimetype: String) -> String {
    let mime_icons_path = "../../src/assets/mimetypes/";
    mimetype.replace("/", "-x-");
    let return_path = format!("{}{}", &mime_icons_path, &mimetype);
    return_path
}

#[tauri::command]
pub fn ls_dir(path: String, sort_method: String, sort_method_dfs: String) -> (Vec<String>, Vec<String>) {
    // first : get all the files
    let mut file_list = list_files(path.clone(), sort_method.clone()).1;
    let mut list_icons_files: Vec<String> = Vec::new();
    for file in &file_list {
        let full_path = format!("{}{}", &path, &file);
        let mime: &str = tree_magic_mini::from_filepath(std::path::Path::new(&full_path)).unwrap_or("");
        list_icons_files.push(cvt_mimetype_iconfilename(mime.to_string()));
    }
    // second : the symlinks now
    let mut symlink_list = list_symlinks(path.clone(), sort_method.clone()).1;
    let mut list_icons_symlinks: Vec<String> = Vec::new();
    for symlink in &symlink_list {
        let full_path = format!("{}{}", &path, &symlink);
        let mime: &str = tree_magic_mini::from_filepath(std::path::Path::new(&full_path)).unwrap_or("");
        list_icons_symlinks.push(cvt_mimetype_iconfilename(mime.to_string()));
    }
    // third : the symlinks now
    let mut dir_list = list_folders(path.clone(), sort_method.clone()).1;
    let mut list_icons_dir: Vec<String> = Vec::new();
    for dir in &dir_list {
        let full_path = format!("{}{}", &path, &dir);
        let mime: &str = tree_magic_mini::from_filepath(std::path::Path::new(&full_path)).unwrap_or("");
        list_icons_dir.push(cvt_mimetype_iconfilename(mime.to_string()));
    }
    let mut merged_file_list = Vec::new();
    merged_file_list.append(&mut file_list);
    merged_file_list.append(&mut dir_list);
    merged_file_list.append(&mut symlink_list);
    let merged_icon_list = Vec::new();
    merged_file_list.append(&mut list_icons_files);
    merged_file_list.append(&mut list_icons_dir);
    merged_file_list.append(&mut list_icons_symlinks);
    (merged_file_list, merged_icon_list)
}