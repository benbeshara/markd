use regex::Regex;
use std::thread;

struct Args {
    in_path: std::path::PathBuf,
    out_path: std::path::PathBuf,
}

struct Element {
    tag: String,
    pattern: regex::Regex,
}

static HELP_STRING: &str = r#"Usage: markd <input> <output>"#;

fn main() {
    println!("Starting...");
    if std::env::args().len() < 3 {
        println!("{0}", HELP_STRING);
        std::process::exit(0);
    }

    let (in_file, out_file) = (
        std::env::args().nth(1).expect(HELP_STRING),
        std::env::args().nth(2).expect(HELP_STRING),
    );

    let args = Args {
        in_path: std::path::PathBuf::from(in_file),
        out_path: std::path::PathBuf::from(out_file),
    };

    if args.in_path.is_file() {
        parse_file(args.in_path, args.out_path);
    } else if args.in_path.is_dir() {
        let dir = args.in_path.read_dir().expect(
            format!(
                "Could not open directory {0}",
                args.in_path.to_str().unwrap(),
            )
            .as_str(),
        );
        if !args.out_path.is_dir() {
            std::fs::create_dir(&args.out_path).expect(
                format!(
                    "Could not create directory {0}",
                    args.out_path.to_str().unwrap()
                )
                .as_str(),
            );
        }

        let handle = thread::spawn(move || {
            for file in dir {
                let mut out_name = args.out_path.to_path_buf();
                out_name.extend([file.as_ref().unwrap().file_name()]);
                parse_file(file.as_ref().unwrap().path(), out_name);
            }
        });

        handle.join().unwrap();
    }

    println!("Done!");
}

fn parse_file(in_file: std::path::PathBuf, mut out_file: std::path::PathBuf) -> bool {
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
        .starts_with(".")
        || !in_file.exists()
    {
        println!(
            "Skipping hidden file {0}",
            in_file.to_str().unwrap_or("unknown")
        );
        return false;
    }

    out_file.set_extension("html");

    println!(
        "Converting {0} > {1}",
        in_file.to_str().unwrap_or("unknown"),
        out_file.to_str().unwrap_or("unknown")
    );

    let mut result = std::fs::read_to_string(in_file).expect("Could not open file");

    println!("Parsing...");

    for r in v {
        result = r.pattern.replace_all(&result, r.tag).to_string();
    }

    std::fs::write(out_file, result).expect("Could not write file");

    return true;
}
