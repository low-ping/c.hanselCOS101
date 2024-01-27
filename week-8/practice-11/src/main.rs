fn main() {
    let numbers = [1, 2, 3, 4, 5];
    println!("Original array = {:?}", numbers);

    // create a slice of 2nd and 3rd element
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}", slice1);

    // Omit the start index
    let slice2 = &numbers[..3];
    // This means the slice starts from index 0 and goes up to zero inclusive)
    println!("index 0 to index 3 sliced = {:?}", slice2);

    // Omit the end index
    let slice3 = &numbers[2..];
    // This means the slice starts from index 2 and goes up to index 5 exclusive
    println!("index 2 to index 5 sliced = {:?}", slice3);

    // Omit the start index and the end index
    // reference the whole array
    let slice4 = &numbers[..];
    // This means the starts from index 0 and goes up to index 5 (exclusive).
    println!("index 0 to index 5 sliced = {:?}", slice4);
}
