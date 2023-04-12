struct Loan {
    loan_amount: f32,
    annual_percentage_rate: f32,
    loan_term: i32,
}

struct Payment {
    payment_number: i32,
    payment_amount: f32,
    amount_towards_interest: f32,
    amount_towards_principal: f32,
    remaining_balance: f32,
}

fn main() {
    let test_loan = Loan {
        loan_amount: 1000.00,
        annual_percentage_rate: 20.00,
        loan_term: 24,
    };

    println!("Calculating the amortization schedule on a ${:2} loan at {}% APR for a term of {} months.", &test_loan.loan_amount, &test_loan.annual_percentage_rate, &test_loan.loan_term);


    let loan_payments = generate_amortization_schedule(test_loan);

    for payment in &loan_payments {
        println!("Payment # {}: Payment Amount: ${:.2}, Interest Amount: ${:.2}, Principal Amount: ${:.2}, Remaining Balance: ${:.2}", payment.payment_number, payment.payment_amount, payment.amount_towards_interest, payment.amount_towards_principal, payment.remaining_balance);
    }
}

fn calculate_monthly_payment(loan: &Loan) -> Payment {
    let monthly_interest_rate: f32 = (loan.annual_percentage_rate / 100.0) / 12.0;
    let denominator: f32 = 1.0 - (1.0 + monthly_interest_rate).powi(-loan.loan_term);
    let monthly_payment: f32 = (monthly_interest_rate * loan.loan_amount) / denominator;
    let amount_towards_interest: f32 = monthly_interest_rate * loan.loan_amount;
    let amount_towards_principal: f32 = monthly_payment - amount_towards_interest;
    let remaining_balance: f32 = loan.loan_amount - amount_towards_principal;
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
    let monthly_interest_rate = loan.annual_percentage_rate / (12.00 * 100.00);
    let monthly_payment = initial_data.payment_amount;
    let mut remaining_balance: f32 = initial_data.remaining_balance;

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

