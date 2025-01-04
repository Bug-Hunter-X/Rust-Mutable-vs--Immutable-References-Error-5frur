fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    
    *y = 10; // Modify x through y
    println!("x = {}", x); // Prints x = 10
    
    //Correct way to have both immutable and mutable references 
    let z = &x; // z is an immutable reference to x
    println!("x = {}", *z); // Prints x = 10
} 