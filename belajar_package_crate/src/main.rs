use rand::prelude::*;

fn main() {
    println!("Hello, world!");

    for i in 0..50{
        println!("Random number {} : {}", i, generate_random_number())
    }
}

fn generate_random_number() -> u32{
    let mut rng = rand::rng();

    return rng.random_range(0..1000)
}
