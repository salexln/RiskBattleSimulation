
extern crate rand;
use rand::Rng;
use std::env;



fn main() {
    //print_random_number();
    parse_args();
}


fn print_random_number() {
    let mut rng = rand::thread_rng();

    if rng.gen() {
        println!("random number is {}", rng.gen::<i32>());
    }
}

fn parse_args() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "--help" {
        println!("--number-of-attackers - Number of soldies to attack");
        println!("--number-of-defenders - Number of soldies to defend");
    } else {
        if args.len() == 5 {
            if args[1] == "--number-of-attackers" && args[3] == "--number-of-defenders" {
                println!("valid input");
            } else {
                println!("Invalid input");
            }
        } else {
            println!("Invalid input");
        }

    }
}
