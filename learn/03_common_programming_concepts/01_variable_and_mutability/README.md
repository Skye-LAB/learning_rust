# Variable And Mutability

## Variabel

Variabel secara default adalah immutable untuk membuatnya mutable tambahkan keyword `mut`.

```rust
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 10;
    println!("The value of x is {}", x);
```

## Konstanta

Konstanta diRust biasa di tulis dengan huruf kapital semua dan underscore untuk memisahkan kata. Konstanta selalu immutable, artinya pemakaian `mut` tidak diperbolehkan di konstanta. Penulisan konstanta haru diikuti dengan anotasi tipe data, kita akan mempelajarinya di bagian [Data
Types](../02_data_types).

```rust
    const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;
    println!("1 hour in seconds is {}", ONE_HOUR_IN_SECONDS);
```

## Shadowing

Seperti yang Anda lihat di [Chapter 2](../../02_guessing_game), Anda mendeklarasikan variabel baru
yang sama namanya dengan sebelumnya. Metode ini disebut *shadowing*, variabel pertama di *shadow*
dengan variabel kedua, artinya nilai dari variabel kedualah yang program lihat.

```rust
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
```

Shadowning berbeda dengan menggunakan keyword `mut` karena kita bisa secara efektif membuah sebuah
variabel baru, kita bisa mengganti tipe dari nilai tetapi menggunakan nama yang sama. Contohnya:

```rust
    let spaces = "   ";
    let spaces = spaces.len();
```
