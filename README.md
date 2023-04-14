# Loan Amortization Generator

This Rust project generates a loan amortization schedule based on a given loan amount, annual percentage rate, and loan term in months.

## How to Use
There are two options on how to use the program:
1. Pass no command line arguments and the program will prompt you for the information needed.
2. Pass 3 command line arguments in the following order: loan amount, annual percentage rate, and the loan term (in months).

### Command Line Argument Usage Example
The below example will give the program a loan of $1000.00 with an APR of 24.99% and a term of 24 months.

- macOS/Linux:
    ```shell
    ./loan-amortization 1000.00 24.99 24
    ```
- Windows:
    ```shell
    ./loan-amortization.exe 1000.00 24.99 24
    ```
  
