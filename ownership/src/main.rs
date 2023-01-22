fn main() {
//    let s1 = String::from("Hello");
//    let s2 = s1;
    let name = "Jace Alex";
    logger(name);

    println!("{name}");
}

fn logger(name:&str) {
    println!("{name}");
}