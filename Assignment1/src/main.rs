mod atm;
use atm::cash::Cash;
use atm::errors::CashError;

fn main() {
    println!("ATM starting...");

    let mut atm_cash = Cash::new(1000);

    let stack_number = 42;
    println!("Stack variable at: {:p}", &stack_number);

    match atm_cash.withdraw(300) {
        Ok(_) => println!("First withdrawal OK."),
        Err(CashError::InsufficientFunds { available, requested }) => {
            println!("Error: You requested {}, but only {} is available.", requested, available);
        }
        Err(CashError::InvalidAmount(amount)) => {
            println!("Error: Invalid withdrawal amount: {}", amount);
        }
    }

    match atm_cash.withdraw(800) {
        Ok(_) => println!("Second withdrawal OK."),
        Err(CashError::InsufficientFunds { available, requested }) => {
            println!("Error: You requested {}, but only {} is available.", requested, available);
        }
        Err(CashError::InvalidAmount(amount)) => {
            println!("Error: Invalid withdrawal amount: {}", amount);
        }
    }

    println!("ATM operations done.");
}
