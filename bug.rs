fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y = 10; // Modify x through y
    println!("x = {}", x); // Prints x = 10

    // This line will cause a compile-time error because we're trying to
    // modify x through z which is an immutable reference
    //*z = 15;
}