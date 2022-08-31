fn main() {
    //comditons - <, >, <=, >=, !=, ==
    let conditon1 = 3 == 3;
    println!("{}", conditon1);

    //compunds conditions - &&, ||, !
    let condition2 = !false || conditon1;
    println!("{}", condition2);

    //control flow - if, else
    let food = "biscuit";

    if food == "cookie" {
        println!("I like cookies!!!")
    }else {
        println!("oh, You dont like cookies...You like dicsuits😡😡😡")
    }

}
