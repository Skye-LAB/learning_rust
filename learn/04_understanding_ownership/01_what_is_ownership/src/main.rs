fn main() {
    {
        // s tidak valid disini, karena belum dideklarasi
        let s = "hello"; // s valid mulai dari sini

        // lakukan sesuatu dengan s
    } // scope berakhir, s tidak lagi valid

    {
        let s = String::from("hello"); // s valid mulai dari sini

        // lakukan sesuatu dengan s
    } // scope berakhir, 'drop' dipanggil dan s tidak lagi valid

    // copy
    let x = 5;
    let y = x;

    // move
    let s1 = String::from("hello");
    let s2 = s1;

    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // ownership dan fungsi
    let s = String::from("hello");  // s masuk ke scope

    takes_ownership(s);             // nilai s move ke fungsi
                                    // jadi s tida valid disini

    let x = 5;                      // x masuk ke scope

    makes_copy(x);                  // x dicopy ke dalam fungsi,
                                    // jadi x masih valid di sini
}

fn takes_ownership(some_string: String) { // some_string masuk ke scope
    println!("{}", some_string);
} // disini, some_string keluar dari scope dan 'drop' dipanggil

fn makes_copy(some_integer: i32) { // some_integer masuk ke scope
    println!("{}", some_integer);
} // disini, some_integer masuk ke scope.
