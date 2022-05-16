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
