use std::io;

fn main() {

// SCALAR TYPES

    // INTEGERS
    // u8 - i8 ===> 8 bit
    // u16 - i16 ===> 16 bit
    // u32 - i32 ===> 32 bit ->> The default
    // u64 - i64 ===> 32 bit
    // u128 - i128 ===> 32 bit
    // isize- usize===> arch (user's system architecture)

    let num1: u8 = 70;
    println!("For 8 bit => {num1}");

    let num1: u16 = 80;
    println!("For 16 bit => {num1}");

    let num1 = 900;
    println!("For 32 bit => {num1}");

    let num1: u64 = 9000;
    println!("For 64 bit => {num1}");

    let num1: u128 = 90000;
    println!("For 128 bit => {num1}");

    let num1: usize = 90000;
    println!("For System bit => {num1}");



    // FLOATING POINT NUMBERS
    let num1: f32 = 3.6;
    println!("For Float 32 bit => {num1}");

    let num1 = 60.5;// f64
    println!("For Float 64 bit => {num1}");



    // MATHEMATICAL OPERATORS WITH NUMBERS
    const SUM: i32 = 5 + 5;
    println!("SUM -> {SUM}");

    const MINUS: i32 = 5 - 5;
    println!("MINUS -> {MINUS}");

    const DIVIDE: i32 = 5 / 5;
    println!("DIVIDE -> {DIVIDE}");

    const MULTIPLY: i32 = 5 * 5;
    println!("MULTIPLY -> {MULTIPLY}");

    const MODULUS: i32 = 5 % 5;
    println!("MODULUS -> {MODULUS}");



    // BOOLEANS

    let is_fun = true;
    println!("{is_fun}");

    let is_fun: bool = true;
    println!("{is_fun}");



    // CHAR
    let emoji = '😑';
    println!("{emoji}");
    let _char = 'z';
    println!("{_char}");


// COMPOUND TYPES

    // TUPLES

    let my_stuff: (&str, u8, char) = ("Jace", 54, '🙃');

    // Tuples can't be logged out for some reason
    // Tuples can be destructured (i.e let (a, b, c) = my_stuff)
    // Tuples can be access with the dot followed by the index (i.e my_stuff.0)
    // Empty tuples are known as UNIT

    let (name, age, emoji) = my_stuff;
    println!("name => {}, age => {}, emoji => {}", name, age, emoji);



    // ARRAYS
    // Unlike arrays in JS and other languages, Arrays in Rust have a fixed length and all the elements MUST be of same type

    let mut scores: [u32; 4] = [80, 87, 90, 100];
    let hun: u32 = scores[3];
    println!("{hun}");

    scores[3] = 90;
    let hun: u32 = scores[3];
    println!("{hun}");

    // You can also add same element many times in an array in a consize manner
    let _threes = [4, 3, 2, 1];

    println!("Please enter a number: ");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to readline");

    let index: usize = index
        .trim()
        .parse()
        .expect("Please enter a number");

    let item = _threes[index];

    println!("{item}");

}


