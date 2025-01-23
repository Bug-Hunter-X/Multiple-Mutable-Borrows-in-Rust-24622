fn main() {
    let mut x = 5;
    { // Create a new scope for the first mutable borrow
        let y = &mut x;
        *y = 6;
    }
    { // Create a new scope for the second mutable borrow 
        let z = &mut x;
        *z = 7; 
    }
    println!("x = {}", x);
}