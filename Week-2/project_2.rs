fn main(){
	let tt:f64 = 450_000.00 * 2.0; // tt being the amount of Toshiba
	let tm:f64 = 1_500_000.00; // tm being total amount of Mac
	let th:f64 = 750_000.00 * 3.0; // th being total amount of Hp
	let td:f64 = 2_850_000.00 * 3.0; // td being total amount of Dell
	let ta:f64 = 250_000.00; // ta being total amount of Acer

	let s:f64 = tt + tm + th + td +ta; //ss being Total sum
	println!("The total sum is {}", s);
	let avg:f64 = s / 10.0; // avg being average amount 
	println!("The average amount is {}", avg);
}