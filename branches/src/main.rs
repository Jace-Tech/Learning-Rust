

fn main() {
//    CONTROL FLOW
//    - if
//    - loop

    let num = 8;

    if num > 5 {
        println!("{num} is greater than 5")
    }



    let temp = 24;

    if temp >= 40{
        println!("It's very hot");
    }
    else if temp >= 25 && temp < 40 {
        println!("It's hot")
    }
    else if temp >= 17 && temp < 25 {
        println!("It's cool")
    }
    else {
        println!("It's cold!")
    }


//    Using if to assign a value to a variable
    let my_num = if num > 10 { num } else { temp };

    println!("My num = {my_num}");
}
