use regex::Regex;
use std::io::Error;
use std::path::{Path, PathBuf};

struct Args {
    in_path: std::path::PathBuf,
    out_path: std::path::PathBuf,
}

struct Element {
    tag: String,
    pattern: Regex,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Usage: {0} <input> <output>", args[0]);

    if args.len() < 3 {
        std::process::exit(0);
    }

    let args = Args {
        in_path: PathBuf::from(&args[1]),
        out_path: PathBuf::from(&args[2]),
    };

    let mut failed_files = 0;
    let mut total_files = 1;

    println!("Markd'ing {0}: Begin", &args.in_path.to_string_lossy());

    if args.in_path.is_file() {
        if let Err(error) = parse_file(&args.in_path, &args.out_path) {
            failed_files += 1;
            println!(
                "Parsing error in file {0} - {1}",
                &args.in_path.to_string_lossy(),
                error
            );
        };
    } else if args.in_path.is_dir() {
        let dir = args.in_path.read_dir();

        if let Err(error) = dir {
            println!(
                "Could not open directory {0} - {1}",
                &args.in_path.to_string_lossy(),
                error
            );
            std::process::exit(1);
        }

        println!("Attempting to compile item(s)...");

        if !args.out_path.is_dir() {
            if let Err(error) = std::fs::create_dir(&args.out_path) {
                println!(
                    "could not create output directory {0} - {1}",
                    &args.out_path.to_string_lossy(),
                    error
                )
            }
        }

        for file in dir.unwrap() {
            total_files += 1;
            let out_name = &args.out_path;
            if let Err(error) = parse_file(
                &file.as_ref().unwrap().path(),
                &out_name.join(file.as_ref().unwrap().file_name()),
            ) {
                failed_files += 1;
                println!(
                    "Notice while parsing file {0}: \"{1}\"",
                    &file.as_ref().unwrap().path().to_string_lossy(),
                    error
                );
            }
        }
    }

    let successful_files = total_files - failed_files;
    println!(
        "Done! {0} succeeded, {1} failed",
        successful_files, failed_files
    );
}

fn parse_file(in_file: &Path, out_file: &Path) -> Result<(), Error> {
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

    if in_file
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with('.')
        || !in_file.exists()
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Skipping hidden file",
        ));
    }

    println!(
        "Converting {0} > {1}",
        in_file.to_str().unwrap_or("unknown"),
        out_file.to_str().unwrap_or("unknown")
    );

    let mut response = std::fs::read_to_string(in_file)?;

    println!("Parsing...");

    for r in v {
        response = r.pattern.replace_all(&response, r.tag).to_string();
    }

    std::fs::write(out_file.with_extension("html"), response)?;
    Ok(())
}
