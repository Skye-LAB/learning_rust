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

Pertama, untuk mendapatkan inputan dari user lalu menampilkannya sebagai output, kita perlu menambahkan library `io`, library `io` berada dalam library standard, yang dipanggil `std`:

```rust
use std::io;
```

Seperti yang sudah Anda pelajari di [Chapter 1](../01_getting_started), bahwa fungsi `main` adalah *entry point* dalam sebuah program.

```rust
fn main() {}
```

Juga, sudah Anda pelajari di [Chapter 1](../01_getting_started), bahwa `println!` adalah sebuah *macro* yang akan menampilkan string ke layar.

```rust
    println!("Guess the number!");

    println!("Please input your guess.");
```
