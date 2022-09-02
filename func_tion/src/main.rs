fn main() {
    println!("Hello, world!");
    let result = add_numbers(3, 6);
    println!("the number is: {}", result)
}

fn add_numbers(x: i32, y:i32) -> i32 {
   return x + y;
}

//rust function can return an expression but a statement
//A expression is anything that gives you or return value
//A statement is variables declariton, its also a function that doesn't return anything 