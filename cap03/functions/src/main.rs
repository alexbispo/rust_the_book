fn main() {
    println!("I'm the main function.");

    let x = 5;
    another_functiion(x, "Mozart");

    let sum_result = sum(8, 6);
    println!("The sum result is: {}", sum_result);

    let new_scope_result = {
        let a = 31;
        let b = 30;
        a + b
    };
    println!("The value of new_scope_result is {}", new_scope_result);

    // let x = (let y = 6); not compile
}

fn another_functiion(x: u32, y: &str) {
    println!("The value of x is {}",  x);
    println!("The value of y is {}",  y);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
