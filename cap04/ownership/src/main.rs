// Ownership Rules:

// Each value in Rust has a variable that's called its owner.
// There can only be one owner at time.
// When the owner goes out of scope, the vaue will be dropped.

// The Rules of Rererences:

// At any time, you can have either one mutable reference or any number of
// imutable references.
// References must always be valid.

// Alex notes:
// After create a imutable reference (read only), you can not create
// a mutable one (read an wirite) untill the imutable refence scope ends.


fn main() {
    // string literals
    // It is unmutable.
    // It is stored in the stack
    // It has know size.
    let phrase = "Alex";
    let phrase_1 = phrase;

    println!("phrase_1: {}", phrase_1);
    println!("phrase: {}", phrase);

    // String type, it is multable, you can push more text to it for example.
    // It is stored in the heap.println
    // It has unknown size.
    let mut phrase_2 = String::from("Alex");
    phrase_2.push_str(", Tom");
    println!("phrase_2: {}", phrase_2);

    // Scalar types
    // They are stored in the stack.
    // They are always deeply copied.
    let x = 5;
    let y = x;

    println!("y: {}", y);

    let mut s1 = String::from("hello");
    // let s2 = s1; // here the ownership is moved.

    // here stack and heap data are copied, it is expensive.
    // the ownership is not moved.
    let s2 = s1.clone();

    s1.push_str(", brazil!");

    println!("s1: {}", s1);

    println!("s2: {}", s2);

    let s3 = String::from("The cat");

    println!("s3: {}", s3);

    takes_ownership(s3);

    // println!("{}", s3); // s3 was moved, so it is invalid.

    let x = 5;

    makes_copy(x); // just stack copy

    println!("x: {}", x);


    let st1 = gives_ownership();

    let mut st2 = String::from("The cat Tempestade.");

    // let st3 =  takes_and_gives_back(st2);

    // because st2 was moved it needs receive the ownership again.
    st2 = takes_and_gives_back(st2);

    println!("st1: {}", st1);
    println!("st2: {}", st2); //st2 was moved.
    // println!("st3: {}", st3);

    // One possible approach to receive the ownership after some function call
    let (st2, st2_len) = calculate_length(st2);
    println!("st2: {} length: {}", st2, st2_len);

    // Using reference, the ownership transference doesn't happens
    let st4 = String::from("Brazil is amazing!");
    let st4_len = calculate_length_by_reference(&st4);
    println!("st4: {} length: {}", st4, st4_len);

    let mut st5 = String::from("The people in Brazil are very welcome!");
    // change function doesn't compiles, because it try change the reference parameter.
    // change(&st5);
    // change2 function receives a mutable reference so it can change it.
    change2(&mut st5);
    println!("st5: {}", st5);

    // Another example of borrow reference
    // Look that st6 still is valid after it be assigned to st7.
    let st6 = String::from("The Brazil's capital is Brasília.");
    let st7 = &st6;
    // it does not compile because refences are imutable by default.
    // st7.push_str(" some text");
    println!("st6: {}", st6);
    println!("st7: {}", st7);

    // here st8 is mutable.
    let mut st8 = String::from("The Brazil has so many beautiful places.");
    let st11 = &st8;
    println!("st11: {}", st11);
    {
        let st10 = &mut st8;
        st10.push_str(" For example: ")
    }
    // and st9 receives a mutable reference from st8.
    let st9 = &mut st8;
    // it does not compiles
    // you can have only one mutable reference to particular piece of data
    // in a particular scope.
    // let st10 = &mut st8;

    // it does not compiles utill the mutable reference ends.
    // let st12 = &st8;
    // println!("st12: {}", st12);

    st9.push_str("São Paulo"); // mutable reference ends
    println!("st8: {}", st8);
    // it does not compile,
    // because an imutable reference created before mutable ones
    // turns invalid after a mutable reference was creted.
    // println!("st11: {}", st11);

    // in this point we can create another read only reference from st8.
    let st12 = &st8;
    println!("st12: {}", st12);

    // let reference_to_nothing = dangle();
}

fn takes_ownership(some_string: String) {
    println!("some_string: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("The cat Mozart.");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_by_reference(s: &String) -> usize {
    s.len()
}

// it does not compiles because try change the reference.
// fn change(some_string: &String) {
//     some_string.push_str("changed text.");
// }

fn change2(s: &mut String) {
    s.push_str(" ");
    s.push_str("I love Brazil!")
}

// it does not compiles
// it trys returns a reference to a value that will be dropped at the functon end.
// fn dangle() -> &String {
//     let s = String::from("Brazil");

//     &s
// }
