use std::fs::read_dir;

fn main() {
    // TODO: parse stdin for a filename

    match read_dir("/tmp/test-consuela") {
        Ok(v) => {
            for entry in v {
                match entry {
                    Ok(entry) => println!("{:?}", entry.path()),
                    Err(err) => eprintln!("i don't understand wtf is going on: {}", err),
                }
            }
        }
        Err(err) => eprintln!("There was an error reading the directory: {}", err),
    };

    println!("No .... lemon pledge");
}
