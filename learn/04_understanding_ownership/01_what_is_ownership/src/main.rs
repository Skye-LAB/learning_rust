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
}
