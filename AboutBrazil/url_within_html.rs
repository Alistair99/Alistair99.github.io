use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("What is the URL to put in an HTML file?");
    let mut alistair_url = String::new();
    io::stdin()
        .read_line(&mut alistair_url)
        .expect("Failed to read line");
    // trim to take out /n at the end of alistair_url
    alistair_url = alistair_url.trim().to_string();

    // Prepare html_content
    let mut html_content = String::from("<html>");

    // declare and initialise temp_str
    let mut temp_str = format!("\n<a href=\"{}\">", alistair_url);
    html_content.push_str(&temp_str);

    temp_str = format!("\n{}\n</a>\n</html>", alistair_url);
    html_content.push_str(&temp_str);

    // Write html_content to file URL.html
    let html_path = alistair_create_file_for_writing("URL.html");
    alistair_write_to_file(&html_content, &html_path);
} // end of fn main()

fn alistair_create_file_for_writing(filename: &str) -> File {
    let path = Path::new(filename);
    let display = path.display(); // to show the path and name of the file
    let file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    file
}

fn alistair_write_to_file(content: &str, mut file: &File) {
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("Couldn't write to {:?}: {}", file, why.to_string()),
        Ok(_) => {} // println!("Successfully wrote to {:?}", file),
    }
}
