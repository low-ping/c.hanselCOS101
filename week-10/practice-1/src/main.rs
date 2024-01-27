fn main(){
    let v = vec![101.0, 250.0, 330.0, 400.0];
    // vector v owns the object in heap
    // only single variable owns the heap memory at any time
    let _v2 = v.clone();
    // here two variables owns the heap value,
    //two pointers to the same content is not allowed in rust

    //Rust is very smart in terms of memory access, so it detects a race collection
    //as two variables point to same heap
    println!("{:?}", v);
}