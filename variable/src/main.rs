fn main() {
    let mut x = 5; // mut is used to make the variable mutable
    // x is immutable by default, so we cannot change its value
    println!("The value of x is: {x}");
    x = 6; // this line will cause an error because x is immutable
    // to make x mutable, we need to use mut keyword
    println!("The value of x is: {x}");
}
