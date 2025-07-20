// ATM Machine with Limited Cash (RAII + Memory Control)
struct Cash {
    total: Box<u32>, // stored on the heap
}

impl Cash {
    fn new(amount: u32) -> Self {
        println!(
            "Cash created at heap address: {:p}",
            &*Box::new(amount)
        );
        Cash {
            total: Box::new(amount),
        }
    }

    fn withdraw(&mut self, amount: u32) {
        println!("Trying to withdraw {}...", amount);
        println!("Total cash before: {}", self.total);

        if *self.total >= amount {
            *self.total -= amount;
            println!("Withdrawal successful. Remaining: {}", self.total);
        } else {
            println!("Not enough balance.");
        }

        println!(
            "Cash struct (on stack) at: {:p}, total (on heap) at: {:p}",
            &self,
            self.total
        );
    }
}

// RAII: Cash will be automatically dropped when it goes out of scope
impl Drop for Cash {
    fn drop(&mut self) {
        println!("ATM shutting down. Releasing cash at {:p}", self.total);
    }
}

fn main() {
    println!("ATM starting...");

    let mut atm_cash = Cash::new(1000); // heap allocation

    // stack variable example
    let stack_number = 42;
    println!("Stack variable at: {:p}", &stack_number);

    atm_cash.withdraw(300);
    atm_cash.withdraw(800); // this will fail

    println!("ATM operations done.");
}
