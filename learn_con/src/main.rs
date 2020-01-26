fn main() {
    //if 
    let y = 0;
    if y == 1 {
        println!("y = 1");
    }

    //if-else
    if y == 1 {
        println!("y = 1");
    } else {
        println!("y != 1");
    }

    //if - else if - else
    println!("++++++++++++");
    let y = 2;
    if y == 1 {
        println!("y = 1");
    } else if y == 0 {
        println!("y = 0");
    } else if y == 2 {
        println!("y = 2");
    } else {
        println!("other");
    }

    //let中使用if
    let condition = true;
    let x = if condition {
        5
    } else {
        6
        //"six" //error
    };
    
    println!("x = {}", x);

    //loop
    let mut counter = 0;
    loop {
        println!("in loop");
        if counter == 10 {
            break;
        }

        //counter = counter + 1;
        counter += 1;
    }
    
    let result = loop {
        counter += 1;
        if counter == 20 {
            break counter*2;
        }
    };
    println!("result = {}", result);

    //while
    let mut i = 0;
    while i != 10 {
        i += 1;
    }
    println!("i = {}", i);

    //for
    println!("+++++++++++++++");
    let arr:[u32; 5] = [1, 2, 3, 4, 5];
    //for element in arr.iter() {
    for element in &arr {
        println!("element = {}", element);
    }
    println!("Hello, world!");
}
