fn main() {
    let x = five();
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    return 5;
}
