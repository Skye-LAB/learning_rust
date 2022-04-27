# Game Menebak Angka

## Membuat Nomor Acak

### Menambahkan Dependensi

Untuk membuat sebuah nilai acak, kita perlu sebuah crate yairu `rand`. Kita bisa menginstalnya dengan menambahkan baris dibawah ini ke *Cargo.toml* di bagian `[dependencies]`.

```toml
rand = "0.8.3"
```

Kemudian jalankan perintah `cargo build` untuk men-download dependensi yang baru saja ditambahkan.

### Membuat Nomor Acak

Untuk membuat nomor secara acak, pertama kita perlu memanggil library `rand`.

```rust
use rand::Rng;
```

`Rng` merupakan sebuah *trait*. Trait akan kita pelajari di [Chapter 10](../generic_types_traits_and_lifetimes)

Kemudian untuk membuat nomor acak, kita menggunakan kode:

```rust
let secret_number = rand::thread_rng().gen_range(1..101);
```

Untuk menulis variabel di Rust, kita menggunakan keyword `let` diikuti dengan nama variabel. Secara default, variabel di Rust tidak bisa diupdate atau *immutable*,untuk membuat sebuah variabel menjadi mutable, kita menambahkan keyword `mut` setelah `let`.

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```
Untuk variabel, kita akan pelajari di [Chapter 3](../03_common_programming_concepts).

Pertama, kita memanggil fungsi `rand::thread_rng()`, tanda `::`, menandakan fungsi `thread_rng` adalah sebuah associated function dari `rand`. Kemudian kita memanggil fungsi `gen_range`, fungsi ini akan menerima ekspresi rentang nomor sebagai argumen dan menghasilkan nomor acak dalam rentang tersebut. Disini kita memakai rentang 1 sampai 100, `1..101` atau `1.=100`.

## Mengizinkan Beberapa Tebakan Dengan Looping

Untuk membuat sebuah looping, agar user dapat menginputkan banyak tebakan, kita menggunakan perintah `loop`.

```rust
loop {
    // kodemu berada disini
}
```

Untuk menghentikan looping ini, kita dapat menggunakan keyword `break`.

## Menangkap Inputan Dari User

Untuk menangkap inputan dari user, kita menggunakan kode berikut:

```rust
let mut guess = String::new();

io::stdin()
    .read_line(&mut guess)
```

Sebelumnya, kita perlu menambahkan library `io` diawal program.

```rust
use std::io;
```

Fungsi `io::stdin` akan me-return sebuah instance dari `std:io:Stdin`, yang merupakan sebuah tipe yang menangani input standar untuk terminal Anda.

Selanjutnya, baris `.read_line(&mut guess)` memanggil fungsi `read_line` yang akan menangkap inputan dari user. Kita juga menambahkan `&mut guess` sebagai argument di fungsi `read_line`. Tanda `&` menunjukkan bahwa argument ini adalah sebuah *reference*, yang memberikan jalan ke banyak bagian di kode mu untuk mengakses satu data tanpa perlu mengkopi data tersebut ke memori berkali-kali. Reference adalah sebuah fitur yang kompleks, juga salah satu keunggulan Rust. Kita akan mempelajari *reference* di [Chapter 4](../04_understanding_ownership).

Selanjutnya adalah perintah ini:

```rust
        .expect("Failed to read line");
```

Fungsi `read_line` sebelumnya akan meletakkan apapun yang user inputkan ke dalam string yang kita berikan, yaitu `&mut guess`, fungsi ini juga akan me-return sebuah nilai, di kasus ini adalah `io::Result`. Tipe `Result` adalah sebuah *enums*, yang dapat memiliki serangkaian kemungkinan tetap, yang dikenal sebagai *varian*, kita akan mempelajari *enums* di [Chapter 6](../06_enums_and_pattern_matching). Enums biasa digunakan bersama `match`, sebuah kondisional untuk mengeksekusi kode berbeda berdasarkan varian enums tersebut.

Enums `Result` memiliki dua varian, yaitu `Ok` dan `Err`. Varian `Ok` menandakan operasi berhasil dan didalamnya adalah nilai yang berhasil dihasilkan. Varian `Err` menandakan operasi gagal dan didalamnya terdapat informasi tentang bagaimana dan mengapa operasi tersebut gagal.

Nilai dalam tipe `Result` sama seperti nilai pada tipe lainnya, mempunyai *methods* didalamnya. sebuah instance dari `io::Result` mempunyai `expect` method. Jika sebuah instance dari `io::Result` adalah sebuah nilai `Err`, maka `expect` akan memnyebabkan program crash dan akan menampilkan pesan yang Anda masukkan sebagai argument didalamnya. Jika nilainya adalah berupa `Ok`, maka nilai yang ada dalam `Ok` akan di-return.

## Menangani Inputan Yang Invalid



## Mencetak Nilai Dengan println! Placeholder

Selanjutnya, adalah perintah dibawah ini:

```rust
    println!("You guessed: {}", guess);
```

Baris ini akan mencetak string yang mengandung inputan dari user. tanda `{}` merupakan sebuah placeholder, yang dengannya kita dapat mencetak nilai dari sebuah variabel.
