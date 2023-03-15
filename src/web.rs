use tiny_http::{Server, Response};

use crate::md_parse;

pub fn main(){
	let server = Server::http("0.0.0.0:9999").unwrap();
	
	for request in server.incoming_requests() {
		println!("{:?} {:?}",
			request.method(),
			request.url()
		);
		
		let path = request.url().strip_prefix('/');
	
		let parsed = match md_parse::parse_file(path.unwrap()){
			Ok(p) => p,
			Err(e) => {println!("Error! {e}"); continue;}
		};
		
		let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();
		
		let mut response = Response::from_string(parsed);
		response.add_header(header);
		
		if let Err(e) = request.respond(response){
			println!("Response failed! {}", e);
		}	
	}
}