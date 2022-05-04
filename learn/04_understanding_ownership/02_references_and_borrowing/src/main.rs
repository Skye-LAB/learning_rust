fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // mutable reference
    let mut s = String::from("hello");

    change(&mut s);

    let r1 = &mut s;
    // error, karena tidap boleh ada dua mutable reference
    // let r2 = &mut s;
    //
    // println!("{}, {}", r1, r2);
}

// fungsi ini mengambil sebuah reference ke String
// sebagai parameter
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
