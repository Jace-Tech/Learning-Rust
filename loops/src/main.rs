fn main () {
//    loop {
//        println!("This is a loop");
//    }

//    Making a counter app
    let mut counter = 0;

    let mut final_count = loop {
        counter += 1;
        if counter >= 10 {
            break counter;
        }
    };

    println!("Final count {final_count}");


    let mut count = 0;

    'first: loop {
        println!("Count: {count}");
        let mut limit = 2;
        loop {
            limit -= 1;
            println!("Limit: {limit}");
            if count == 2 {
                break 'first;
            }
            if limit == 0 {
                break;
            }
        }
        count += 1;
    }
}





