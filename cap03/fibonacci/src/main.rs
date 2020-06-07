use std::io;

fn main() {
    loop {

        println!("Get the nth Fibonacci number");
        println!("Press q and enter to exit");

        println!("Enter the nth number:");

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Something was bad.");

        let user_input = user_input.trim();

        if user_input == "q" {
            println!("Bye!");
            break;
        }

        let user_input: i32 = match user_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        let mut fib_before = 0;
        let mut fib = 1;

        println!("");
        println!("{}!", fib_before);
        println!("{}!", fib);

        for _ in 1..user_input - 1 {
            let fib_n = fib_before + fib;

            println!("{}!", fib_n);

            fib_before = fib;
            fib = fib_n;
        }

        println!("");
    }
}
