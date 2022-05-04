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
    let s = String::from("hello"); // s masuk ke scope

    takes_ownership(s); // nilai s move ke fungsi
                        // jadi s tidak valid disini

    let x = 5; // x masuk ke scope

    makes_copy(x); // x dicopy ke dalam fungsi,
                   // jadi x masih valid di sini

    // nilai return dan ownership
    let s1 = gives_ownership(); // gives_ownership memindahkan(move) nilai retun ke s1

    let s2 = String::from("hello"); // s2 masuk ke scope

    let s3 = takes_and_gives_back(s2); // s2 berpindah(move) ke
                                       // takes_and_gives_back, yang juga
                                       // memindahkan(move) nilai return ke s3
}

fn takes_ownership(some_string: String) { // some_string masuk ke scope
    println!("{}", some_string);
} // disini, some_string keluar dari scope dan 'drop' dipanggil

fn makes_copy(some_integer: i32) { // some_integer masuk ke scope
    println!("{}", some_integer);
} // disini, some_integer masuk ke scope.

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string masuk ke scope

    some_string // some_string di-return dan pindah(move)
                // ke dimana fungsi dipanggil
}

fn takes_and_gives_back(a_string: String) -> String { // a_string masuk ke scope

    a_string // a_string di-return dan berpindah ke dimana fungsi dipanggil
}
