// src/atm/drop.rs
use crate::atm::cash::Cash;

impl Drop for Cash {
    fn drop(&mut self) {
        println!("ATM shutting down. Releasing cash at {:p}", self.total);
    }
}
