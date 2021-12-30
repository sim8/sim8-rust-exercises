fn main() {
    let mut x = 5;
    const ONE_MINUTE: i32 = 1000 * 60;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The length of a minute is: {}", ONE_MINUTE);
    let _x: bool = true;
    // shadowing allows changing data type
}
