use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the first co-efficient: ");
    io::stdin().read_line(&mut input1).expect("Display an error");
    let a:f32 = input1.trim().parse().expect("Display an error");

    println!("Enter the second co-efficient: ");
    io::stdin().read_line(&mut input2).expect("Display an error");
    let b:f32 = input2.trim().parse().expect("Display an error");

    println!("Enter the third co-efficient: ");
    io::stdin().read_line(&mut input3).expect("Display an error");
    let c:f32 = input3.trim().parse().expect("Display an error");


    let d:f32 = (b.powf(2.0)) - 4.0 * a * c ;


    // To decide the nature of the root
    if d > 0.0 {
        println!("The equation has distinct roots");
    } else if d == 0.0 {
        println!("The equation has one real root");
    } else if d < 0.0 {
        println!("The equation has no real root");
    } else{
        println!("The equation cannot be solved") 
    };
    //Solving for the root
    let x = -b + (d).sqrt();
    let x1 = x / (a * 2.0);
     println!("The value of x1 is: {}", x1);
    let y = -b - (d).sqrt();
    let y1 = y / (a * 2.0);
    println!("The value of x2 is: {}", y1)

}
