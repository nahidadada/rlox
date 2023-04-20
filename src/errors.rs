pub mod log {
    pub fn error(line: i32, message: &str) {
        report(line, "", message);
    }
    
    pub fn report(line: i32, place: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, place, message);
        //had_error = true;
    }
}