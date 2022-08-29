fn main() {
    //Primitives - The basic data types used to declare a variable

        //Scaler Types - Represents a Single Value

            //Integer type - A number 
                let i: i32  = 24;
                println!("{:?}", i);

            //Floating-point type - A number with a deciamal
                let f: f32 = 24.5;
                println!("{}", f);

            //Boolean type - True of false
                let b: bool = true;
                println!("{}", b);

            //Character type - any character between single quotation ranging from emojis, kanjis ..etc
                let c: char = '🥷';
                println!("{}", c);

        //Compound types - Represents a group of values

            //Tuples - a group of different types
                let t: (i32, bool, char) = (1, true, 's');
                println!("{}, {}, {}", t.0, t.1, t.2);

            //Array - a group/collection of elements of the same type
                let a = [1,2,3,4,5];
                println!("{}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);

}
// ! = marco/metaprogramming - runs at compile time