use rust_decimal::{Decimal, MathematicalOps};

struct Loan {
    loan_amount: Decimal,
    annual_percentage_rate: Decimal,
    loan_term: i32,
}

struct Payment {
    payment_number: i32,
    payment_amount: Decimal,
    amount_towards_interest: Decimal,
    amount_towards_principal: Decimal,
    remaining_balance: Decimal,
}


fn main() {

    let test_loan = Loan {
        loan_amount: Decimal::from(1000),
        annual_percentage_rate: Decimal::from(20),
        loan_term: 24,
    };

    println!("Calculating the amortization schedule on a ${:2} loan at {}% APR for a term of {} months.", &test_loan.loan_amount, &test_loan.annual_percentage_rate, &test_loan.loan_term);


    let loan_payments = generate_amortization_schedule(test_loan);

    for payment in &loan_payments {
        println!("Payment # {}: Payment Amount: ${:.2}, Interest Amount: ${:.2}, Principal Amount: ${:.2}, Remaining Balance: ${:.2}", payment.payment_number, payment.payment_amount, payment.amount_towards_interest, payment.amount_towards_principal, payment.remaining_balance);
    }
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
        amount_towards_interest,
        amount_towards_principal,
        remaining_balance,
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
            amount_towards_interest,
            amount_towards_principal,
            remaining_balance,
        };

        payments.push(payment);
    }

    return payments;
}


