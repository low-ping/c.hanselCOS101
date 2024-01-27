use std::io::Write;

fn main() { 
    let lager_drinks:[&str;6] = ["\n|\tExport", "\n|\tDesperados", "\n|\tGoldberg", "\n|\tGulder", "\n|\tHeineken", "\n|\tStar"];
    let stout:[&str;6] = ["\t\t\t|\tLegend", "\t\t|\tTurbo King", "\t\t|\tWilliams", "\t\t", "\t\t", "\t\t\t\t"]; 
    let non_alcoholic:[&str;6] = ["\t\t\t\t\t|\tMaltina", "\t\t\t\t|\tAmstel Malta", "\t\t\t\t|\tMalta Gold", "\t\t\t\t\t\t\t\t|\tFayrouz", "\t\t\t\t", "\t\t\t"];

    let mut file = std::fs::File::create("Nigeria_Brewies_plc.txt").expect("create failed");
    file.write_all("\t\t\t\tNigeria Brewies plc".as_bytes()).expect("write failed");
    file.write_all("\n\t\t\t\t------------------".as_bytes()).expect("write failed");
    file.write_all("\n|\tLager Drinks".as_bytes()).expect("write failed");
    file.write_all("\t\t|\tStout Drinks".as_bytes()).expect("write failed");
    file.write_all("\t\t\t\t|\tNon-Alcoholic Drinks".as_bytes()).expect("write failed");
    for item in 0..5{
        file.write_all(lager_drinks[item].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        file.write_all(stout[item].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        file.write_all(non_alcoholic[item].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
    }

    
    
    
    
    println!("Data written to file.");


}