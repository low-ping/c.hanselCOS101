use std::io::Read;
use std::io;
fn main(){
	println!("Welcome to glo.com
		click 1. For Admin
		click 2. For For Project Manager
		click 3. For staff
		click 4. For customer
		click 5. for vendor
		click 6. to quit");
	let mut input: String = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input");
	let choice:u32= input.trim().parse().expect("Failed to read input");

	if choice == 1 {
		calm();
	} else if choice == 2 {
		say();
	} else if choice == 3 {
		hello();
	} else if choice == 4 {
		hi();
	} else if choice == 5 {
		good();
	} else {
		println!("Not among Options");
	}


fn hello() {
	let mut file = std::fs::File::open("staff_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);
}
fn hi() {
	let mut file = std::fs::File::open("customer_table_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);
}
fn good() {
	let mut file = std::fs::File::open("dataplan_table_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);
}
fn calm() {
	let mut file = std::fs::File::open("globacom_dbase_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);
}
fn say() {
	let mut file = std::fs::File::open("project_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);
}

}