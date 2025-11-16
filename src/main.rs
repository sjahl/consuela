use chrono::offset::Utc;
use chrono::DateTime;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::{create_dir, read_dir, rename};
use std::io;
use std::path::Path;
use std::time::SystemTime;

// Represents a file and creation date
struct DatedFile {
    path: String,
    ctime: String,
}

// Build a list of DatedFiles in the given filepath
fn dir_listing(fp: &str) -> io::Result<Vec<DatedFile>> {
    let mut file_list: Vec<DatedFile> = Vec::new();
    let my_folders = Regex::new(r"^\d{4}-\d{2}$|^\.DS_Store$").unwrap();
    for entry in read_dir(fp)? {
        match entry {
            Ok(entry) => {
                let name = entry.path();
                let time = entry.metadata()?.created()?;
                let dtime = derive_date(time);
                if !my_folders.is_match(name.file_name().unwrap().to_str().unwrap()) {
                    file_list.push(DatedFile {
                        path: name.display().to_string(),
                        ctime: dtime,
                    });
                }
            }
            Err(err) => eprintln!("i don't understand wtf is going on: {err}"),
        }
    }
    Ok(file_list)
}

// Derive the YYYY-MM creation date of the file
fn derive_date(st_ctime: SystemTime) -> String {
    let datetime: DateTime<Utc> = st_ctime.into();
    datetime.format("%Y-%m").to_string()
}

fn move_file_to_directory(source: &Path, target_dir: &Path) -> Result<(), io::Error> {
    // Generate the target filename
    let target_filename = source.file_name().ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Source path must have a filename",
    ))?;
    let target_path = target_dir.join(target_filename);

    // Move the file
    rename(source, target_path)
}

fn main() {
    // TODO: parse stdin for a filename

    let args: Vec<String> = env::args().collect();
    let root_folder = Path::new(&args[1]);
    let files_list = match dir_listing(&root_folder.display().to_string()) {
        Ok(v) => v,
        Err(err) => panic!("error: {err}"),
    };

    let mut date_groups = HashMap::new();
    for item in files_list {
        date_groups
            .entry(item.ctime)
            .or_insert_with(Vec::new)
            .push(item.path);
    }

    for (k, v) in &date_groups {
        let full_fp = root_folder.join(k);
        let _ = create_dir(&full_fp);
        println!("will move {v:?} into {k:?}");
        for filename in v {
            let source = Path::new(filename);
            match move_file_to_directory(source, &full_fp) {
                Ok(()) => println!("File moved successfully"),
                Err(error) => println!("Error moving file: {error}"),
            }
        }
    }

    println!("No .... lemon pledge");
}
