# Instalasi Rust

Langkah pertama, install Rust. Kita akan men-download Rust menggunakan `rustup`, sebuah command line tool untuk mengelola versi-versi Rust dan alat-alat yang terkait.

## Install rustup di Linux

Jika kalian menggunakan Linux, buka terminal dan jalankan perintah berikut:

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Kalian mungkin akan ditanyai password kalian saat instalasi berlangsung. Jika instalasi sukses, akan muncul baris berikut:

```bash
Rust is installed now. Great!
```

Untuk mengecek apakah Rust sudah berhasil terinstall dengan benar, gunakan perintah berikut:

```bash
rustc --version
```

Jika Rust terinstall dengan benar di komputer Anda maka Anda akan melihat baris seperti berikut:

```bash
rustc 1.60.0 (7737e0b5c 2022-04-04)
```

## Update dan Uninstall

Meng-update Rust dengan `rustup` sangatlah mudah, buka terminal Anda dan jalankan perintah berikut:

```bash
rustup update
```

Perintah berikut adalah untuk meng-uninstall `rustup`.

```bash
rustup self Uninstall
```

## Dokumentasi

Untuk melihat dokumentasi Rust secara lokal, sehingga Anda dapat melihatnya secara offline, jalankan perintah `rustup doc` untuk membuka dokumentasi lokal di browser Anda.
