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
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

Variabel `s` merupakan tipe string literal(`&str`) yang nilainya adalah `"hello"`. Variabel `s` hanya valid didalam tanda `{}`, sehingga variabel `s` tidak dapat diakses diluar tanda `{}` atau diluar scope.

## Tipe String
