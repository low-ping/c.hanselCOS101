use std::io::Write;

fn main() { 
    let name:[&str;6] = ["\n1.|\tAigbogun Alamba Daudu", "\n2.|\tMurtala Afeez Bendu", "\n3.|\tOkorocha Callistus Ogbona", "\n4.|\tAdewale Jimoh Akanbi", 
                        "\n5.|\tOsazuwa Faith Etieye", ""];
    let ministry:[&str;6] = ["\t\t\t\t|Internal Affairs", "\t\t\t\t\t|Justice", "\t\t\t|Defence", "\t\t\t\t|Power & Steel", "\t\t\t\t|Petroleum", "\t\t\t\t"]; 
    let zone:[&str;6] = ["\t\t|South West", "\t\t\t\t|North East", "\t\t\t\t|South South", "\t\t\t|South West", "\t\t\t\t|South East", "\t\t\t"];

    let mut file = std::fs::File::create("Efcc_record.txt").expect("create failed");
    file.write_all("\t\t\t\tConvicted Ministers".as_bytes()).expect("write failed");
    file.write_all("\n\t\t\t----------------------".as_bytes()).expect("write failed");
    file.write_all("\nno|\tNames".as_bytes()).expect("write failed");
    file.write_all("\t\t\t\t\t\t\t\t|Ministry".as_bytes()).expect("write failed");
    file.write_all("\t\t\t\t\t|Geopolitical zone".as_bytes()).expect("write failed");
    for item in 0..5{
        file.write_all(name[item].as_bytes()).expect("write failed");
        
        file.write_all(ministry[item].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        file.write_all(zone[item].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
    }

    
    
    
    
    println!("Data written to file.");


}
