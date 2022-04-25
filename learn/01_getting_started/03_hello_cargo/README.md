# Hello, Cargo!

## Penjelasan Cargo

Cargo adalah Rust build system dan package manager. Banyak pengguna Rust menggunakan tool ini untuk mengelola project Rust mereka, karena Cargo dapat menangani banyak tugas untuk Anda, seperti mem-build kode mu, meng-download library dan mem-build library-library.

Saat Anda menulis program Rust yang lebih kompleks, Anda suatu saat pasti perlu menambahkan dependensi. Jika Anda memulai proyek Anda dengan Cargo, menambahkan dependensi akan lebih mudah.

Untuk mengecek apakah Cargo sudah terinstall atau belum, gunakan perintah berikut:

```bash
$ cargo --version
```

Jika Cargo sudah terinstall di komputer Anda, maka Anda akan melihat baris seperti berikut:

```bash
cargo 1.60.0 (d1fd9fe 2022-03-01)
```

## Membuat Proyek Dengan Cargo

Anda dapat membuat proyek baru dengan Cargo dengan perintah berikut:

```bash
$ cargo new hello_cargo
```

Setelah menjalankan perintah diatas, maka akan terbuat struktur folder seperti berikut:

```
hello_cargo/
├── Cargo.toml
├── .gitignore
├── .git/
└── src/
    └── main.rs
```

Perintah tadi juga akan meng-inisialisasi git repositori baru bersama dengan file `.gitignore`. Git Files tidak akan dibuat, jika Anda menjalankan perintah `cargo new` didalam sebuah git repositori.

Buka file *Cargo.toml* dan Anda akan melihat yang hampir seperti ini:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

Bagian pertama `[package]`, bagian ini adalah untuk mengonfigurasi package kita.

Bagian kedua `[dependencies]`, disini adalah tempat untuk menambahkan dependensi. Di Rust, sebuah *package* disebut *crates*.

Buka file *src/main.rs*.

```rust
fn main() {
  println!("Hello, world!");
}
```

Cargo juga membuat program `Hello, world` untuk Anda. Cargo membantu Anda untuk mengatur proyek Anda, Cargo mengharapkan *source files* Anda diletakkan didalam folder *src*.

## Menjalankan dan Build Proyek Cargo

Untuk mem-build proyek Cargo, Anda dapat jalankan perintah berikut:

```bash
$ cargo build
```

Perintah diatas akan membuat file executable di *target/debug/hello_cargo*. Anda dapat menjalankan file tersebut dengan perintah:

```bash
$ ./target/debug/hello_cargo
Hello, world!
```

Menjalanan perintah `cargo build` untuk yang pertama kali, Cargo akan membuat file *Cargo.lock*. File ini akan melacak versi dependensi dengan tepat di proyek Anda.

Kita baru saja  mem-build proyek kita dengan `cargo build` dan menjalankannya  dengan `./target/build/hello_cargo`, tapi kita juga bisa menggunakan perintah `cargo run` untuk meng-compile code dan menjalankan file executable yang dihasilkan.

```bash
$ cargo rus
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo juga menyediakan perintah `cargo check` untuk mengecek program tanpa membuat file executable.

## Build Untuk Release

Ketika proyek Anda sudah siap untuk release, Anda dapat menggunakan perintah `cargo build --release` untuk meng-compile-nya dengan pengoptimalan. Perintah ini akan membuat file executable di folder *target/release*.
