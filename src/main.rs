use rust_decimal::{Decimal, MathematicalOps};
use serde::Serialize;
use std::fs::File;
use rust_decimal::prelude::FromPrimitive;
use csv::{WriterBuilder};

struct Loan {
    loan_amount: Decimal,
    annual_percentage_rate: Decimal,
    loan_term: i32,
}

#[derive(Serialize)]
struct Payment {
    payment_number: i32,
    payment_amount: Decimal,
    amount_towards_interest: Decimal,
    amount_towards_principal: Decimal,
    remaining_balance: Decimal,
}

fn main() {
    let test_loan = Loan {
        loan_amount: Decimal::from(45000),
        annual_percentage_rate: Decimal::from_f32(6.00).unwrap(),
        loan_term: 60,
    };

    println!("Calculating the amortization schedule on a ${:2} loan at {}% APR for a term of {} months.", &test_loan.loan_amount, &test_loan.annual_percentage_rate, &test_loan.loan_term);

    let loan_payments = generate_amortization_schedule(test_loan);

    for payment in &loan_payments {
        println!("Payment # {}: Payment Amount: ${:.2}, Interest Amount: ${:.2}, Principal Amount: ${:.2}, Remaining Balance: ${:.2}", payment.payment_number, payment.payment_amount, payment.amount_towards_interest, payment.amount_towards_principal, payment.remaining_balance);
    }

    println!("Writing to CSV file.");
    output_csv(loan_payments);
}

fn calculate_monthly_payment(loan: &Loan) -> Payment {
    let monthly_interest_rate: Decimal = (loan.annual_percentage_rate / Decimal::from(100)) / Decimal::from(12);
    let denominator: Decimal = Decimal::from(1) - (Decimal::from(1) + monthly_interest_rate).powi((-loan.loan_term).into());
    let monthly_payment: Decimal = (monthly_interest_rate * loan.loan_amount) / denominator;
    let amount_towards_interest: Decimal = monthly_interest_rate * loan.loan_amount;
    let amount_towards_principal: Decimal = monthly_payment - amount_towards_interest;
    let remaining_balance: Decimal = loan.loan_amount - amount_towards_principal;
    let payment_number: i32 = 1;

    return Payment {
        payment_number,
        payment_amount: monthly_payment,
        amount_towards_interest: amount_towards_interest,
        amount_towards_principal: amount_towards_principal,
        remaining_balance: remaining_balance,
    };
}

fn generate_amortization_schedule(loan: Loan) -> Vec<Payment> {
    // Vector to hold all of our payments
    let mut payments = Vec::new();

    // Get the starting data
    let initial_data = calculate_monthly_payment(&loan);
    let monthly_interest_rate = loan.annual_percentage_rate / Decimal::from(1200);
    let monthly_payment = initial_data.payment_amount;
    let mut remaining_balance: Decimal = initial_data.remaining_balance;

    // Add our initial payment to the vector
    payments.push(initial_data);

    for payment_number in 2..=loan.loan_term {
        let amount_towards_interest = remaining_balance * monthly_interest_rate;
        let amount_towards_principal = monthly_payment - amount_towards_interest;

        remaining_balance -= amount_towards_principal;

        let payment = Payment {
            payment_number: payment_number,
            payment_amount: monthly_payment,
            amount_towards_interest: amount_towards_interest,
            amount_towards_principal: amount_towards_principal,
            remaining_balance: remaining_balance,
        };

        payments.push(payment);
    }

    return payments;
}

fn output_csv(payments: Vec<Payment>) {
    let file = File::create("output.csv").unwrap();
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);

    // Add header
    writer.serialize((
        "PAYMENT NUMBER",
        "PAYMENT AMOUNT",
        "INTEREST AMOUNT",
        "PRINCIPAL AMOUNT",
        "REMAINING BALANCE"
    )).expect("An error was encountered when serializing the header.");

    for row in payments {
        let payment_amount_str = format!("{:.2}", row.payment_amount);
        let amount_towards_interest_str = format!("{:.2}", row.amount_towards_interest);
        let amount_towards_principal_str = format!("{:.2}", row.amount_towards_principal);
        let remaining_balance_str = format!("{:.2}", row.remaining_balance);

        writer.serialize((
            row.payment_number,
            payment_amount_str,
            amount_towards_interest_str,
            amount_towards_principal_str,
            remaining_balance_str,
        )).expect("An error occurred when writing the data.");
    }
}