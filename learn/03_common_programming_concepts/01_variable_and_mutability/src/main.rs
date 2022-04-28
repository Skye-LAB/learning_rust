fn main() {
    // VARIABEL
    // variabel secara default adalah immutable
    // untuk membuatnya mutable tambahkan keyword mut
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 10;
    println!("The value of x is {}", x);

    // KONSTANTA
    // konstanta diRust biasa di tulis dengan huruf kapital semua dan
    // underscore diantara kata-kata.
    const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;
    println!("1 hour in seconds is {}", ONE_HOUR_IN_SECONDS);

    // SHADOWING
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
