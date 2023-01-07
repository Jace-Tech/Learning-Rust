fn main() {
    println!("Hello from Main");
    another_function();
    say_hello("Jace");
    let result_add = add(4, 5);

    println!("The result of the add: {result_add}")
}


fn another_function() {
    println!("Hello from Another Function");
}

//Parameters in function
fn say_hello(name: &str) {
    println!("Hello {name}");
}

//Return in functions
//Rust explicitly returns the last expression in a function as it's value, Note you MUST return a type that correspond with Return type specified in the function
//fn add(x: u32, y: u32) -> u32{
//    return x + y
//}

fn add(x: u32, y: u32) -> u32{
    x + y
}

// STATEMENTS AND EXPRESSION
// STATEMENT: They are piece of code which does not return a result back, You can assign a variable to them and they usually endswith a semi-colon
// EXPRESSION: They are those piece of code that returns a result, They doesn't necessary need to be closed with a semi-colon;

