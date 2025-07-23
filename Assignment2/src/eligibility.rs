use crate::errors::{LoanError, AgeError, IncomeError};

pub fn check_loan_eligibility(
    income: u32,
    age: u32,
    loan_amount: u32,
    co_applicant_age: Option<u32>,
) -> Result<(), LoanError> {
    // Check primary applicant
    validate_age(age)?;
    validate_income(income, loan_amount)?;

    // Optional co-applicant
    if let Some(co_age) = co_applicant_age {
        if let Err(e) = validate_age(co_age) {
            println!("⚠️ Warning: Co-applicant is not eligible: {}", e);
        }
    }

    Ok(())
}

fn validate_age(age: u32) -> Result<(), LoanError> {
    match age {
        21..=60 => Ok(()),
        _ => Err(LoanError::Age(AgeError::InvalidAge(age))),
    }
}

fn validate_income(income: u32, loan_amount: u32) -> Result<(), LoanError> {
    if income >= loan_amount / 2 {
        Ok(())
    } else {
        Err(LoanError::Income(IncomeError::InsufficientIncome {
            income,
            required: loan_amount / 2,
        }))
    }
}
