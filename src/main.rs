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
    if std::env::args().len() < 3 {
        println!("{0}", HELP_STRING);
        std::process::exit(0);
    }

    let (in_file, out_file) = (
        std::env::args().nth(1).expect(HELP_STRING),
        std::env::args().nth(2).expect(HELP_STRING),
    );
    
    println!("{0}\nStarting...", HELP_STRING);

    let args = Args {
        in_path: std::path::PathBuf::from(in_file),
        out_path: std::path::PathBuf::from(out_file),
    };

    if args.in_path.is_file() {
        if let Err(error) = parse_file(&args.in_path, &mut args.out_path.to_owned()) {
            println!("Parsing error in file {0} - {1}", &args.in_path.to_string_lossy(), error);
        };
    } else if args.in_path.is_dir() {
        let dir = args.in_path.read_dir();
        
        if let Err(error) = dir {
            println!("Could not open directory {0} - {1}", &args.in_path.to_string_lossy(), error);
            std::process::exit(1);
        }

        if !args.out_path.is_dir() {
            if let Err(error) = std::fs::create_dir(&args.out_path) {
                println!("could not create output directory {0} - {1}", &args.out_path.to_string_lossy(), error)
            }
        }

        let handle = thread::spawn(move || {
            for file in dir.unwrap() {
                let mut out_name = args.out_path.to_owned();
                out_name.extend([file.as_ref().unwrap().file_name()]);
                if let Err(error) = parse_file(&file.as_ref().unwrap().path(), &mut out_name) {
                    println!("Parsing error in file {0} - {1}", &args.in_path.to_string_lossy(), error);
                }
            }
        });

        handle.join().unwrap();
    }

    println!("Done!");
}

fn parse_file(in_file: &std::path::PathBuf, out_file: &mut std::path::PathBuf) -> Result<(), std::io::Error> {
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
        return Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "Skipping hidden file"));
    }

    out_file.set_extension("html");

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
    
    std::fs::write(out_file, response)?;
    
    return Ok(());
}
