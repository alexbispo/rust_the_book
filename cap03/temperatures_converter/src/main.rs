use std::io;

fn main() {
    println!("Temperature converter!");
    println!("===============================================================");

    loop {
        println!("Quit: q <enter>");
        println!("Fahrenheit to Celsius: f <temperature_number> <enter>");
        println!("Celsius to Fahrenheit: c <temperature_number> <enter>");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Something was bad!");

        println!("");

        let user_input = user_input.trim();

        if user_input == "q" {
            println!("Bye!");
            break;
        }

        let inputs: Vec<&str> = user_input.trim().split(' ').collect();

        if inputs.len() != 2 {
            println!("Sorry! Invalid option.\n");
            continue;
        }

        let origin_temperature = inputs[0];

        let temperature: f32 = match inputs[1].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Sorry! Invalid option.\n");
                continue;
            }
        };

        if origin_temperature == "f" {
            let converted_temperature = (temperature - 32.0) * (5.0/9.0);

            println!("{} fahrenheit to celsius is {:.2}", temperature, converted_temperature);
        } else if origin_temperature == "c" {
            let converted_temperature = (temperature * (9.0/5.0)) + 32.0;

            println!("{} celsius to fahrenheit is {:.2}", temperature, converted_temperature);
        } else {
            println!("Sorry! Invalid option.\n");
            continue;
        }

        println!("");
    }
}
