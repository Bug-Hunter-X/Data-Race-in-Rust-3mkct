fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y = 10; 
    println!("x = {}", x);
    //To avoid data race don't create multiple mutable reference
}