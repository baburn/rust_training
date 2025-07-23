mod eligibility;
mod errors;

use eligibility::check_loan_eligibility;
use std::io;

fn main() {
    println!("--- Loan Approval System ---");

    // User input
    let income = input("Enter your income: ").parse::<u32>().unwrap_or(0);
    let age = input("Enter your age: ").parse::<u32>().unwrap_or(0);
    let loan_amount = input("Enter the desired loan amount: ").parse::<u32>().unwrap_or(0);

    let co_applicant = input("Do you have a co-applicant? (y/n): ").to_lowercase();
    let co_age = if co_applicant.trim() == "y" {
        Some(input("Enter co-applicant's age: ").parse::<u32>().unwrap_or(0))
    } else {
        None
    };

    match check_loan_eligibility(income, age, loan_amount, co_age) {
        Ok(()) => println!("✅ Loan Approved!"),
        Err(e) => println!("❌ Loan Denied: {}", e),
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
