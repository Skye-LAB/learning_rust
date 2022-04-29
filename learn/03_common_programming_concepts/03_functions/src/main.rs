fn main() {
    let x = double(5);

    println!("x is {}", x);
}

// mendeklarasikan fungsi double dengan parameter n dan return type i32
fn double(n: i32) -> i32 {
    // mengembalikan nilai dari n kali n
    n * n
}
