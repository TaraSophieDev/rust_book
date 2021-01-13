fn main() {
    
}

fn shadowing() {
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of x is: {}", y);
}

fn mutable_variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 3;
    println!("The value of x is: {}", x);    
}