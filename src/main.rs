#[allow(dead_code)]
fn main() {
    
}


#[allow(dead_code)]
fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0;
    let y: f32 = 3.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'c';
    let z = 'z';
    let heart_eyed_cat = '😻';

    //Tuple Type
    let tup = (500, 6.4, 1);
    let (x,y,z)  = tup;
    println!("The value of y is: {}", y);

    //Acces variable with index
    let x = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2


}

#[allow(dead_code)]
fn shadowing() {
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of x is: {}", y);

    let spaces = "       ";
    let spaces = spaces.len();
}

#[allow(dead_code)]
fn mutable_variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 3;
    println!("The value of x is: {}", x);    
}

