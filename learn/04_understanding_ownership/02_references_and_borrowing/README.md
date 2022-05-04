# Reference And Borrowing

## Reference

Sebelumnya kita perlu menggunakan tuple untuk mengembalikan ownership dari sebuah `String`. Dengan *reference* kita tidak perlu menggunakan tuple, reference adalah seperti pointer, yaitu sebuah alamat yang dengannya kita dapat mengakses data yang tersimpan di alamat itu. Untuk menggunakan reference kita dapat menggunakan tanda `&`.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Di program diatas, `&s1` adalah sebuah reference dari `s1`, saat `&s1` keluar dari sebuah scope nilainya `s1` tidak akan didrop, karena reference tidak memiliki ownership.

## Borrowing

Seperti di kehidupan nyata, jika seseorang memiliki(own) sesuatu, Anda dapat meminjam(borrow) dari mereka. Setelah selesai, Anda harus mengembalikannya, Anda tidak memilikinya.

Reference, secara default juga immutable, jadi kita tidak dapat mengubahnya.

## Mutable Reference

Untuk membuat mutable reference, kita dapat menambahkan keyword `mut`.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Namun, Anda hanya dapat memiliki satu mutable reference ke sebuah data dalam satu waktu.

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

Program diatas akan error, karena miliki dua mutable reference untuk satu data. Rust membatasi hal
tersebut untuk mencegah terjadinya *data race* atau balapan data. Data race sama seperti balapan
pada umumnya yang terjadi karena tiga perilaku ini:

  - Dua atau lebih pointer mengakses data yang sama pada saat yang sama.
  - Sedikitnya, satu pointer digunakan untuk mengubah data.
  - Tidak ada mekanisme untuk menyinkronan akses ke data.

Namun Anda bisa menggunakan kurung kurawal untuk membuat scope baru.

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 keluar dari scope, jadi kita dapat membuat reference baru.

let r2 = &mut s;
```

Juga kita tidak bisa memiliki mutable reference dan immutable reference di saat yang sama. Karena
immutable reference tidak berharap nilainya tiba-tiba berubah.

```rust
let mut s = String::from("hello");

let r1 = &s; // tidak masalah
let r2 = &s; // tidak masalah
let r3 = &mut s; // masalah

println!("{}, {}, and {}", r1, r2, r3);
```

Perlu diingat, bahwa scope reference berakhir disaat reference itu terakhir digunakan.

```rust
let mut s = String::from("hello");

let r1 = &s; // tidak masalah
let r2 = &s; // tidak masalah
println!("{} and {}", r1, r2);
// variable r1 dan r2 tidak digunakan lagi setelah ini.

let r3 = &mut s; // tidak masalah
println!("{}", r3);
```

## Dangling Reference

Di Rust, compiler menjamin reference pasti memiliki data.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle mengembalikan(return) reference dari String
    let s = String::from("hello"); // s masuk ke scope

    &s // kita mengembalikan reference sebuah String, s
} // disini, s keluar dari scope dan didrop
```

Program diatas akan error, karena `&s` menunjuk sebuah invalid memori.

```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///reference_and_borrowing)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` due to previous error
```

Error diatas mengacu pada sebuah fitur *lifetime*, yang akan kita pelajari pada [Chapter 10](../../10_generic_types_traits_and_lifetimes). Kita dapat melihat pesan errornya adalah:

```
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```
