//Rust program for Nigerian Researchers Guide..
use std::io;

fn main() {

for _n in 0..150 {


println!("RESEARCHERS PUBLICATION INPUT SYSTEMS");

let mut input1 = String::new();
println!("Enter name: ");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let name = input1.trim();


let mut input2 = String::new();
println!("Enter number of papers published ");
io::stdin().read_line(&mut input2).expect("Not a valid input");
let no_of_papers:u32 = input2.trim().parse().expect("Not a valid input");

println!("Your name is {}", name);

if no_of_papers > 3 && no_of_papers <5 {
    println!("Congratulations, Your incentive is 500,000");

} else if no_of_papers> 5 && no_of_papers < 10 {
    println!("Congratulations, Your incentive is 800,000");  

} else if no_of_papers >= 10 {
    println!("Congratulations, Your incentive is 1,000,000"); 
} else {
    println!("Congratulations, Your incentive is 100,000");
}
}
}