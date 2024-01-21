use std::io;

fn main() {
    variables();
    println!();
    scalar_types();
    println!();
    compound_types();
    println!();
    functions_and_comments();
    println!();
    conditions();
    println!();
    loops();
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

    // runtime_error(a);
}

fn runtime_error(array: [u8; 5]) {
    // let array = [3; 5];

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

fn functions_and_comments() {
    fn labeled_number(value: i32, label: char) {
        println!("The value with label is {value}{label}");
    }

    labeled_number(5, 'h');

    let y = {
        // this line is a statement because it contains semicolon
        // statements return () by default
        let x = 3;
        // the line doesn't have a semicolon because its an expression
        x + 1
    };

    println!("The value of y is: {y}");

    fn five() -> i32 {
        5
    }

    let x = five();
    println!("The value of x is: {x}");

    fn nine(x: i32) -> i32 {
        x + five() + 10
    }

    let z = nine(x); // should output 20
    println!("The value of z is: {z}");
}

fn conditions() {
    let number = 3;

    if number < 7 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // the if condidtion must be an bool
    if number == 3 {
        println!("The number is three.")
    }

    if number % 2 == 0 {
        println!("The number {number} is divisible by 2");
    } else if number % 3 == 0 {
        println!("The number {number} is divisible by 3");
    } else {
        println!("The number {number} is not divisible by 2 or 3");
    }

    let condidtion = true;
    // when assigining values from condition
    // the values must be the same type 
    let number = if condidtion { 10 } else { 20 };
    println!("The new value of number is {number}");
}

fn loops() {
    fn counter_loop() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result of the loop is {result}");
    }

    fn nested_loop() {
        let mut count = 0;

        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }

    fn conditional_loop() {
        let mut number = 3;

        while number != 0 {
            println!("{number}");

            number -= 1;
        }

        println!("End of count!");
    }

    fn collection_loop() {
        let a = [10, 20, 30, 40, 50];
        
        for element in a {
            println!("The value is: {element}");
        }

        println!();

        for number in (1..4).rev() {
            println!("{number}");
        }
        println!("End of for loop count!");
    }

    fn fa_to_cel() {
        println!("Type temperature in farenheit to convert it to celcius:");

        let mut fa = String::new();

        io::stdin()
            .read_line(&mut fa)
            .expect("Failed to read line.");

        let fa: i32 = match fa.trim().parse() {
            Ok(num) => num,
            Err(_) => return println!("You provided invalid number."),
        };

        let cel = (fa - 32) * 5 / 9;

        println!("{fa} fahrenheit converted to celcius is {cel}.");
    }

    fn get_fib_number() {
        println!("Type in nth fibonnaci number:");

        let mut fib_index = String::new();

        io::stdin()
            .read_line(&mut fib_index)
            .expect("Falied to read line.");

        let fib_index: u32 = match fib_index.trim().parse() {
            Ok(num) => num,
            Err(_) => return println!("You provided invalid number."),
        };

        let (mut a, mut b, mut helper) = (1, 1, 1);
        for index in 2..fib_index {
            helper = b;
            b = a + b;
            a = helper;
        }

        println!("The {fib_index} index fib number is {b}");
    }

    fn carol_lyrics() {
        let start = [
            "On the",
            "day of Christmas",
            "My true love brought to me",
        ];

        let verses = [
            "Twelve drummers drumming",
            "Eleven pipers piping",
            "Ten lords a-leaping",
            "Nine ladies dancing",
            "Eight maids a-milking",
            "Seven swans a-swimming",
            "Six geese a-laying",
            "Five golden rings",
            "Four calling birds",
            "Three French hens",
            "Two turtle doves",
            "And a partridge in a pear tree",
        ];

        let counters = [
            "first",
            "second",
            "third",
            "fourth",
            "fifth",
            "sixth",
            "seventh",
            "eighth",
            "ninth",
            "tenth",
            "eleventh",
            "twelfth",
        ];

        if verses.len() != counters.len() {
            return println!("The verses and counters lengths of the carol are not equal.");
        }

        for index in 0..counters.len() {
            println!("{} {} {}\n{}", start[0], counters[index], start[1], start[2]);
            
            for verse_i in (1..(index + 2)).rev() {
                if index == 0 {
                    println!("A partridge in a pear tree");
                } else {
                    println!("{}", verses[verses.len()-verse_i]);
                }
            }

            println!();
        }
    }

    counter_loop();
    println!();
    nested_loop();
    println!();
    conditional_loop();
    println!();
    collection_loop();
    println!();
    fa_to_cel();
    get_fib_number();
    carol_lyrics();
}