// processor.rs
use crate::transaction::Transaction;

pub trait Processor {
    fn process(&self, tx: Transaction);
}

pub struct BankProcessor;

impl Processor for BankProcessor {
    fn process(&self, tx: Transaction) {
        match tx {
            Transaction::Deposit { account_id, amount } => {
                println!("Deposited {} to account {}", amount, account_id);
            }
            Transaction::Withdraw { account_id, amount } => {
                println!("Withdrew {} from account {}", amount, account_id);
            }
            Transaction::Transfer { from, to, amount } => {
                println!("Transferred {} from account {} to {}", amount, from, to);
            }
        }
    }
}
