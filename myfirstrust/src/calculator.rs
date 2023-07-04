use std::io::{self, Write};

pub fn calapp() {
    loop {
        println!("Welcome to Rust Calculator!");
        println!("Please select an operation:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");

        let choice = read_input("Enter your choice: ");
        let choice = choice.trim().parse::<u32>();

        match choice {
            Ok(1) => perform_addition(),
            Ok(2) => perform_subtraction(),
            Ok(3) => perform_multiplication(),
            Ok(4) => perform_division(),
            Ok(5) => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }

    println!("Thank you for using Rust Calculator!");
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
}

fn perform_addition() {
    let num1 = read_input("Enter the first number: ");
    let num2 = read_input("Enter the second number: ");

    let num1 = num1.trim().parse::<f64>();
    let num2 = num2.trim().parse::<f64>();

    match (num1, num2) {
        (Ok(n1), Ok(n2)) => {
            let result = n1 + n2;
            println!("Result: {}", result);
        }
        _ => println!("Invalid input. Please enter valid numbers."),
    }
}

fn perform_subtraction() {
    let num1 = read_input("Enter the first number: ");
    let num2 = read_input("Enter the second number: ");

    let num1 = num1.trim().parse::<f64>();
    let num2 = num2.trim().parse::<f64>();

    match (num1, num2) {
        (Ok(n1), Ok(n2)) => {
            let result = n1 - n2;
            println!("Result: {}", result);
        }
        _ => println!("Invalid input. Please enter valid numbers."),
    }
}

fn perform_multiplication() {
    let num1 = read_input("Enter the first number: ");
    let num2 = read_input("Enter the second number: ");

    let num1 = num1.trim().parse::<f64>();
    let num2 = num2.trim().parse::<f64>();

    match (num1, num2) {
        (Ok(n1), Ok(n2)) => {
            let result = n1 * n2;
            println!("Result: {}", result);
        }
        _ => println!("Invalid input. Please enter valid numbers."),
    }
}

fn perform_division() {
    let num1 = read_input("Enter the first number: ");
    let num2 = read_input("Enter the second number: ");

    let num1 = num1.trim().parse::<f64>();
    let num2 = num2.trim().parse::<f64>();

    match (num1, num2) {
        (Ok(n1), Ok(n2)) => {
            if n2 != 0.0 {
                let result = n1 / n2;
                println!("Result: {}", result);
            } else {
                println!("Error: Division by zero is not allowed.");
            }
        }
        _ => println!("Invalid input. Please enter valid numbers."),
    }
}
