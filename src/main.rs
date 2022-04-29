extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("let try ...");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .ok()
            .expect("Can read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Your try is: {}", input);

        match input.cmp(&secret_number) {
            Ordering::Less      => println!("is less"),
            Ordering::Equal     => { println!("right"); break },
            Ordering::Greater   => println!("is greater"),
        }

        println!("Your try is: {} vs {}", input, secret_number);
    }

    println!("DONE")
}