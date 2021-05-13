use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

pub fn open_file(
    path: &Path,
    open_options: Option<&mut OpenOptions>,
) -> Result<File, Box<dyn std::error::Error>> {
    let mut oo;
    let open_options = match open_options {
        Some(options) => options,
        None => {
            //let mut oo = OpenOptions::new(); // Does not live long enough
            oo = OpenOptions::new();           // Hoist "o" to the outter scope
            oo.create(true).append(true).write(true)
        }
    };

    let file: File = match open_options.open(path) {
        Ok(file) => file,
        Err(e) => {
            return Err(e.into());
        }
    };

    Ok(file)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = open_file(Path::new("./file.txt"), None)?; 
    write!(&file, "Hello, World\n").unwrap();

    // Error: OpenOptions::new() freed at end of statement
    // let open_options = OpenOptions::new().read(true);
    // So we must hoist "oo" to outter scope
    let mut oo = OpenOptions::new();
    let open_options = oo.read(true);

    let rd_file = open_file(Path::new("./file.txt"), Some(open_options))?;
    let reader = BufReader::new(rd_file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    
    Ok(())
}
