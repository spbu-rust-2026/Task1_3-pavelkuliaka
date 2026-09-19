use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut path = String::new();
    let result = io::stdin().read_line(&mut path);
    let path = path.trim();
    if let Ok(size) = result
        && size != 0
    {
        let result = File::open(path);
        if let Ok(mut file) = result {
            let mut buffer = Vec::new();
            let status = file.read_to_end(&mut buffer);
            if status.is_ok() {
                println!("success");
                return Ok(());
            }
        }
    }
    println!("failure");
    Ok(())
}
