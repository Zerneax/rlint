//mod file;

//use config_lint::Rules;
use super::file::Files;
use super::config_lint::Rules;
use super::config_lint::Rule;
use super::file::FileForAnalyze;

pub fn analyze_files(files: &Files, rules: &Rules) {
    let mut errors: Vec<String> = Vec::new();

    for file in &files.files {
        errors.append(&mut check_rules_on_file(file, rules));
    }

    display_errors(errors)

}

pub fn check_rules_on_file(file: &FileForAnalyze, rules: &Rules) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();
    
    for line in &file.lines {
        for rule in &rules.rules {
            match rule {
                Rule::LineLength(ref value) => {
                    if line.content.len() > *value as usize {
                        errors.push(file.filename.to_string() + ":" + &line.line.to_string() + " - line is too long");
                    }
                }
                Rule::Quote(ref value) => {
                    if line.content.contains(value) {
                        errors.push(file.filename.to_string() + ":" + &line.line.to_string() + " - Should be '");
                    }
                }
                Rule::NoTrailingWhitespace => {
                    let last_char: Option<char> = line.content.chars().last();
                    if last_char.is_some() && last_char.unwrap() == ' ' {
                        errors.push(file.filename.to_string() + ":" + &line.line.to_string() + " - trailing whitespace");
                    }
                }
            }
        }
    }

    errors
}

pub fn display_errors(errors: Vec<String>) {
    for error in errors {
        println!("ERROR: {}", error);
    }
}
