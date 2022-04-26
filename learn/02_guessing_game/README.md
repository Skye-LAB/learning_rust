# Game Menebak Angka

## Mengolah Tebakan

Bagian pertama dalam game menebak angka adalah program untuk menanyai inputan dari user dan mengecek apakah sama seperti yang diharapkan. Ok, kita mengambil inputan dari user dengan program berikut:

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

```
