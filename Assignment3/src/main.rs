mod logger;

use logger::{Logger, FileLogger, ConsoleLogger};

fn main() {
    let mut file_logger = FileLogger::new("log.txt");
    let console_logger = ConsoleLogger;

    file_logger.log("Writing to file...");
    console_logger.log("Writing to console...");

    // Simulate critical failure
    //file_logger.simulate_failure(); // Uncomment to test panic!
}
