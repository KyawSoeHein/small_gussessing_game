use rand::{random, Rng};
use std::io;

fn main() {
    let random_number: isize = generate_new_random_number();
    let mut already_won: bool = false;

    while !already_won {
        already_won = play_game(random_number);
    }
}

fn generate_new_random_number() -> isize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..100);
}

fn play_game(random_number: isize) -> bool {
    let mut buffer = String::new();
    println!("Enter your guess");
    io::stdin().read_line(&mut buffer).unwrap();
    let user_input_num = buffer.trim().parse::<isize>().unwrap();
    println!("Randon number is {}", random_number);
    if random_number < user_input_num {
        println!("Too Big");
        return false;
    } else if random_number > user_input_num {
        println!("Too Small");
        return false;
    } else {
        println!("You are correct");
    }

    return true;
}
