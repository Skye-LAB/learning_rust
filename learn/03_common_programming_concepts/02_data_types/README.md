# Data Types

## Tipe Scalar

Tipe scalar mewakili nilai tunggal, diantaranya integer, floating-point, numbers, Boolean dan karakter.

### Integer

Di Rust integer dibagi menjadi dua, yaitu signed(dapat menyimpan nilai negatif) dan unsigned(tidak dapat menyimpan nilai negatif). berikut tabel daftar integer:

| Length | Signed | Unsigned |
| :----: | :----: | :------: |
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| 128-bit| i128   | u128     |
| arch   | isize  | usize    |

Setiap varian signed dapat menyimpan data sebanyak -(2^n-1^) sampai 2^n-1^-1, dimana *n* adalah besaran bit yang varian itu pakai.

Selain itu, tipe `usize` dan `isize` memiliki nilai yang bergantung pada arsitektur dari komputer yang Anda gunakan, dimana akan memiliki nilai 64 bit jika Anda menggunakan komputer berarsitektur 64-bit dan 32 bit jika Anda menggunakan komputer berarsitektur 32-bit.