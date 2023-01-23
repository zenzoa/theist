use std::io;

pub fn create_error(error_string: &str) -> Box<io::Error> {
	Box::new(io::Error::new(io::ErrorKind::Other, error_string.to_string()))
}
