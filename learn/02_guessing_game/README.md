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

## Menyimpan Nilai Dengan Variabel

Selanjutnya, kita membuat sebuah variabel untuk menyimpan inputan dari user, seperti ini:

```rust
    let mut guess = String::new();
```

Untuk membuat sebuah variabel di Rust di memakai keyword `let`. Secara default, variabel di Rust tidak bisa diupdate atau *immutable*,untuk membuat sebuah variabel menjadi mutable, kita menambahkan keyword `mut` setelah `let`.

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

Kembali ke program game menebak angka, variabel `guess` akan diisi dengan hasil dari pemanggilan fungsi `String::new()`, sebuah fungsi yang akan mengembalikan instance baru suatu `String`. tanda `::`, menandakan fungsi `new` adalah sebuah associated function dari `String`.