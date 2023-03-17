use crate::md_parse;
use tiny_http::{Response, Server};

pub fn main() {
    let listen_addr = format!("0.0.0.0:{}", std::env::var("LISTEN_PORT").expect("999"));
    let server = Server::http(&listen_addr).unwrap();

    println!("Listening on {}", &listen_addr);

    for request in server.incoming_requests() {
        println!("{:?} {:?}", request.method(), request.url());

        let path = request.url().strip_prefix('/');

        let header =
            tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();

        let mut response: Response<std::io::Cursor<Vec<u8>>>;

        if path.expect("").is_empty() {
            response = Response::from_string(index());
        } else {
            response = Response::from_string(markdown_file(path.unwrap()));
        }

        response.add_header(header);

        if let Err(e) = request.respond(response) {
            println!("Response failed! {}", e);
        }
    }
}

fn index() -> String {
    let file_path = format!("./{}", std::env::var("DATA_DIR").expect("testfiles"));
    let files = std::path::Path::new(&file_path);
    let mut res = String::from("");
    files
        .read_dir()
        .expect("Could not read directory")
        .for_each(|file| {
            if let Ok(file) = file {
                res.push_str(
                    format!(
                        "<a href=\"{1}/{0}\">{0}</a><br />",
                        file.file_name().to_str().unwrap(),
                        file_path
                    )
                    .as_str(),
                );
            }
        });
    res
}

fn markdown_file(path: &str) -> String {
    match md_parse::parse_file(path) {
        Ok(p) => p,
        Err(e) => {
            println!("Error! {e}");
            String::from("404")
        }
    }
}
