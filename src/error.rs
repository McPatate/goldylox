// pub trait ErrorReporter {
//     fn new() -> Self;
//     fn error(&mut self, line: usize, r#type: &str, message: &str);
//     fn reset(&mut self);
//     fn has_error(&self) -> bool;
// }

pub struct ErrorReporter {
    program_error: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self {
            program_error: false,
        }
    }

    pub fn error(&mut self, line: usize, r#type: &str, message: &str) {
        println!("[line {}] {} error : {}", line, r#type, message);
        self.program_error = true;
    }

    pub fn reset(&mut self) {
        self.program_error = false;
    }

    pub fn has_error(&self) -> bool {
        self.program_error
    }
}
