fn main() {
    /*if Expressions*/
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /*Multiple Conditions with else if*/
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    /*Using if in a let Statement*/
    let condition = true;
    let number = if condition { 5 } else { 6 };

    /*Repetition with Loops*/
    /*    loop {
            println!("again!");
        }
    */
    /*Returning Values from Loops*/
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    /*Conditional Loops with while*/
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    /*Looping Through a Collection with for*/
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    for element in &a {
        println!("the value is: {}", element);
    }
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    let mut index = 0;
    while index < 4 {
        println!("{}!", index);
        index += 1
    }
    for number in (0..a.len()).rev() {
        println!("{}!", number);
    };
    println!("LIFTOFF!!!");
}
