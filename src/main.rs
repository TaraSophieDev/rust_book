#[allow(dead_code)]
fn main() {
    reference_borrowing();
}

fn reference_borrowing() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        // r1 goes out of scope here, so we can make a new reference with no problems.
    }
    let r2 = &mut s;
    // r2 will work bc r1 went out of scope

    let r2 = &s; //Works
    //{let r3 = &mut s;} Won't work.
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn ownership() {
    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print 'hello, world!'


    let s1 = String::from("hello");
    //let s2 = s1; // Doesn't work because it tries to copy an empty value because of dropping s1
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

}

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

    //Access variable with index
    let x = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}

fn shadowing() {
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of x is: {}", y);

    let spaces = "       ";
    let spaces = spaces.len();
}

fn mutable_variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 3;
    println!("The value of x is: {}", x);    
}

