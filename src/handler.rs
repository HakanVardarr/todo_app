use std::fs::File;
use tiny_http::{Header, Request, Response};

pub fn handle_file(
    request: Request,
    file_name: &str,
    content_type: Option<&str>,
) -> Result<(), std::io::Error> {
    if let Some(ct) = content_type {
        let header = Header::from_bytes("Content-Type", ct).unwrap();
        match File::open(file_name) {
            Ok(file) => request.respond(
                Response::from_file(file)
                    .with_status_code(200)
                    .with_header(header),
            ),
            Err(_) => request.respond(
                Response::from_file(File::open("error.html").unwrap()).with_status_code(404),
            ),
        }
    } else {
        match File::open(file_name) {
            Ok(file) => request.respond(Response::from_file(file).with_status_code(200)),
            Err(_) => request.respond(
                Response::from_file(File::open("error.html").unwrap()).with_status_code(404),
            ),
        }
    }
}
