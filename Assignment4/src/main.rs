mod user;
mod errors;
mod logger;

use user::User;
use logger::ConsoleLogger;

fn main() {
    let console_logger = ConsoleLogger;

    let mut alice = User::new("Alice", "pass123", &console_logger);
    let mut bob = User::new("Bob", "bobpass", &console_logger);

    match alice.check_balance("pass123") {
        Ok(balance) => println!("Alice Balance: {}", balance),
        Err(e) => println!("Error: {:?}", e),
    }

    match alice.transfer_funds("pass123", 300, &mut bob) {
        Ok(()) => println!("Transfer successful."),
        Err(e) => println!("Transfer failed: {:?}", e),
    }

    match bob.check_balance("bobpass") {
        Ok(balance) => println!("Bob Balance: {}", balance),
        Err(e) => println!("Error: {:?}", e),
    }

    alice.print_history();
    bob.print_history();
}
