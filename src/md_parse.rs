use regex::Regex;
use std::{io::Error, path::Path};

struct Element {
    tag: String,
    pattern: Regex,
}

pub fn parse_file(in_file: &str) -> Result<String, Error> {
    let v = vec![
        Element {
            tag: "<h1>$1</h1>".to_string(),
            pattern: Regex::new(r"(?m)^#\s(.*)").unwrap(),
        },
        Element {
            tag: "<h2>$1</h2>".to_string(),
            pattern: Regex::new(r"(?m)^#{2}\s(.*)").unwrap(),
        },
        Element {
            tag: "<h3>$1</h3>".to_string(),
            pattern: Regex::new(r"(?m)^#{3}\s(.*)").unwrap(),
        },
        Element {
            tag: "<h4>$1</h4>".to_string(),
            pattern: Regex::new(r"(?m)^#{4}\s(.*)").unwrap(),
        },
        Element {
            tag: "<strong>$1</strong>".to_string(),
            pattern: Regex::new(r"(?m)[^\\]\*\*(.+)\*\*").unwrap(),
        },
        Element {
            tag: "<em>$1</em>".to_string(),
            pattern: Regex::new(r"(?m)[^\\*+]\*(.+)\*").unwrap(),
        },
        Element {
            tag: "<br />".to_string(),
            pattern: Regex::new(r"(?m)^$").unwrap(),
        },
    ];
    
    let file = Path::new(in_file);
    let filename = match file.file_name() {
        Some(it) => it,
        None => return Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "Invalid file")),
    };

    if filename
        .to_str()
        .unwrap()
        .starts_with('.')
        || !file.exists()
    {
        return Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "Invalid file"));
    }

    println!(
        "Converting {0}",
        file.to_str().unwrap_or("unknown")
    );

    let mut response = std::fs::read_to_string(in_file)?;
    
    println!("Parsing...");
    
    for r in v {
        response = r.pattern.replace_all(&response, r.tag).to_string();
    }
    
    Ok(response)
}
