pub struct ErrorReporter {
    pub had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
        self.had_error = true;
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn reset(&mut self) {
        if self.had_error {
            self.had_error = false;
        }
    }
}
