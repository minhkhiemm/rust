extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    //  print guessing a number
    println!("Guessing game@@");
    // random secret number
    let secret = rand::thread_rng().gen_range(1, 100);
    // create loop
    loop {
        println!("guess a number: ");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("failed to read from standard input");
        let num: u32 = num.trim().parse().expect("this is not a number");
        match num.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
