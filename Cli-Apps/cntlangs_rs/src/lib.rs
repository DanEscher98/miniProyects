use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
pub mod cli;
pub mod data;

type FileData = HashMap<String, (usize, usize, usize)>;

pub fn classify_files(pwdir: &Path) -> FileData {
    //! It takes a &Path and uses `recurse_files()` to traverse its contents classifying the file
    //! by its extension. It returns a hashmap that contains the total size, lines
    let mut file_data: FileData = HashMap::new();
    for (extype, file) in recurse_files(pwdir).iter() {
        let file = File::open(file).expect("file not found");
        // check if file has readable encoding
        if let Ok(ln) = count_lines(&file) {
            let sz: usize = { file.metadata().unwrap().len().try_into().unwrap() };
            if let Some(&(lines, files, size)) = file_data.get(extype) {
                file_data.insert(extype.to_string(), (lines + ln, files + 1, size + sz));
            } else {
                // initialize the extension key
                file_data.insert(extype.to_string(), (ln, 1, sz));
            }
        }
        todo!("Emit alert about file with unreadable encoding")
    }
    return file_data;
}

fn recurse_files(_pwdir: &Path) -> Vec<(String, String)> {
    //! Recurse through the directory and yield only the files that are not in `IGNFILE`

    unimplemented!();
}

fn count_lines(file: &File) -> Result<usize, io::Error> {
    let file = BufReader::new(file);
    let mut counter: usize = 0;
    for _ in file.lines() {
        counter += 1;
    }
    Ok(counter)
}

pub fn human_readable(byte_sz: usize) -> String {
    let kb: f32 = 1024.0;
    let mb = kb.powf(2.0);
    let gb = kb.powf(3.0);
    let size_hr: String;
    let byte_sz: f32 = byte_sz as f32;
    size_hr = if byte_sz > gb {
        format!("{:.2}G", byte_sz / gb)
    } else if byte_sz > mb {
        format!("{:.2}M", byte_sz / mb)
    } else if byte_sz > kb {
        format!("{:.2}K", byte_sz / kb)
    } else {
        format!("{}B", byte_sz)
    };
    size_hr
}

pub fn format_output(_file_data: FileData) {
    unimplemented!();
}
