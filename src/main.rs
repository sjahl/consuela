use chrono::offset::Utc;
use chrono::DateTime;
use std::collections::HashMap;
use std::fs::read_dir;
use std::io;
use std::time::SystemTime;

// Represents a file and creation date
struct DatedFile {
    path: String,
    ctime: String,
}

// Build a list of DatedFiles in the given filepath
fn dir_listing(fp: &str) -> io::Result<Vec<DatedFile>> {
    let mut file_list: Vec<DatedFile> = Vec::new();
    for entry in read_dir(fp)? {
        match entry {
            Ok(entry) => {
                let name = entry.path();
                let time = entry.metadata()?.created()?;

                file_list.push(DatedFile {
                    path: name.display().to_string(),
                    ctime: derive_date(time)?,
                })
            }
            Err(err) => eprintln!("i don't understand wtf is going on: {}", err),
        }
    }
    Ok(file_list)
}

// Derive the YYYY-MM creation date of the file
fn derive_date(st_ctime: SystemTime) -> io::Result<String> {
    let datetime: DateTime<Utc> = st_ctime.into();
    Ok(datetime.format("%Y-%m").to_string())
}

fn main() {
    // TODO: parse stdin for a filename

    // List all the files in the given directory
    // Build a list of DatedFiles
    // Create all the possible date directories
    // Move each file into its dated folder
    let files_list = match dir_listing("/tmp/test-consuela") {
        Ok(v) => v,
        Err(err) => panic!("error: {}", err),
    };

    let mut date_groups = HashMap::new();

    for item in files_list {
        date_groups
            .entry(item.ctime)
            .or_insert_with(Vec::new)
            .push(item.path)
    }

    for (k, v) in &date_groups {
        println!("{k:?} has {v:?}");
    }

    println!("No .... lemon pledge");
}
