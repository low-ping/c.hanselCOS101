// A rust program to perform calculations.
use std::io;
fn trapezium(h:f32, b1:f32, b2:f32) {
    

    let area = h / 2.0 * (b1 * b2);
    println!("The area of the trapezium is {}", area);
}
fn rhombus(d1:f32, d2:f32) {
    
   let area_1 = d1 * d2 * 0.5 ;
   println!("The area of the rhombus is {}", area_1);
}
fn parallelogram(b3:f32, a:f32) {
   
      let area_2 = b3 * a;
      println!("The area of the parralelogram is {}", area_2);
} 
fn cube(l:f32) {
    
    let area_3 = 6.0 * l.powf(2.0);
    println!("The area of the cube is {}", area_3);      
          
} 
fn cylinder(r:f32, h1:f32) {
    
    let area_4 = 22.0/7.0 * r.powf(2.0) * h1;
    println!("The area of the cylinder is {}", area_4); 
}
         
fn main() {
    
    println!("Welcome to a calculator
             click 1. for trapezium calculations
             click 2. for rhombus calculations
             click 3. for parallelogram calculations
             click 4. for cube calculation
             click 5. for cylinder calculations
             click 6. to quit
             ");
    
    println!("Enter your choice: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("not a valid option");
    let mut choice:u32 = input1.trim().parse().expect("not a valid option");

    if choice == 1 {
        //Trapezium
        println!("Enter the height of the trapezium");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("not a valid height");
        let h:f32 = input2.trim().parse().expect("not a valid height");

        println!("Enter the first base of the trapezium");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("not a valid height");
        let b1:f32 = input3.trim().parse().expect("not valid height");

        println!("Enter the second base of the trapezium");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("not a valid height");
        let b2:f32 = input4.trim().parse().expect("not a valid height");

        trapezium(h, b1, b2);

    } else if choice == 2 {
        //Rhombus
        println!("Enter the first diagonal of the rhombus");
        let mut input5 = String::new();
        io::stdin().read_line(&mut input5).expect("not a valid diagonal");
        let d1:f32 = input5.trim().parse().expect("not a valid diagonal");

        println!("Enter the second diagonal of the rhombus");
        let mut input6 = String::new();
        io::stdin().read_line(&mut input6).expect("not a valid height");
        let d2:f32 = input6.trim().parse().expect("not valid height");

        rhombus(d1, d2);
    } else if choice == 3 {
        //Parallelogram
        println!("Enter the first diagonal:");
        let mut input8 = String::new();
        io::stdin().read_line(&mut input8).expect("not a valid diagonal");
        let b3:f32 = input8.trim().parse().expect("not a valid diagonal");

        println!("Enter the second diagonal of the parallelogram:");
        let mut input9 = String::new();
        io::stdin().read_line(&mut input9).expect("not a valid diagonal");
        let a:f32 = input9.trim().parse().expect("not a valid diaagonal");

        parallelogram(b3, a);

    } else if choice == 4 {
        //Cube
        println!("Enter the length of the cube");
        let mut input10 = String::new();
        io::stdin().read_line(&mut input10).expect("not a valid length");
        let l:f32 = input10.trim().parse().expect("not a valid length");

        cube(l);
    } else if choice == 5 {
        //Cylinder
        println!("Enter the radius of the cylinder");
        let mut input11 = String::new();
        io::stdin().read_line(&mut input11).expect("not a valid length");
        let r:f32 = input11.trim().parse().expect("not a valid length");

        println!("Enter the height of the cylinder");
        let mut input12 = String::new();
        io::stdin().read_line(&mut input12).expect("not a valid height");
        let h1:f32 = input12.trim().parse().expect("not a valid height");

        cylinder(r, h1);
    } else if choice == 6 {
        println!("Quit");
    } else {
        println!("Oops, we can't help you.");
    }


    

    
    
}       
