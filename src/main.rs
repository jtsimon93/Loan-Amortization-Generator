use std::{
    env, io,
    fs::File,
    process::exit,
};
use rust_decimal::{
    Decimal, MathematicalOps,
    prelude::FromPrimitive,
};
use serde::Serialize;
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

    // Get CLI arguments
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // No arguments, prompt user for input
        let loan = prompt_user_for_loan_info();

        output_menu(loan);
    } else if args.len() == 2 {
        // Only 1 argument supplied, see if user is requesting help
        if args[1] == "help" {
            output_help_info();
        } else {
            println!("Invalid arguments.");
            exit(1);
        }
    } else if args.len() == 4 {
        // Process arguments
        let loan_amount_input = process_float_argument(&args[1], "loan amount");
        let annual_percentage_rate_input = process_float_argument(&args[2], "annual percentage rate");
        let loan_term_input = process_integer_argument(&args[3], "loan term");

        let loan = Loan {
            loan_amount: Decimal::from_f32(loan_amount_input).unwrap(),
            annual_percentage_rate: Decimal::from_f32(annual_percentage_rate_input).unwrap(),
            loan_term: loan_term_input,
        };

        output_menu(loan);
    } else {
        println!("Invalid arguments.");
    }
}

fn prompt_user_for_loan_info() -> Loan {
    let loan_amount = get_float_input("Please enter the loan amount:");
    let annual_percentage_rate = get_float_input("Please enter the annual percentage rate:");
    let loan_term = get_integer_input("Please enter the loan term (in months):");

    let loan = Loan {
        loan_amount: Decimal::from_f32(loan_amount).unwrap(),
        annual_percentage_rate: Decimal::from_f32(annual_percentage_rate).unwrap(),
        loan_term,
    };

    return loan;
}

fn get_float_input(prompt: &str) -> f32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        match input.trim().parse::<f32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn get_integer_input(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please try again."),
        }
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

fn process_float_argument(argument: &str, argument_name: &str) -> f32 {
    let prompt_format = format!("The supplied {argument_name} was invalid. Please enter a valid {argument_name}", argument_name = argument_name);
    let prompt = prompt_format.as_str();
    loop {
        match argument.trim().parse::<f32>() {
            Ok(num) => return num,
            Err(_) => return get_float_input(&prompt),
        }
    }
}

fn process_integer_argument(argument: &str, argument_name: &str) -> i32 {
    let prompt_format = format!("The supplied {argument_name} was invalid. Please enter a valid {argument_name}", argument_name = argument_name);
    let prompt = prompt_format.as_str();
    loop {
        match argument.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => return get_integer_input(&prompt),
        }
    }
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
            payment_number,
            payment_amount: monthly_payment,
            amount_towards_interest,
            amount_towards_principal,
            remaining_balance,
        };

        payments.push(payment);
    }

    return payments;
}

fn output_csv(loan: Loan) {
    let payments = generate_amortization_schedule(loan);

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

fn output_to_terminal(loan: Loan) {
    println!("Calculating the amortization schedule on a ${:2} loan at {}% APR for a term of {} months.", &loan.loan_amount, &loan.annual_percentage_rate, &loan.loan_term);

    let loan_payments = generate_amortization_schedule(loan);

    for payment in &loan_payments {
        println!("Payment # {:<3}: Payment Amount: ${:>8.2}, Interest Amount: ${:>9.2}, Principal Amount: ${:>9.2}, Remaining Balance: ${:>9.2}", payment.payment_number, payment.payment_amount, payment.amount_towards_interest, payment.amount_towards_principal, payment.remaining_balance);
    }
}

fn output_menu(loan: Loan) {
    println!();
    println!("----- OPERATIONS MENU -----");
    println!("1. Print amortization schedule to the screen.");
    println!("2. Export amortization schedule to CSV file.");
    println!("3. Help");
    println!("4. Exit");

    let option = get_integer_input("Please enter an option: ");

    if option == 1 {
        output_to_terminal(loan);
    } else if option == 2 {
        output_csv(loan);
    } else if option == 3 {
        output_help_info();
    } else if option == 4 {
        exit(0);
    } else {
        println!("An invalid option was selected.");
    }
}

fn output_help_info() {
    println!("---- LOAN AMORTIZATION SCHEDULE HELP -----");
    println!("Enter 0 arguments to have the program prompt you for the information. Otherwise supply the below arguments.");
    println!("Command line arguments: loan amount, annual percentage rate, loan term (in months)");
}
