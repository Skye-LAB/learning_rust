# Control Flow

## if else

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

## else if

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

## else

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

# if di dalam let

Kita juga bisa menuliskan if didalam let statement.

```rust
    let condition = true;
    let number = if condition { 5 } else { 6 };
```
