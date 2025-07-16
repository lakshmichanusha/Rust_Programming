fn main() {
    let a = ("a", "b","c");

    if let ("a", "b","c") = a {
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    } else {
     
        println!("Value unmatched");
    }
}
