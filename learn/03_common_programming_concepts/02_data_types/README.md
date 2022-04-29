# Data Types

## Tipe Scalar

Tipe scalar mewakili nilai tunggal, diantaranya integer, floating-point, numbers, Boolean dan karakter.

### Integer

Di Rust integer dibagi menjadi dua, yaitu signed(dapat menyimpan nilai negatif) dan unsigned(tidak dapat menyimpan nilai negatif).

```rust
let x = 5 // i32
let y: u32 = 10 // u32
```

Secara default tipe integer adalah `i32`.

Berikut tabel daftar tipe integer:

| Length | Signed | Unsigned |
| :----: | :----: | :------: |
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| 128-bit| i128   | u128     |
| arch   | isize  | usize    |

Setiap varian signed dapat menyimpan data sebanyak -(2<sup>n-1</sup>) sampai 2<sup>n-1</sup>-1, dimana *n* adalah besaran bit yang varian itu pakai.

Selain itu, tipe `usize` dan `isize` memiliki nilai yang bergantung pada arsitektur dari komputer yang Anda gunakan, dimana akan memiliki nilai 64 bit jika Anda menggunakan komputer berarsitektur 64-bit dan 32 bit jika Anda menggunakan komputer berarsitektur 32-bit.

Anda juga dapat menulisakan integer literals, contoh:

| Number literals | Example |
| :-------------: | :-----: |
| Decimal         | `98_22` |
| Hex             | `0xff`  |
| Octal           | `0o77`  |
| Binary          |`0b11_00`|
| Byte(`u8` only) | `b'A'`  |

### Floating-Point

Di Rust tipe floating-point dibagi menjadi dua tipe, yaitu `f32` dan `f64`.

```rust
let x = 5.0 // f64
let y: f32 = 10.0 // f32
```

Secara default tipe floating-point adalah `i32`.

### Operasi Numerik

Rust juga mendukung operasi matematika sederhana seperti penjumlahan, pengurangan, perkalian, pembagian dan mencari sisa hasil bagi atau biasa disebut modulus.

```rust
// penjumlahan
let sum = 5 + 10;

// pengurangan
let difference = 95.5 - 4.3;

// perkalian
let product = 4 * 30;

// pembagian
let quotient = 56.7 / 32.2;
let floored = 2 / 3; // Results in 0

// modulus
let remainder = 43 % 5;
```

### Boolean

Tipe boolean menyimpan dua nilai yaitu `true` dan `false`.

```rust
let t = true;
let f: bool = false;
```

### Karakter

Contoh penulisan `char` di Rust:

```rust
char c = 'z';
let heart_eyed_cat = '😻';
```

Di Rust `char` dituliskan dengan tanda petik satu `' '`, berbeda dengan string yang menggunakan tanda petik dua `" "`. di Rust tipe `char` memiliki besaran empat bytes dan mewakili nilai Unicode tunggal, yang berarti dapat mewakili lebih daripada hanya ASCII. Karakter Jepang, China atau bahkan emoji, valid sebagai nilai `char`. Akan lebih dijelaskan di [Chapter 8](../../08_common_collections) tentang **Storing UTF-8 Encoded Text With String**

## Tipe Compound

Tipe compound atau gabungan dapat menyimpan banyak data ke dalam satu tipe. Rust memiliki dua tipe compound, yaitu tuple dan array.

### Tuple

Tuple memiliki pangjang yang telah ditentukan, saat sudah dideklarasikan, panjang tuple tidak dapat ditambahkan. Contoh mendeklarasikan tuple:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

Tipe nilai didalam value boleh berbeda.

Untuk mengakses setiap nilai didalam tuple, kita dapat menggunakan tanda `.`.

```rust
let x: (i32, f64, u8) = (500, 6.4, 1);

let five_hundred = x.0;

let six_point_four = x.1;

let one = x.2;
```

Kita juga bisa melakukan *destructure assignment* kepada tuple, contoh:

```rust
let tup = (500, 6.4, 1);

let (x, y, z) = tup;

println!("The value of y is: {}", y);
```
