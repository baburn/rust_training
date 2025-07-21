#[derive(Debug)]
enum LoanError {
    AgeError,
    IncomeError,
}

struct LoanApplication {
    income: u32,
    age: u8,
    loan_amount: u32,
    co_applicant_income: Option<u32>,
}

impl LoanApplication {
    fn total_income(&self) -> u32 {
        self.income + self.co_applicant_income.unwrap_or(0)
    }

    fn check_eligibility(&self) -> Result<(), LoanError> {
        if self.age < 21 {
            return Err(LoanError::AgeError);
        }
        if self.total_income() < 25000 {
            return Err(LoanError::IncomeError);
        }
        Ok(())
    }
}

fn main() {
    let applications = vec![
        LoanApplication {
            income: 20000,
            age: 25,
            loan_amount: 100000,
            co_applicant_income: Some(10000),
        },
        LoanApplication {
            income: 15000,
            age: 19,
            loan_amount: 50000,
            co_applicant_income: None,
        },
        LoanApplication {
            income: 30000,
            age: 30,
            loan_amount: 200000,
            co_applicant_income: None,
        },
    ];

    let mut iter = applications.into_iter();
    while let Some(app) = iter.next() {
        match app.check_eligibility() {
            Ok(()) => {
                println!("Loan approved for age: {}, income: {}", app.age, app.total_income());
            }
            Err(e) => match e {
                LoanError::AgeError => println!("Loan denied: Age must be at least 21 (got {}).", app.age),
                LoanError::IncomeError => println!(
                    "Loan denied: Combined income ({}) is below threshold.",
                    app.total_income()
                ),
            },
        }
    }
}
