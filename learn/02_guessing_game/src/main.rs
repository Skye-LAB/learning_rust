use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Gues the number!");

    // membuat sebuah nomor random
    let secret_number = rand::thread_rng().gen_range(1..101);

    // akan terus looping sampai ter-break
    loop {
        let mut guess = String::new();

        // menangkap inputan user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // mengecek apakah inputan user adalah nomor atau bukan
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // menentukan tebakan user benar atau tidak
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
