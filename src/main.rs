fn main() {
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
