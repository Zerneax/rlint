use std::fs;

pub fn get_files_of_directory(directory: &str, extension: &str) -> Vec<String>{
    let mut files_to_analyze = Vec::new();
    let entries = fs::read_dir(directory).expect("Error when reading directory");

    for e in entries {
        match e {
            Ok(entry) => {
                let path = entry.path();
                if !path.is_dir() {
                    if !path.extension().is_none() && path.extension().unwrap().eq(extension) {
                        files_to_analyze.push(String::from(path.to_str().unwrap()));
                    }
                } else {
                    files_to_analyze.append(&mut get_files_of_directory(path.to_str().unwrap(), extension));
                }
            },
            Err(_) => println!("Error on entry: {:?}", e)
        }
    }

    files_to_analyze
}
