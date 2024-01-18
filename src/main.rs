use std::io;

fn main() {
    variables();
    println!();
    scalar_types();
    println!();
    compound_types();
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

fn compound_types() {
    //tuples
    let tup: (i32, f64, u8) = (-500, 6.4, 2);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let negative_five_hundred = tup.0;
    let two = tup.2;

    println!(
        "Negative five hundred: {negative_five_hundred}, two: {two}"
    );

    //arrays
    let a: [u8; 5] = [1, 2, 3, 4, 5];

    let first_element = a[0];
    println!("The first element of the array is: {first_element}");

    fn runtime_error() {
        let array = [3; 5];

        println!("Enter array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line.");

        let index: usize = index
            .trim()
            .parse()
            .expect("Entered index is not a number.");
        
        let element = array[index];

        println!(
            "The value of the element at index {index} is: {element}."
        );
    }
}
