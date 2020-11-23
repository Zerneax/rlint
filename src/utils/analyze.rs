//mod file;

//use config_lint::Rules;
use super::file::Files;
use super::config_lint::Rules;
use super::config_lint::Rule;
use super::file::Line;

pub fn analyze_files(files: &Files, rules: &Rules) {
    let mut errors: Vec<&Line> = Vec::new();

    for file in &files.files {
        for line in &file.lines {
            for rule in &rules.rules {
                match rule {
                    Rule::LineLength(ref value) => {
                        if line.content.len() > *value as usize {
                            errors.push(line);
                        }
                    }
                }
            }
        }
    }

    display_errors(errors)

}

pub fn display_errors(errors: Vec<&Line>) {
    for error in errors {
        println!("error : {}", error);
    }
}
