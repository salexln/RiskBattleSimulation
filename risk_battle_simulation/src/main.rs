
extern crate rand;
use rand::Rng;



fn main() {
    print_random_number();
}


fn print_random_number() {
    let mut rng = rand::thread_rng();

    if rng.gen() {
        println!("random number is {}", rng.gen::<i32>());
    }
}
