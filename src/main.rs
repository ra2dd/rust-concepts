fn main() {
    variables();
    println!();
    scalar_types();
}

fn variables() {
    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;
    println!("The value of changed mutable x is {x}");

    {
        let x = x * 2;
        println!("The value of shadowed value of x in inner scope is {x}");
    }

    let x = x + 1;
    println!("New shadowed value of x is {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("String converted to integer: {spaces}");
}

fn scalar_types() {
    let hex_number = 0x64;
    println!("Number written in hex format: {hex_number}");

    let decimal_number = 5_333;
    println!("Number writeen in decimal format: {decimal_number}");

    let unsigned_number: u8 = 255;
    println!("Unsigned number: {unsigned_number}");

    let float_number: f64 = 5.0001;
    println!("Float number: {float_number}");

    let quotient = 75.8 / 26.5;
    println!("The quotient from division is {quotient}");

    let t: bool = true;
    println!("Boolean: {t}");

    let c: char = 'A';
    println!("Char: {c}");
}
