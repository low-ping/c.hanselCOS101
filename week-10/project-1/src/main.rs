//Rust program to determine the total cost of 3 products
//from a particular brand
struct Laptop {
    price:u32, quantity:u32,
}
impl Laptop {
    fn cost(&self)->u32 {
        self.price * self.quantity
    }
}
fn main() {
    // instantiate the structure
    let hp  = Laptop {
        price:650_000,
        quantity:3,
    };
    println!("Price for hp laptop = {}.\nQuantity = {}\nCost is {}",
    hp.price,hp.quantity,hp.cost());
    // Second variable instantiated
    let ibm = Laptop {
        price:755_000,
        quantity:3,
    };
    println!("\nPrice for Ibm = {}.\nQuantity = {}\nCost is {}",
    ibm.price, ibm.quantity, ibm.cost());
    // Third variable instantiated
    let toshiba = Laptop {
        price:550_000,
        quantity:3,
    };
    println!("\nPrice for Toshiba = {}.\nQuantity = {}\nCost is {}",
    toshiba.price, toshiba.quantity, toshiba.cost());
    // Fourth variable to be instantiated
    let dell = Laptop {
        price:850_000,
        quantity:3,
    };
    println!("\nPrice for Dell = {}.\nQuantity = {}\nCost is {}",
    dell.price, dell.quantity, dell.cost());

    let total_price = hp.cost() + ibm.cost() + toshiba.cost() + dell.cost();
    println!("\nTotal purchases is {}", total_price);
}