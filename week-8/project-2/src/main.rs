// A rust program for Ernest and Young(EY)
use std::io;
fn main() {
    let mut name : Vec<String> = Vec::new();
    let mut years_of_experience: Vec<usize> = Vec::new();

    println!("How many developers are you scouting for:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid number");
    let n= input1.trim().parse().expect("not a valid number");

    for _i in 0..n{
        println!("Enter name: ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let developers_name = input2.trim().to_string();
        name.push(developers_name);

        println!("Enter years of experience");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Not a valid number");
        let d_years_of_experience:usize = input3.trim().parse().expect("not a valid number");
        years_of_experience.push(d_years_of_experience);
    }

    for count in 0..n{
        println!("The developers {} details is as follows ", count+1);
        
        println!("DEVELOPERS NAME: {} 
                YEARS OF EXPERIENCE: {}", name[count], years_of_experience[count]);
    }
    let years_of_experience = years_of_experience.iter().max();
        match years_of_experience{
           Some(max) => println!("The highest number of experience is {}", max),
           None => todo!(),
        }
        
}
