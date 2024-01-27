use std::io;

fn main() {
    println!("How many siblings do you have");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("not a valid number");
    let n:u32 = input1.trim().parse().expect("not a valid nuber");

    for _i in 1..n+1{
        println!("Enter the first name of each siblings");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("not a valid number");
        let _first_name = input2.trim();

        println!("Enter age of each siblings");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("not a valid string");
        let mut _age:u32 = input3.trim().parse().expect("not a valid string");


        

        if _age > 18  {
            println!("Is your sibling single (enter yes or no)");
            let mut input4 = String::new();
            io::stdin().read_line(&mut input4).expect("not a valid string");
            let _single = input4.trim();

            if _single == "yes" {
                println!("Is your sibling a student(enter yes or no)");
                let mut input6 = String::new();
                io::stdin().read_line(&mut input6).expect("not a valid string");
                let _student = input6.trim();

                if _student == "yes" {
                    println!("Enter university of sibling: ");
                    let mut input8 = String::new();
                    io::stdin().read_line(&mut input8).expect("not a valid string");
                    let _university = input8.trim();

                    println!("Enter the course of study of sibling: ");
                    let mut input9 = String::new();
                    io::stdin().read_line(&mut input9).expect("not a valid string");
                    let _course_of_study  = input9.trim();
                    
                    println!("The details of your sibling is as follows: ");
                    println!("First name: {} \n Age: {} \n Single: {} \n Student: {} \n Course of study: {}",_first_name, _age, _single, _student, _course_of_study );



                }

            } else if _single == "no" {
                println!("Do you have any offspring: ");
                let mut input10 = String::new();
                io::stdin().read_line(&mut input10).expect("not a valid string");
                let _no_of_offspring  = input10.trim();
                
                println!("What city do you live in");
                let mut input11 = String::new();
                io::stdin().read_line(&mut input11).expect("not a valid string");
                let _city = input11.trim();
                
                println!("The details of sibling is as follows: ");
                println!("First name: {} \n Age: {} \n Single: {} \n City of residence: {} ", _first_name, _age, _single, _city);



            }
        } else if _age < 18 {
            println!("have you written waec(enter yes or no)");
            let mut input12 = String::new();
            io::stdin().read_line(&mut input12).expect("not a valid string");
            let _waec_status = input12.trim();
            if _waec_status == "yes" {
                println!("Enter the secondary school attended");
                let mut input13 = String::new();
                io::stdin().read_line(&mut input13).expect("not a valid string");
                let _sec_school = input13.trim();

                println!("The detail of sibling is as follows: ");
                println!("First name: {} \n Age: {} \n Do you have waec result: {} \n Secondary school attended: {}", _first_name, _age, _waec_status, _sec_school);
            } 
        } else {
            println!("What class are you in currently:");
            let mut input14 = String::new();
            io::stdin().read_line(&mut input14).expect("not a valid string");
            let _current_class = input14.trim();

            println!("The details of siblings is as follows: ");
            println!("First name: {} \n Age: {} \n Current class in: {}", _first_name, _age, _current_class);
        }
        

        
    
        
    }
}
  

        
    


    
