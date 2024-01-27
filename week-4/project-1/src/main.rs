use std::io;

fn main () {
   
   let mut input1 = String::new();
   let mut input2 = String::new();

   println!("Enter first distance: ");
   io::stdin().read_line(&mut input1).expect("Not a valid number");
   let d1:f32 = input1.trim().parse().expect("Not a valid number");

   println!("Enter first time: ");
   io::stdin().read_line(&mut input2).expect("Not a valid number");
   let t1:f32 = input2.trim().parse().expect("Not a valid number");

   let c1:f32 = d1 * 1.609;
   let s1 = c1 / t1;
   println!("The speed is: {}", s1);

   // Second Question
   let mut input3 = String::new();
   let mut input4 = String::new();

   println!("Enter second distance: ");
   io::stdin().read_line(&mut input3).expect("Not a valid number");
   let d2:f32 = input3.trim().parse().expect("Not a valid nuber");

   println!("Enter second time");
   io::stdin().read_line(&mut input4).expect("Not a valid number");
   let t2:f32 = input4.trim().parse().expect("Not a valid number");

   let c2:f32 = d2 * 1.609;
   let s2 = c2 / t1;
   println!("The speed is: {}", s2);
}