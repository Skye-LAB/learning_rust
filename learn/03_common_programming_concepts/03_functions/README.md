# Functions

## Mendeklarasikan Sebuah Fungsi

Untuk mendeklarasikan sebuah fungsi, kita menggunakan keyword `fn` diikuti dengan name fungsi.

```rust
fn another_function() {}
```

## Menambahkan Paramenter

Untuk menambahkan parameter, kita dapat sisipkan diantara tanda `()`.

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

Kita perlu menambahkan tipe data pada setiap parameter.

## Mengembalikan(return) Sebuah Nilai

Untuk mengebalikan data pada sebuah fungsi, sebelumnya kita perlu tahu tipe data apa yang akan dikembalikan(return type). Untuk menambahkan return type, kita dapat menambahkan `->` diikuti tipe data, setelah pendeklarasian fungsi.

```rust
fn double(n: u32) -> u32 {}
```

Untuk mengembalikan sebuah nilai dari suatu fungsi, kita dapat menggunakan kode seperti ini:

```rust
fn double(n: u32) -> u32 {
    return n * n;
}
```

Atau dapat disederhanakan dengan:

```rust
fn double(n: u32) -> u32 {
    n * n
}
```
