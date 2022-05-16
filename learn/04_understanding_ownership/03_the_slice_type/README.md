# The Slice Type

## String Slice

*string slice* atau irisan string merupakan sebuah reference ke bagian dari String.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

Juga dapat disederhanakan sebagai berikut:

```rust
let s = String::from("hello world");

let hello = &s[..5];
let world = &s[6..];
let hello_world = &s[..];
```

## String Literal Adalah Slice(Irisan)

String literal, juga merupakan irisan, itulah mengapa mereka immutable.

```rust
let s = "Hello, world!";
```

## Irisan Lainnya

Irisan juga dapat diterapkan pada tipe data lainnya.

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

## Contoh

Ada sebuah program yang akan mengembalikan kata pertama dalam sebuah kalimat.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return string slice dari 0 sampai i
            return &s[..i];
        }
    }

    &s[..]
}
```

Pertama, fungsi `first_word()` menerima string literal sebagai parameter, untuk agar dapat dijalankan pada tipe `String` maupun string literal.

```rust
let s = String::from("hello world");

let word = first_word(&s);
```

Atau:

```rust
let s = "hello world";

let word = first_word(s);
```

Karena kita ingin mengecek setiap element pada string, kita ubah string ke sebuah array byte.

```rust
let bytes = s.as_bytes();
```

Kemudian, kita buat sebuah iterator untuk array byte tadi.

```rust
for (i, &item) in bytes.iter().enumerate() {}
```

Kita akan pelajari iterator lebih dalam di [Chapter 13](../../13_iterators_and_closures). Untuk sekarang, kita perlu tahu bahwa `iter` adalah sebuah method yang akan mengmbalikan setiap element dalam collection dan `enumerate` membungkus hasil dari `iter` dan mengmbalikan setiap element ke dalam sebuah tuple. Item pertama adalah indeks dan yang kedua adalah reference ke element tersebut.


Karena method `enumerate` mengmbalikan sebuah tuple, jadi kita dapat melakukan destructure kepadanya, dengan `i` untuk setiap indeks dan `&item` untuk setiap element bytenya.

Lalu, kita cari nilai byte yang merepresentasikan spasi dengan byte literal.

```rust
if item == b' ' {
    return &s[..i];
}
```

Kemudian, mengmbalikan sebagian atau keseluruhan string tersebut.

```bash
 Compiling the_slice_type v0.1.0 (file:///the_slice_type)
    Finished dev [unoptimized + debuginfo] target(s) in 2.69s
     Running `target/debug/the_slice_type`
The first word is Hello
```
