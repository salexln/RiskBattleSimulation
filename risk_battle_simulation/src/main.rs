
extern crate rand;
use rand::Rng;
use std::env;
use rand::distributions::{IndependentSample, Range};
use std::process;



fn main() {
    let mut values = parse_args();
    attack(values.0, values.1);
}

fn attack(mut attackers : i32, mut defenders : i32 ) {
    if attackers > 3 {
        println!("You cannot attack with more than 3 soldiers");
        return;
    }

    if defenders > 2 {
        println!("You cannot defend with more than 2 soldiers");
        return;
    }

    if attackers < defenders {
        println!("Attackers number must be bigger than defenders");
        return;
    }

    let attack_rolls = roll_dices(attackers);
    let defend_rolls = roll_dices(defenders);

    let results = decide_on_winner(attack_rolls, defend_rolls);

    println!("", );
    println!("**** Summery ****");
    println!("Attacker won: {}", results.0);
    println!("Defender won: {}", results.1);

    if results.0 == 2 {
        println!("Defender lost 2 soldiers");
        return;
    }

    if results.1 == 2 {
        println!("Attacker lost 2 soldiers");
        return;
    }

    if results.0 == 1 && results.1 == 1 {
        println!("Both attacker and defender lost 1 soldier");
    }
}

fn decide_on_winner(mut attack_rolls : Vec<i32>, mut defend_rolls : Vec<i32>) -> (i32, i32){
    attack_rolls.sort();
    attack_rolls.reverse();
    defend_rolls.sort();
    defend_rolls.reverse();

    println!("Attack rolls : {:?}", attack_rolls);
    println!("Defend rolls : {:?}", defend_rolls);

    let mut idx = 0;
    let mut defender_wins = 0;
    let mut attack_wins = 0;
    for defender in defend_rolls {
        if defender >= attack_rolls[idx] {
            defender_wins += 1;
        } else {
            attack_wins +=1;
        }
        idx += 1;
    }

    return (attack_wins, defender_wins);
}

fn roll_dices(num : i32) -> Vec<i32> {
    let mut rolls : Vec<i32> = Vec::new();

    for x in 0..num {
        let roll = roll_dice();
        rolls.push(roll);
    }
    return rolls
}

// generates a random number between 1 and 6
fn roll_dice() -> i32 {
    let between = Range::new(1, 6);
    let mut rng = rand::thread_rng();
    let mut roll = between.ind_sample(&mut rng);

    return roll;
}

fn parse_args() -> (i32, i32){
    let args: Vec<String> = env::args().collect();
    let mut attackers = -1;
    let mut defenders = -1;
    if args[1] == "--help" {
        println!("--number-of-attackers - Number of soldies to attack");
        println!("--number-of-defenders - Number of soldies to defend");
        process::exit(1);
    } else {
        if args.len() == 5 {
            if args[1] == "--number-of-attackers" && args[3] == "--number-of-defenders" {
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
