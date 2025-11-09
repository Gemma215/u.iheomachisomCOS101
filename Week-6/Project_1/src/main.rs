use std::io;

fn main() {
    println!("       Welcome to the Menu");
    println!("P = Poundo Yam / Edinkaiko Soup - ₦3,200");
    println!("F = Fried Rice & Chicken        - ₦3,000");
    println!("A = Amala & Ewedu Soup          - ₦2,500");
    println!("E = Eba & Egusi Soup            - ₦2,000");
    println!("W = White Rice & Stew           - ₦2,500");
    let mut total = 0;

    loop {
        println!("Enter your choice (P/F/A/E/W) or 'Q' to finish:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice = choice.trim().to_uppercase();

        if choice == "Q" {
            break;
        }

        let (item, price_per_unit) = if choice == "P" {
            ("Poundo Yam / Edinkaiko Soup", 3200)
        } else if choice == "F" {
            ("Fried Rice & Chicken", 3000)
        } else if choice == "A" {
            ("Amala & Ewedu Soup", 2500)
        } else if choice == "E" {
            ("Eba & Egusi Soup", 2000)
        } else if choice == "W" {
            ("White Rice & Stew", 2500)
        } else {
            println!("Invalid choice! Please try again.");
            continue;
        };

        println!("Enter quantity for {}:", item);

        let mut qty_input = String::new();
        io::stdin()
            .read_line(&mut qty_input)
            .expect("Failed to read quantity");

        let quantity: i32 = match qty_input.trim().parse() {
            Ok(num) if num > 0 => num,
            _ => {
                println!("Invalid quantity! Please enter a positive number.");
                continue;
            }
        };

        let item_total = price_per_unit * quantity;
        total += item_total;

        println!(
            "Added {} x {} = ₦{}",
            quantity, item, item_total
        );
    }

    println!("Your total bill is: ₦{}", total);
    println!("Thank you for your order!");

}
