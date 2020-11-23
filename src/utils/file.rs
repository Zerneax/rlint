use std::fs;
use std::fs::File;
use std::str;
use std::string::String;
use std::io::Read;
use std::fmt;

pub struct Files {
    pub files: Vec<FileForAnalyze>,
}

impl fmt::Display for Files {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.files.iter().fold(Ok(()), |result, file| {
            result.and_then(|_| writeln!(f, "{}", file))
        })
    }
}

pub struct FileForAnalyze {
    pub lines: Vec<Line>,
}

impl fmt::Display for FileForAnalyze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.lines.iter().fold(Ok(()), |result, line| {
            result.and_then(|_| writeln!(f, "{}", line))
        })
    }
}   

pub struct Line {
    pub content: String,
    line: i32,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line: {0} | content: {1}", self.line, self.content)
    }
}

pub fn get_files_to_analyze(directory: &str, extension: &str) -> Files {
    Files {files: get_files_of_directory(directory, extension)}
}

fn get_files_of_directory(directory: &str, extension: &str) -> Vec<FileForAnalyze> {
    let mut files_to_analyze: Vec<FileForAnalyze> = Vec::new();
    let entries = fs::read_dir(directory).expect("Error when reading directory");

    for e in entries {
        match e {
            Ok(entry) => {
                let path = entry.path();
                if !path.is_dir() {
                    if !path.extension().is_none() && path.extension().unwrap().eq(extension) {
                        let filename: &str = path.to_str().unwrap();
                        files_to_analyze.push(read_file(filename));
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

fn read_file(filename: &str) -> FileForAnalyze {
    let lines_string = get_content_of_file(filename);
    let mut lines_line: Vec<Line> = Vec::new();

    for (pos, l) in lines_string.iter().enumerate() {
        lines_line.push(Line {content: l.to_string(), line: pos as i32});
    }

    FileForAnalyze {lines: lines_line}

}

fn get_content_of_file(filename: &str) -> Vec<String>{
    let mut file_bytes = Vec::new();

    let mut file = File::open(filename).expect("Error when opening file");
    file.read_to_end(&mut file_bytes).expect("Error when reading file");
    
    let file_string: String = str::from_utf8(&mut file_bytes).expect("Error when parsing file").to_string();

    return file_string.split("\n").map(String::from).collect::<Vec<String>>();
}
