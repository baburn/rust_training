// src/atm/transaction.rs
use crate::atm::cash::Cash;
use crate::atm::errors::CashError;

impl Cash {
    pub fn withdraw(&mut self, amount: u32) -> Result<(), CashError> {
        if amount == 0 {
            return Err(CashError::InvalidAmount(amount.try_into().unwrap()));
        }

        println!("Trying to withdraw {}...", amount);
        println!("Total cash before: {}", self.total);

        if *self.total >= amount {
            *self.total -= amount;
            println!("Withdrawal successful. Remaining: {}", self.total);
            Ok(())
        } else {
            Err(CashError::InsufficientFunds {
                available: *self.total,
                requested: amount,
            })
        }
    }
}
