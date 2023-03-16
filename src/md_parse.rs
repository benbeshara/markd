use regex::Regex;
use std::{io::Error, path::Path};

struct Element<'a> {
    tag: &'a str,
    pattern: Regex,
    operation: &'a str,
}

pub fn parse_file(in_file: &str) -> Result<String, Error> {
    let v = vec![
        Element {
            tag: "<h1>$1</h1>",
            pattern: Regex::new(r"(?m)^#\s(.*)").unwrap(),
            operation: "Tag",
        },
        Element {
            tag: "<h2>$1</h2>",
            pattern: Regex::new(r"(?m)^#{2}\s(.*)").unwrap(),
            operation: "Tag",
        },
        Element {
            tag: "<h3>$1</h3>",
            pattern: Regex::new(r"(?m)^#{3}\s(.*)").unwrap(),
            operation: "Tag",
        },
        Element {
            tag: "<h4>$1</h4>",
            pattern: Regex::new(r"(?m)^#{4}\s(.*)").unwrap(),
            operation: "Tag",
        },
        Element {
            tag: "<strong>$1</strong>",
            pattern: Regex::new(r"(?m)[^\\]\*\*(.+)\*\*").unwrap(),
            operation: "Tag",
        },
        Element {
            tag: "<em>$1</em>",
            pattern: Regex::new(r"(?m)[^\\*+]\*(.+)\*").unwrap(),
            operation: "Tag",
        },
        Element {
            tag: "<br />",
            pattern: Regex::new(r"(?m)^$").unwrap(),
            operation: "Tag",
        },
        Element {
            tag: "",
            pattern: Regex::new(r"(?m)^!\s(.*)").unwrap(),
            operation: "Title",
        },
    ];

    let file = Path::new(in_file);
    let filename = match file.file_name() {
        Some(it) => it,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Invalid file",
            ))
        }
    };

    if filename.to_str().unwrap().starts_with('.') || !file.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Invalid file",
        ));
    }

    println!("Converting {0}", file.to_str().unwrap_or("unknown"));

    let mut response = std::fs::read_to_string(in_file)?;
    let mut title = "Markdown Page".to_string();

    println!("Parsing...");

    for r in v {
        if r.operation.eq("Title") {
            if let Some(capt) = r.pattern.captures(&response) {
                title = capt
                    .get(0)
                    .map_or("Markdown Page", |m| m.as_str())
                    .replace("! ", "");
            }
        }
        response = r.pattern.replace_all(&response, r.tag).to_string();
    }

    let result = format!(
        "<!DOCTYPE html><html><head><title>{}</title></head><body>{}</body></html>",
        title, response
    );

    Ok(result)
}
