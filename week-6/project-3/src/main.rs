//Rust program that display multiplication table of n numbers
use std::io;

fn main() {
    println!("Enter the amount of number you want to see:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid number");
    let n:u64 = input1.trim().parse().expect("Not a valid number");

    print!("");

    for a in 1..n+1 {
        for b in 1..n+1 {
            print!("{}\t", a*b);
        }
        println!("");
    }
}