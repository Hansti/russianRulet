use std::io;
use rand::Rng;

fn main() {
    let bullets: u32 = input().trim().parse().expect("Please type a number!");
    let secret_number = rand::thread_rng().gen_range(1, 6);

    if secret_number > bullets {
        println!("You won!!!");
    } else {
        println!("You die!!!");
        println!("--> BOOM :(");
    }
} 

fn input() -> String {
    println!("Please input bullet count in revolver.");

    let mut bullet_count = String::new();
    io::stdin().read_line(&mut bullet_count)
        .expect("Failed to read line");

    bullet_count
}


