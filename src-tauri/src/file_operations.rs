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

fn cvt_DirEntry_Str(input: &mut Vec<DirEntry>) -> Vec<String> {
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
        let mut sorted_list = cvt_DirEntry_Str(unsorted_list);
        sorted_list
    } else if sort_method=="Z-A" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.file_name().to_string_lossy().into_owned();
            let b_name = b.file_name().to_string_lossy().into_owned();

            b_name.cmp(&a_name)
        });        
        let mut sorted_list = cvt_DirEntry_Str(unsorted_list);
        sorted_list
    } else if sort_method=="last_modified" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().modified().unwrap();
            let b_name = b.metadata().unwrap().modified().unwrap();

            a_name.cmp(&b_name)
        });        
        let mut sorted_list = cvt_DirEntry_Str(unsorted_list);
        sorted_list
    } else if sort_method=="most_recently_modified" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().modified().unwrap();
            let b_name = b.metadata().unwrap().modified().unwrap();

            b_name.cmp(&a_name)
        });        
        let mut sorted_list = cvt_DirEntry_Str(unsorted_list);
        sorted_list
    } else if sort_method=="smallest_size" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().len();
            let b_name = b.metadata().unwrap().len();

            a_name.cmp(&b_name)
        });        
        let mut sorted_list = cvt_DirEntry_Str(unsorted_list);
        sorted_list
    } else if sort_method=="biggest_size" {
        unsorted_list.sort_by(|a, b| {
            let a_name = a.metadata().unwrap().len();
            let b_name = b.metadata().unwrap().len();

            b_name.cmp(&a_name)
        });        
        let mut sorted_list = cvt_DirEntry_Str(unsorted_list);
        sorted_list
    } else if sort_method=="extension" {
        unsorted_list.sort_by(|a, b| {
            let a_tmp = a.path();
            let b_tmp = b.path();
            let a_name = a_tmp.extension().and_then(|e| e.to_str()).unwrap_or("");
            let b_name = b_tmp.extension().and_then(|e| e.to_str()).unwrap_or("");

            b_name.cmp(&a_name)
        });        
        let mut sorted_list = cvt_DirEntry_Str(unsorted_list);
        sorted_list
    } else {
        let mut unsorted_return_list = cvt_DirEntry_Str(unsorted_list);
        unsorted_return_list
    }
}

#[tauri::command]
pub fn list_folders(path: String, sort_method: String) -> Vec<String> {
    let mut unsorted_list: Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(path).unwrap().flatten() {
        if entry.metadata().unwrap().is_dir() {
            unsorted_list.push(entry);
        }
    }
    let mut sorted_list = sort(&mut unsorted_list, sort_method);
    sorted_list
}   