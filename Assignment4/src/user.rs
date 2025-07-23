use crate::errors::WalletError;
use crate::logger::Logger;

pub struct User<'a> {
    pub username: String,
    password: String,
    balance: u32,
    history: Vec<String>,
    logger: &'a dyn Logger,
}

impl<'a> User<'a> {
    pub fn new(username: &str, password: &str, logger: &'a dyn Logger) -> Self {
        logger.log("User account created.");
        User {
            username: username.to_string(),
            password: password.to_string(),
            balance: 1000,
            history: vec![],
            logger,
        }
    }

    pub fn check_balance(&self, password: &str) -> Result<u32, WalletError> {
        if self.password != password {
            return Err(WalletError::IncorrectPassword);
        }
        self.logger.log("Balance checked.");
        Ok(self.balance)
    }

    pub fn transfer_funds(
        &mut self,
        password: &str,
        amount: u32,
        to_user: &mut User,
    ) -> Result<(), WalletError> {
        if self.password != password {
            return Err(WalletError::IncorrectPassword);
        }

        if self.balance < amount {
            return Err(WalletError::InsufficientFunds);
        }

        self.balance -= amount;
        to_user.balance += amount;

        let msg = format!("Transferred {} to {}", amount, to_user.username);
        self.history.push(msg.clone());
        to_user.history.push(format!("Received {} from {}", amount, self.username));
        self.logger.log(&msg);
        Ok(())
    }

    pub fn print_history(&self) {
        println!("--- Transaction History for {} ---", self.username);
        for record in &self.history {
            println!("{}", record);
        }
    }
}
