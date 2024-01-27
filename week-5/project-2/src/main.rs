//A rust program to  determine annual incentive
 use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter level true or false if experience");
    io::stdin().read_line(&mut input1).expect("An error message");
    let experience:bool = input1.trim().parse().expect("An error message");

    println!("Enter age: ");
    io::stdin().read_line(&mut input2).expect("An error message");
    let age:i32 = input2.trim().parse().expect("An error message");

    if age >= 40 && experience == true {
        println!("Incentive is 1560000");
    }else if age >= 30 && experience == true {
        println!("Incentive is 1480000");
    }else if age < 28 && experience == true {
        println!("Incentive is 1300000");
    }else {
        println!("Incentive is 100000");
    }
    
    
}