# Control Flow

## If expression

### if

If di Rust hampir sama dengan bahasa pemrograman lainya.

```rust
let x = 5;

if x > 0 {
    println!("x is greater than 0");
}
```

Bedanya adalah kondisi if di Rust harus bertipe data boolean.

```rust
let number = 3;

if number {
    println!("number was three");
}
```

Jika tidak maka akan terjadi error.

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///control_flow)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

### else if

Daripada membuat banyak `if` seperti ini:

```rust
let x = 5;

if x > 0 {
    println!("x is greater than 0");
}
if x == 0 {
    println!("x is 0");
}
```

Kita dapat menyederhanakan dengan `else if`.

```rust
let x = 5;

if x > 0 {
    println!("x is greater than 0");
} else if x == 0 {
    println!("x is 0");
}
```

### else

```rust
let x = 5;

if x > 0 {
    println!("x is greater than 0");
} else if x == 0 {
    println!("x is 0");
} else {
    println!("x is a negative number");
}
```

### if di dalam let

Kita juga bisa menuliskan if didalam let statement.

```rust
    let condition = true;
    let number = if condition { 5 } else { 6 };
```

## Looping

Ada tiga jenis looping pada Rust, yaitu `loop`, `while`, `for`.

### loop

Pertama, untuk membuat looping ini, kita dapat menggunakan keyword `loop`.

```rust
loop {
    println!("again!");
}
```

Untuk memanipulasi looping ini, kita dapat menggunakan `continue`, untuk melanjutkan ke putaran selanjutnya atau `break`, untuk menghentikan looping.

```rust
let mut counter = 0;

loop {
    println!("{}", counter);

    if counter == 5 {
        continue;
    } else if continue == 10 {
        break;
    }

    counter += 1;
};
```

Kita juga dapat mengembalikan nilai dari sebuah loop.

```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};

println!("The result is {}", result);
```

Output program diatas adalah `The result is 20`.

### while

Selanjutnya, kita dapat menggunakan `while` untuk looping dengan kondisi tertentu.

```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);

    number -= 1;
}
```

Program diatas, dapat dilihat `while` loop memiliki kondisi `number != 0`.

### for

Dengan `for` loop, kita dapat meniterasi suatu `array`, atau `tuple` untuk mengakses element-element didalamnya.

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {}", element);
}
```
