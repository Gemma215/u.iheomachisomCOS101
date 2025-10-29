
fn main(){
	//name
	println!("Name?");
	let mut name = String::new(),
	io::stdin().read_line("&mut name").expect("Failed to read input");
	name = name.trim().parse().expect("Invalid input");

	//dob
	println!("Date of birth ?(dd/mm/yy) ");
	let mut dob = String::new();
	io::stdin().read_line("&mut dob").expect("Failed to read input");
	dob = dob.trim().parse().expect("Invalid input");


	// email.address
	println!("Email address");
	let mut email addresss = String::new();
	io::stdin().read_line("&mut email address").expect("Filed to read input");
	email address = email addresss.trim().parse().expeact("Invalid input");


}