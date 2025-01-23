fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; // This will cause a compile-time error
    *y = 6;
    *z = 7; 
}