fn main() {
	// Given values
	let principal:f64 = 520_000_000.0;
	let rate:f64 = 10.0;
	let years:f64 = 5.0;

	// Calculate amount using the formula: A = P * (1 + R/100)^n
	let amount:f64  = principal * (1.0 + rate / 100.0).powf(years);

	// Compound Interest = A - P
	let compound_interest = amount - principal;
	println!("Compound Interest is {} naira", compound_interest);
}