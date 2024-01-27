use std::io::Write;

fn main() {
    let name:[&str;5] = ["\n|Oluchi Mordi", "\n|Adams Aliyu", "\n|Shania Bolade", "\n|Adekunle Gold", "\n||Blanca Edemoh"];
    let matric:[&str;5] = ["\t\t|ACC10211111", "\t\t|ECO10110101", "\t\t|CSC10338838", "\t\t|EEE11020202", "\t\t|MEE10202001"];
    let dept:[&str;5] = ["\t\t\t|Accounting", "\t\t\t|Economics", "\t\t\t|Computer", "\t\t\t|Electrical", "\t\t\t|Mechanical"];
    let level:[&str;5] = ["\t\t\t\t|300", "\t\t\t\t|100", "\t\t\t\t|200", "\t\t\t\t|200", "\t\t\t\t|100"];
    let mut file = std::fs::File::create("pau-sims.tex.txt").expect("create failed");
    file.write_all("\t\t\t\tPAU SIMS".as_bytes()).expect("write failed");
    file.write_all("\t\n\nStudent Name".as_bytes()).expect("write failed");
    file.write_all("\t\tMatric Number".as_bytes()).expect("write failed");
    file.write_all("\t\t\tDepartment".as_bytes()).expect("write failed");
    file.write_all("\t\t\t\tLevel".as_bytes()).expect("write failed");
    for index in 0..4{
        file.write_all(name[index].as_bytes()).expect("write failed");
        file.write_all(matric[index].as_bytes()).expect("write failed");
        file.write_all(dept[index].as_bytes()).expect("write failed");
        file.write_all(level[index].as_bytes()).expect("write failed");

        

    }
    println!("Data written to file");
}