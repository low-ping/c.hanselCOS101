//Rust program for the school of registry

use std::io;

fn main() {
let mut num = 0;
while num < 150 {
    println!("");
num = num +1;
    
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();

    println!("Enter Name ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let name = input1.trim();

    println!("Enter Current level ");
    io::stdin().read_line(&mut input2).expect("Not a valid integer type");
    let level:i32 = input2.trim().parse().expect("Not a valid integer type");

    println!("Enter Current CGPA ");
    io::stdin().read_line(&mut input3).expect("Not a valid float type");
    let cgpa:f32 = input3.trim().parse().expect("Not a valid float type");

    println!("Enter Email: ");
    io::stdin().read_line(&mut input4).expect("Not a valid email");
    let email = input4.trim();

    println!("Enter Department ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let dept = input5.trim();

    println!("Enter State of Origin ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let state_of_origin = input6.trim();

    println!("Are you a class rep (enter true of false) ");
    io::stdin().read_line(&mut input7).expect("Not a valid boolean");
    let reps:bool = input7.trim().parse().expect("Not a valid boolean");

    if level != 100 && cgpa > 4.0 && cgpa <= 5.0 && reps == true {
        println!("You can vote");

    } else {
        println!("You are not eligible to vote");
    }
    println!("Name: {} \nEmail: {} \nDepartment: {} \nState of Origin:
    {}", name, email, dept, state_of_origin);










}

}
