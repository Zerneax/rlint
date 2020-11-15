mod file;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let directory_name = &args[1];
    let file_extension = &args[2];

    let files_to_analyze: Vec<String> = file::get_files_of_directory(directory_name, file_extension);
    println!("Files to analyze, {:?}", files_to_analyze);
}

