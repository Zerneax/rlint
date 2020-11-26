extern crate config;
use std::collections::HashMap;
use std::fmt;

pub enum Rule {
    LineLength(i32),
    Quote(String)
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rule::LineLength(value) => write!(f, "{}", value),
            Rule::Quote(value) => write!(f, "{}", value)
        }
    }
}

pub struct Rules {
    pub rules: Vec<Rule>
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.rules.iter().fold(Ok(()), |result, rule| {
            result.and_then(|_| writeln!(f, "{}", rule))
        })
    }
}

pub fn get_config() -> Rules {
    let mut config_lint = config::Config::default();
    config_lint
        .merge(config::Environment::with_prefix("APP")).unwrap()
        .merge(config::File::with_name("Settings")).unwrap();
    match config_lint.try_into::<HashMap<String,String>>() {
        Ok(map) => return init_rules(map),
        Err(_) => return Rules { rules: vec![] }
    }


}

pub fn init_rules(settings: HashMap<String, String>) -> Rules {
    let mut rules: Vec<Rule> = Vec::new();
    for (key, value) in settings {
        if key.eq("LineLength") {
            rules.push(Rule::LineLength(value.parse::<i32>().unwrap()));
        } else if key.eq("Quote") {
            if value.eq("simple") {
                rules.push(Rule::Quote(String::from("\"")));
            } else {
                rules.push(Rule::Quote(String::from("'")));
            }
        }
    }

    Rules {rules: rules}
}
