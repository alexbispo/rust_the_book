use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Acerte o número!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop  {
        println!("Por favor digite o seu chute.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Chute inválido");
                continue;
            }
        };
    
        println!("Você chutou: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
