use std::io;
use rand::Rng;

fn main() {
    println!("Guess the 6 digit number!");

    let secret_number = rand::thread_rng().gen_range(100000, 1000000);
    let secret_number = secret_number.to_string();

    let mut tries:u32 = 0; 

    loop {
        tries = tries + 1;

        println!("Please input your guess.");

        let mut guess = String::new();
        let mut guess_rem = String::new();
        let mut secret_rem = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        guess = guess.trim().to_string();

        if guess.to_lowercase() == "peek"{
            println!("Secret Number is {}",secret_number);
            tries = tries - 1;
            continue
        }
        if guess.to_lowercase() == "quit"{
            break
        }

        println!("you guessed: {}",guess);

        if guess.len() != 6 {
            println!("length:{} - guess must be a six digit number",guess.len());
            continue
        }

        let _: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("NaN - guess must be a six digit number");
                continue
            }
        };

        if guess == secret_number {
            println!("You win! Took {} tries",tries);
            break;
        }

        //lets generate some hints
        let guess_bytes = guess.as_bytes();
        let mut correct_place = 0;
        for (i, _) in guess_bytes.iter().enumerate() {
            if &guess[i..(i+1)] == &secret_number[i..(i+1)] {
                correct_place = correct_place + 1;
            } else {
                guess_rem.push_str(&guess[i..(i+1)]);
                secret_rem.push_str(&secret_number[i..(i+1)]);
            }
        }
        if correct_place > 0 {
            println!("{} digits are in the correct place.",correct_place);

        }
        let mut correct_value = 0;
        loop {
            if guess_rem.len() == 0 {
                break;
            }
            let my_char = guess_rem.pop().unwrap();
            let search = secret_rem.find(my_char);
            if search.is_some() {
                correct_value = correct_value + 1;
                secret_rem.remove(search.unwrap());
            } 
        }
       if correct_value > 0 {
            println!("{} digits are correct, but not in the correct place.",correct_value);

        }
    }
}
