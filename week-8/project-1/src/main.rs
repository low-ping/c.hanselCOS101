// A rust program to validate staff level
use std::io;

fn main() {
    let mut name : Vec<String> = Vec::new();
    let mut occupation : Vec<String> = Vec::new();
    let mut years_of_experience : Vec<usize> = Vec::new();
    let mut rank : Vec<String> = Vec::new();
    let mut position : Vec<String> = Vec::new();

    println!("How many staff do you want to check level: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("not a valid name");
    let n = input1.trim().parse().expect("not a valid name");
    for _i in 0..n{
        println!("
        click 1. For Office administrator
        click 2. For Academic
        click 3. For Lawyer
        click 4. For Teacher");

        //Using choice
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("not a valid choice");
        let choice:u32 = input2.trim().parse().expect("not a valid choice");

        if choice == 1{
            println!("Enter Staff name: ");
            let mut input3 = String::new();
            io::stdin().read_line(&mut input3).expect("not a valid string");
            let staff_name = input3.trim().to_string();
            name.push(staff_name);
            
            println!("Enter staff occupation: ");
            let mut input4 = String::new();
            io::stdin().read_line(&mut input4).expect("not a valid string");
            let staff_occupation = input4.trim().to_string();
            occupation.push(staff_occupation);
             
            println!("Enter staff rank: ");
            let mut input5 = String::new();
            io::stdin().read_line(&mut input5).expect("not a valid string");
            let staff_rank = input5.trim().to_string();
            rank.push(staff_rank.clone());
            

            if staff_rank == "Intern" {
                let rank_1 = "APS 1-2".to_string();
                position.push(rank_1);
            } else if staff_rank == "Administrator" {
                let rank_2 = "APS 3-5".to_string();
                position.push(rank_2);
            } else if staff_rank == "Senior Administrator" {
                let rank_3 = "APS 5-8".to_string();
                position.push(rank_3);
            } else if staff_rank == "Office Manager" {
                let rank_4 = "EL1 8-10".to_string();
                position.push(rank_4);
            } else if staff_rank == "Director" {
                let rank_5 = "EL2 10-13".to_string();
                position.push(rank_5);
            } else if staff_rank == "CEO" {
                let rank_6 = "SES".to_string();
                position.push(rank_6);
            } else {
                println!("Quit");
            }
            
            println!("Enter staff of years experience: ");
            let mut input6 = String::new();
            io::stdin().read_line(&mut input6).expect("not a valid string");
            let staff_years_of_experience:usize = input6.trim().parse().expect("not a valid number");
            years_of_experience.push(staff_years_of_experience);



            
        } 
        else if choice == 2{
                println!("Enter Staff name: ");
                let mut input8 = String::new();
                io::stdin().read_line(&mut input8).expect("not a valid string");
                let staff_name = input8.trim().to_string();
                name.push(staff_name);
                
                println!("Enter staff occupation: ");
                let mut input9 = String::new();
                io::stdin().read_line(&mut input9).expect("not a valid string");
                let staff_occupation = input9.trim().to_string();
                occupation.push(staff_occupation);
                 
                println!("Enter staff rank: ");
                let mut input10 = String::new();
                io::stdin().read_line(&mut input10).expect("not a valid string");
                let staff_rank = input10.trim().to_string();
                rank.push(staff_rank.clone());
            
                
                if staff_rank == "No position" {
                    let rank_1 = "APS 1-2".to_string();
                    position.push(rank_1);
                } else if staff_rank == "Research assistant" {
                    let rank_2 = "APS 3-5".to_string();
                    position.push(rank_2);
                } else if staff_rank == "PhD Candidate" {
                    let rank_3 = "APS 5-8".to_string();
                    position.push(rank_3);
                } else if staff_rank == "Post-Doc Researcher" {
                    let rank_4 = "EL1 8-10".to_string();
                    position.push(rank_4);
                } else if staff_rank == "Senior Lecturer" {
                    let rank_5 = "EL2 10-13".to_string();
                    position.push(rank_5);
                } else if staff_rank == "Dean" {
                    let rank_6 = "SES".to_string();
                    position.push(rank_6);
                } else {
                    println!("Quit");
                }
                println!("Enter staff of years experience: ");
                let mut input11 = String::new();
                io::stdin().read_line(&mut input11).expect("not a valid string");
                let staff_years_of_experience:usize = input11.trim().parse().expect("not a valid number");
                years_of_experience.push(staff_years_of_experience);
    
    
                
            }
             else if choice == 3 {
                    println!("Enter Staff name: ");
                    let mut input13 = String::new();
                    io::stdin().read_line(&mut input13).expect("not a valid string");
                    let staff_name = input13.trim().to_string();
                    name.push(staff_name);
                    
                    println!("Enter staff occupation: ");
                    let mut input14 = String::new();
                    io::stdin().read_line(&mut input14).expect("not a valid string");
                    let staff_occupation = input14.trim().to_string();
                    occupation.push(staff_occupation);
                     
                    println!("Enter staff rank: ");
                    let mut input15 = String::new();
                    io::stdin().read_line(&mut input15).expect("not a valid string");
                    let staff_rank = input15.trim().to_string();
                    rank.push(staff_rank.clone());

                    if staff_rank == "Paralegal" {
                        let rank_1 = "APS 1-2".to_string();
                        position.push(rank_1);
                    } else if staff_rank == "Junior Associate"
                     {
                        let rank_2 = "APS 3-5".to_string();
                        position.push(rank_2);
                    } else if staff_rank == "Associate" {
                        let rank_3 = "APS 5-8".to_string();
                        position.push(rank_3);
                    } else if staff_rank == "Senior Associate 1-2" {
                        let rank_4 = "EL1 8-10".to_string();
                        position.push(rank_4);
                    } else if staff_rank == "Senior Associate 3-4" {
                        let rank_5 = "EL2 10-13".to_string();
                        position.push(rank_5);
                    } else if staff_rank == "Partner" {
                        let rank_6 = "SES".to_string();
                        position.push(rank_6);
                    } else {
                        println!("Quit");
                    }
                    
                    println!("Enter staff of years experience: ");
                    let mut input16 = String::new();
                    io::stdin().read_line(&mut input16).expect("not a valid string");
                    let staff_years_of_experience:usize = input16.trim().parse().expect("not a valid number");
                    years_of_experience.push(staff_years_of_experience);
    
        
                    
                }
                 else if choice == 4{
                        println!("Enter Staff name: ");
                        let mut input18 = String::new();
                        io::stdin().read_line(&mut input18).expect("not a valid string");
                        let staff_name = input18.trim().to_string();
                        name.push(staff_name);
                        
                        println!("Enter staff occupation: ");
                        let mut input19 = String::new();
                        io::stdin().read_line(&mut input19).expect("not a valid string");
                        let staff_occupation = input19.trim().to_string();
                        occupation.push(staff_occupation);
                         
                        println!("Enter staff rank: ");
                        let mut input20 = String::new();
                        io::stdin().read_line(&mut input20).expect("not a valid string");
                        let staff_rank = input20.trim().to_string();
                        rank.push(staff_rank.clone());

                        if staff_rank == "Placement" {
                            let rank_1 = "APS 1-2".to_string();
                            position.push(rank_1);
                        } else if staff_rank == "Classroom Teacher" {
                            let rank_2 = "APS 3-5".to_string();
                            position.push(rank_2);
                        } else if staff_rank == "Snr Teacher" {
                            let rank_3 = "APS 5-8".to_string();
                            position.push(rank_3);
                        } else if staff_rank == "Leading Teacher" {
                            let rank_4 = "EL1 8-10".to_string();
                            position.push(rank_4);
                        } else if staff_rank == "Deputy Principal" {
                            let rank_5 = "EL2 10-13".to_string();
                            position.push(rank_5);
                        } else if staff_rank == "Principal" {
                            let rank_6 = "SES".to_string();
                            position.push(rank_6);
                        } else {
                            println!("Quit");
                        }
                        
                        println!("Enter staff of years experience: ");
                        let mut input21 = String::new();
                        io::stdin().read_line(&mut input21).expect("not a valid string");
                        let staff_years_of_experience:usize = input21.trim().parse().expect("not a valid number");
                        years_of_experience.push(staff_years_of_experience);
            
                        
                    } else {
                        println!("Staff Occupation is not found");

                    }
                
                }
            
        
        
        for index in 0..n{
            
            println!("Staff {} Details is as follows", index+1);
            println!("STAFF NAME: {}", name[index]);
            println!("STAFF OCCUPATION: {}", occupation[index]);
            println!("YEARS OF EXPERIENCE: {}", years_of_experience[index]);
            println!("RANK: {}", rank[index]);
            println!("POSITION: {}", position[index]);
        
        }


    }
