use std::io::{self, Error, ErrorKind};

pub(crate) fn error(line: u32, message: &str) -> io::Error {
    report(line, "", message)
}

pub(crate) fn report(line: u32, r#where: &str, message: &str) -> Error {
    eprintln!("[line {} ] Error {}: {}", line, r#where, message);
    Error::new(ErrorKind::Other, "Invalid grammer")
}
