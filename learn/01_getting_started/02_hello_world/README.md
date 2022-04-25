# Hello, World!

## Penjelasan Program

struktur dari Rust adalah sebagai berikut:

```rust
fn main() {
  // programmu berada disini   
}
```

fungsi `fn main()` akan dijalankan pertama kali di setiap program Rust. Badan fungsi akan terdapat di dalam kurung kurawal `{}`.

didalam fungi `main` terdapat kode:

```rust
println!("Hello, world!");
```

Pertama, `println!` merupakan sebuah macro, jadi bukan sebuah fungsi biasa. Macro akan kita pelajarin di materi ke-19. Macro biasanya diakhiri dengan tanda seru `!`

Kedua, kita mengakhiri sebuah baris dengan tanda titik koma `;`

## Compiling

Sebelum dapat menjalan program Rust, kita meng-compilenya dengan Rust compiler yaitu `rustc`. Untuk meng-compile program Rust, gunakan perintah berikut:

```
$ rustc main.rs
```

Jika kalian pernah belajar C/C++, Anda akan melihat ini mirip `gcc` atau `clang`. Setelah compiling berhasil, Rust akan menghasilkan sebuah executable binary file.

Anda dapat melihatnya dengan perintah `ls`.

```bash
$ ls
main    main.rs
```

Jika kalian menjalankan file `main`, akan mengeluarkan output `Hello, world!`.

```bash
$ ./main
Hello, world!
```
