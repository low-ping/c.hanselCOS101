struct Employee {
    name:String,
    company:String,
    age:u32,
}
fn main() {
    let emp1 = Employee {
        company:("Ernst & Young").to_string(),
        name:("Ebibiong Jessica").to_string(),
        age:25
    };
    println!("Name = {} \n",emp1.name);
    println!("company = {} \n", emp1.company);
    println!("Age = {} ", emp1.age);
}