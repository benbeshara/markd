use crate::md_parse;
use tiny_http::{Response, Server};

pub fn main() {
    let listen_addr = format!("0.0.0.0:{}", std::env::var("LISTEN_PORT").expect("999"));
    let server = Server::http(&listen_addr).unwrap();

    println!("Listening on {}", &listen_addr);

    for request in server.incoming_requests() {
        println!("{:?} {:?}", request.method(), request.url());

        let path = request.url().strip_prefix('/');

        let parsed = match md_parse::parse_file(path.unwrap()) {
            Ok(p) => p,
            Err(e) => {
                println!("Error! {e}");
                continue;
            }
        };

        let header =
            tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();

        let mut response = Response::from_string(parsed);
        response.add_header(header);

        if let Err(e) = request.respond(response) {
            println!("Response failed! {}", e);
        }
    }
}
