use std::io;

fn main() {
    // Prompt the user for experience and age
    println!("Is the employee experienced? (yes/no): ");
    let mut experience_str = String::new();
    io::stdin().read_line(&mut experience_str).expect("Failed to read input");
    let experience = experience_str.trim().to_lowercase();

    println!("Enter the employee's age: ");
    let mut age_str = String::new();
    io::stdin().read_line(&mut age_str).expect("Failed to read input");
    let age: u32 = age_str.trim().parse().expect("Please enter a valid age");

    // Determine incentive
    let incentive: f64;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000.0;
        } else if age >= 30 && age < 40 {
            incentive = 1_480_000.0;
        } else if age < 28 {
            incentive = 1_300_000.0;
        } else {
            // Handle ages between 28 and 29
            incentive = 1_300_000.0; // You can adjust this as needed
        }
    } else {
        incentive = 100_000.0;
    }

    println!("\nThe employee's annual incentive is: ₦{:.2}", incentive);
}


