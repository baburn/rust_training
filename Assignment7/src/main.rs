mod transaction;
mod processor;

use std::sync::mpsc;
use std::thread;
use transaction::Transaction;
use processor::{Processor, BankProcessor};

fn main() {
    let (tx, rx) = mpsc::channel::<Transaction>();

    let processor = BankProcessor;
    let handle = thread::spawn(move || {
        while let Ok(transaction) = rx.recv() {
            processor.process(transaction);
        }
    });

    tx.send(Transaction::Deposit {
        account_id: 1001,
        amount: 500,
    }).unwrap();

    tx.send(Transaction::Withdraw {
        account_id: 1001,
        amount: 200,
    }).unwrap();

    tx.send(Transaction::Transfer {
        from: 1001,
        to: 1002,
        amount: 300,
    }).unwrap();

    // Allow some time for the thread to process
    drop(tx); // closes the channel
    handle.join().unwrap();
}
