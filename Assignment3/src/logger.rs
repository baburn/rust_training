use std::fs::{OpenOptions, File};
use std::io::Write;

pub trait Logger {
    fn log(&self, message: &str);
}

pub struct FileLogger {
    file: File,
}

impl FileLogger {
    pub fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|e| {
                panic!("Critical Failure (file open): {}", e); // triggers panic
            });
        FileLogger { file }
    }

    pub fn simulate_failure(&mut self) {
        panic!("Disk full or permission denied"); // Simulating a crash
    }
}

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        let mut file_ref = &self.file;
        writeln!(file_ref, "{}", message).unwrap_or_else(|e| {
            panic!("Critical Failure (write): {}", e);
        });
    }
}

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("Console: {}", message);
    }
}
