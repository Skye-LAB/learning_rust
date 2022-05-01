# What Is Ownership

## Stack Dan Heap

Pertama, kita harus tahu tentang apa itu stack dan heap. Stack dan heap adalah dua tempat penyimpanan yang berbeda di komputer.

  - Stack sangat cepat, namun ukurannya tidak dapat bertambah atau berkurang. Sedangkan heap tidak terlalu cepat namun ukuran heap dapat berkurang maupun bertambah.
  
  - Rust perlu mengetahui ukuran sebuah variabel saat compile time. Sehingga variabel sederhana seperti `i32` akan ditaruh kedalam stack.

  - Tapi ada beberapa tipe yang tidak bisa kita ketahui ukurannya saat compile time, sedangkan stack perlu berapa ukurannya secara tepat. Lalu bagaimana? Pertama, kita dapat menaruh data tersebut ke dalam heap, karena heap dapat menyimpan data dengan berbagai ukuran. Lalu kita simpan pointer dari data tersebut ke dalam stack, hal ini sangat masuk akal karena kita tahu ukuran dari sebuah pointer.

## Ownership Rules

Pertama, kita harus tahu apa saja rules atau aturan apa saja dalam ownership.

  - Setiap nilai dari Rust mempunyai variabel yang dipanggil *owner*.

  - Hanya bisa terdapat satu *owner* dalam satu waktu.

  - Ketika suatu *owner* keluar dari sebuah scope, nilainya akan dihapuskan.

## Scope Variabel

Contoh ownership pertama adalah melihat scope suatu variabel.

```rust
{
    // s tidak valid disini, karena belum dideklarasi
    let s = "hello"; // s valid mulai dari sini

    // lakukan sesuatu dengan s
} // scope berakhir, s tidak lagi valid
```

Variabel `s` merupakan tipe string literal(`&str`) yang nilainya adalah `"hello"`. Variabel `s` hanya valid didalam tanda `{}`, sehingga variabel `s` tidak dapat diakses diluar tanda `{}` atau diluar scope.

## Tipe String

Tipe `String` berbed dengan string literal, dimana ukuran tipe `String` dapat bertambah dan berkurang dan tipe `String` juga mutable sedangkan tipe string literal immutable. Sehingga nilai tipe `String` akan disimpan ke dalam heap.

```rust
let s = String::from("hello");
```

Ketika kode diatas dijalankan, nilai `String` `"hello"` akan disimpan di heap, lalu pointernya akan disimpan ke dalam variabel `s`.

Mengubah nilai tipe `String`:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```

Ketika suatu variabel bertipe `String` keluar dari scope, Rust akan memanggil fungsi spesial, yaitu `drop`, dimana fungsi ini akan membebaskan memori yang ada di heap.

```rust
{
    let s = String::from("hello"); // s valid mulai dari sini

    // lakukan sesuatu dengan s
} // scope berakhir, 'drop' dipanggil dan s tidak lagi valid
```

## Copy

Apa yang terjadi ketika kode ini dijalankan?

```rust
let x = 5;
let y = x;
```

Mungkin Anda berpikir, kaitkan nilai `5` ke variabel `x` dan variabel `x` ke variabel `y`, jadi kedua variabel tersebut mempunyai nilai `5`. Pertanyaan tersebut benar, karena nilai integer sudah diketahui ukurannya akan disimpan kedalam stack, jadi membuat sebuah copy dari suatu nilai sangatlah cepat.

Lalu bagaimana dengan nilai yang disimpan di dalam heap?

## Move

Coba pikirkan apa yang terjadi jika kode ini dijalankan.

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Apakah sama seperti sebelumnya? Tidak. Pertama kita harus tahu, `String` terbuat dari pointer yang menunjuk ke memori yang menyimpan content dari string tersebut. Pointer ini disimpan di stack. Ketika kita menetapkan(assign) `s1` ke `s2`, pointer tersebut dikopi.

Lalu bagaimana salah satu variabel diatas keluar dari sebuah scope, bagaimana jika salah satu variabel tersebut ter`drop`, apakah variabel yang lainnya masih menyimpan suatu data. Untuk menghindari masalah ini, setelah baris `let s2 = s1`, Rust membuat variabel `s1` tidak valid lagi.

## Clone

Kita juga dapat mengkopi data dari heap menggunakan fungsi `clone`.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

## Ownership dan Fungsi

Menyertakan(passing) variabel ke sebuah fungsi, sama jadinya seperti assignment diatas.

```rust
fn main() {
    let s = String::from("hello");  // s masuk ke scope

    takes_ownership(s);             // nilai s move ke fungsi
                                    // jadi s tidak valid disini

    let x = 5;                      // x masuk ke scope

    makes_copy(x);                  // x dicopy ke dalam fungsi,
                                    // jadi x masih valid di sini

} // disini, x keluar dari scope.

fn takes_ownership(some_string: String) { // some_string masuk ke scope
    println!("{}", some_string);
} // disini, some_string keluar dari scope dan 'drop' dipanggil

fn makes_copy(some_integer: i32) { // some_integer masuk ke scope
    println!("{}", some_integer);
} // disini, some_integer masuk ke scope.
```

## Nilai Return

Mengembalikan(return) sebuah nilai juga dapat memberikan ownership.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership memindahkan(move) nilai retun ke s1

    let s2 = String::from("hello");     // s2 masuk ke scope

    let s3 = takes_and_gives_back(s2);  // s2 berpindah(move) ke
                                        // takes_and_gives_back, yang juga
                                        // memindahkan(move) nilai return ke s3
} // disini scope s3 berakhir, s2 dipindah(move), juga scope s1 berakhir

fn gives_ownership() -> String {

    let some_string = String::from("yours"); // some_string masuk ke scope

    some_string                              // some_string di-return dan pindah(move)
                                             // ke dimana fungsi dipanggil
}

fn takes_and_gives_back(a_string: String) -> String { // a_string masuk ke scope

    a_string  // a_string di-return dan berpindah ke dimana fungsi dipanggil
}
```

Lalu bagaimana jika kita ingin membuat sebuah fungsi menggunakan suatu nilai tanpa mengambil ownership? Kita bisa return banyak nilai dengan tuple.

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```
