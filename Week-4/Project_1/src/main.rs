use std::io;


fn main() {
    //Prompt the user for input
    println!("Enter the values of a, b, c:");

    //Read input values
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    io::stdin().read_line(&mut a_str).expect("Failed to read input");
    io::stdin().read_line(&mut b_str).expect("Failed to read input");
    io::stdin().read_line(&mut c_str).expect("Failed to read input");


let a: f64 = a_str.trim().parse().expect("Please enter a valid number");
let b: f64 = b_str.trim().parse().expect("Please enter a valid number");
let c: f64 = c_str.trim().parse().expect("Please enter a valid number");


let discriminant = b * b - 4.0 * a * c;

println!("\nDiscriminant = {}",discriminant);


if discriminant > 0.0 {
    let root1 = (-b + discriminant.sqrt()) /(2.0 * a);
    let root2cargo struct ; = (-b - discriminant.sqrt()) /(2.0 * a);
    println!("Two distinct real roots:");
    println!("Root 1 = {}", root1);
    println!("Root 2 = {}", root2);
} else if discriminant == 0.0 {
    let root = -b / (2.0 * a);
    println!("One real root:");
    println!("Root = {}",root);
}else {
    println!("No real roots(discriminat is negative).");
}

}
