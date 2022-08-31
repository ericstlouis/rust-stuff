//crate = packages
//io - helpful methods used for when inputing and outputing
use std::io;

fn main() {
    println!("What is your name?");
    //initialize a mutable string varibales that will be used to saved the user input
    let mut input = String::new();

    //stdin() - create an instance of stdin()
    let stdin = io::stdin();
    
    //using the method read_line to read the user input ans store in input
    stdin.read_line(&mut input).unwrap();

    //print output user input
    println!("hello {}", input);

}

//ref: https://www.becomebetterprogrammer.com/rust-read-user-input-stdin/

