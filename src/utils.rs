pub fn error(line: u32, message: &str, had_err: &mut bool) {
    report(line, "", message, had_err);
}

pub fn report(line: u32, location: &str, message: &str, had_err: &mut bool) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
    *had_err = true;
}
