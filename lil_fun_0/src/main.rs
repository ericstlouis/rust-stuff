use std::io;

fn main() {
    println!("what is your favorite food?");
    println!("pizza, salad or avocados");

    let mut input = String::new();

    let stdin = io::stdin();
    stdin.read_line(&mut input).expect("read the line!");
    // println!("{}", input)
    input.to_string();

    match input {
        "pizza".to_string() => println!("nice you got taste.🍕🤤"),
    }

    // if input == "pizza" {
    //     println!("nice you got taste.🍕🤤");
    // }  else if input == "salad" {
    //     println!("healthy but tasty I see you🥗😋");
    // }  else if input == "avacadoes" {
    //     println!("ewww, die you dirty bitch🥑🤢🤮");
    // } else {
    //     println! ("follow instructions idiot😡👊🏾");
    // }



}
