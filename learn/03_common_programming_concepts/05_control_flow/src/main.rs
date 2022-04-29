fn main() {
    let x = 3;

    // percabangan if else
    if x > 0 {
        println!("x is greater then 0");
    } else if x == 0 {
        println!("x is 0");
    } else {
        println!("x is a negative number");
    }

    // percabangan if else dengan let
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // looping loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
