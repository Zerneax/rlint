mod file;
use std::env;
use file::Files;

fn main() {

    let args: Vec<String> = env::args().collect();

    let directory_name = &args[1];
    let file_extension = &args[2];

    let files_to_analyze: Files = file::get_files_to_analyze(directory_name, file_extension);
    println!("Files to analyze, {}", files_to_analyze);
}

