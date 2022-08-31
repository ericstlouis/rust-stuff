use std::io;
fn main() {

    //unsigned integer(u8, u16, u32...) = 0 - 255
    //signded integer(i8, i16, i32...) = -128 -127

    //Bacic addition
    let x: i8 = 9;  
    let y: i8 = 10; 

    let z = x + y;
    println!("{}", z);

    //Type conversion 
    let a = 127 as i64;
    let b = 10 as i32;

    // *best pracitce* is too convert the lower byte to a higher byte to prevent
    // overflow and underflow
    let c = a * (b as i64);
    println!("{}", c);

    //Converting string to an integer
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).expect("read the line");

    /*
    convert string into integer by:
    **trimming** an invisible terminal character???
    **parse** the string into a number...if it could
    **unwrap** is marco similiar to expect, 
    it basically allows the function to pass to the next step 
    if has a ok value. if it has no value or passes a err it panics and throw an error
    */
    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 3);

}

//overflow is basically a bytes reaches it limit?...I think
