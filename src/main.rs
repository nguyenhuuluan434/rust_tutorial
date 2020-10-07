/*Function definitions in Rust start with fn and have a set of parentheses after the function name.*/
fn main() {
    println!("Hello, world!");
    another_function(5, 9);
    function_expression();
    let mul = multi(10, 20);
    println!("{:?}", mul);
}

/*fn another_function() {
    println!("Another function.");
}*/
/*Functions can also be defined to have parameters, which are special variables that are part of a function’s signature.*/
fn another_function(x: i32, y: i64) {
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", y);
}

fn function_expression() {
    let x: i32 = 0;
    let y = {
        let x = 4;
        x + 1;
    };
    println!("The value of x is: {}", x);
    println!("The value of x is: {:?}", y);
}

/*Functions can return values to the code that calls them*/
fn multi(x: i32, y: i32) -> i32 {
    return x * y;
}
