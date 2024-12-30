fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x
    *y = 10; 
    *z = 20; //This will cause a data race
    println!("x = {}", x);
}