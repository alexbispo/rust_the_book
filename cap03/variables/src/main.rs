const SIX: u16 = 6;

fn main() {

    // Variables and constants

    let mut x = 5;
    println!("The value of 'x' is: {} ", x);

    x = SIX;
    println!("Now the value of 'x' is {}", x);


    // integer types
    let y = 3; // i32 by default
    let y = y + 5;
    let y = y * 2;

    println!("The value of 'y' is {}", y);

    let guess: u32 = "23".parse().expect("Not a number");

    println!("The value of guess is {}", guess);


    // float types

    let a = 2.55; // f64 by default

    println!("The value of 'a' is {}", a);

    let b: f32 = 3.44; // f32

    println!("The value of 'b' is {}", b);


    // mathematical operations

    // addition
    let sum = a + b;
    println!("The value of 'sum' is {}", sum);

    // subtraction
    let difference = a - b;
    println!("The value of 'difference' is {}", difference);

    // multiplication
    let product = a * b;
    println!("The value of 'product' is {}", product);

    // division
    let quotient = a / b;
    println!("The value of 'quotient' is {}", quotient);

    // remainder
    let remainder = a % b;
    println!("The value of 'remainder' is {}", remainder);


    // boolean type

    let t = true;

    let f: bool = false;

    println!("The value of 't' is {}", t);
    println!("The value of 'f' is {}", f);


    // character

    let _c = 'z';
    let _z = 'Z';
    let cat: char = '😺';
    println!("The value of 'cat' is {}", cat);


    // tuple

    let tup: (&str, f64, u32) = ("Mozart", 2.5, 0);
    println!("The value of 'tup' is {:?}", tup);

    let (name, age, genre) =  tup;
    println!("{} is {} and {}", name, age, genre);
    println!("{} is {} and {}", tup.0, tup.1, tup.2);


    // array

    let mut names: [&str; 2] = ["Alex", "Bispo"];
    names[0] = "Mozart";
    println!("The value of 'name's is: {:?}", names);
    println!("The value of 'names[1]' is: {:?}", names[1]);
    let _index = 2;
    // println!("It will cause a Panic! {}", names[index]);

    let eco = ["Tom"; 3];
    println!("The value of eco is {:?}", eco);
}
