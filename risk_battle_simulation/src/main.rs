
extern crate rand;
use rand::Rng;
use std::env;



fn main() {
    //print_random_number();
    let mut values = parse_args();
    println!("{:?}", values);
}


fn parse_args() -> (i32, i32){
    let args: Vec<String> = env::args().collect();
    let mut attackers = -1;
    let mut defenders = -1;
    if args[1] == "--help" {
        println!("--number-of-attackers - Number of soldies to attack");
        println!("--number-of-defenders - Number of soldies to defend");
    } else {
        if args.len() == 5 {
            if args[1] == "--number-of-attackers" && args[3] == "--number-of-defenders" {
                println!("Valid input");
                attackers = args[2].parse().unwrap();
                defenders = args[4].parse().unwrap();
            } else {
                println!("Invalid input");
            }
        } else {
            println!("Invalid input");
        }

    }
    return (attackers, defenders);
}
