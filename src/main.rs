mod utils;

use utils::file as fi;
use utils::config_lint as cl;
use utils::analyze as an;

use std::env;

use fi::Files;
use cl::Rules;

fn main() {

    let args: Vec<String> = env::args().collect();

    let directory_name = &args[1];
    let file_extension = &args[2];

    let rules: Rules = cl::get_config();
    //println!("{}", rules);
    let files_to_analyze: Files = fi::get_files_to_analyze(directory_name, file_extension);
    //println!("Files to analyze, {}", files_to_analyze);
    an::analyze_files(&files_to_analyze, &rules);
}


