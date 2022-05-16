fn main() {
    let word = String::from("Hello world!");

    // string slice
    let s = &word[..5];

    let first_word = first_word(&word);

    // ini error karena tidak boleh ada mutable
    // dan immutable reference bersamaan
    // word.clear();

    print!("The first word is {}", first_word);
}

// fungsi first_word mengambil string literal sebagai parameter
// dan mereturn string literal juga
fn first_word(s: &str) -> &str {
    // mengubah string menjadi byte
    let bytes = s.as_bytes();

    // meniterasi setiap item pada bytes
    for (i, &item) in bytes.iter().enumerate() {
        // cek apakah spasi atau bukan
        if item == b' ' {
            // return string slice dari 0 sampai i
            return &s[..i];
        }
    }

    // return seluruh string
    &s[..]
}
