fn main() {
    /*by default variables are immutable.*/
    /* by adding **mut** in front of the variable name to make them mutable*/
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = x + 1;
    println!("The value of x is: {}", x);

    /*Constants aren’t just immutable by default—they’re always immutable.*/
    const MAX_POINTS: u32 = 100_000;

    /*Shadowing,you can declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable */
    /*Shadow allow assign new data to variable with same name, the mutable variable is not*/
    let spaces = "   ";
    println!("{}", spaces);
    let spaces = spaces.len();
    println!("{}", spaces);

    /*Data Type*/
    /*Every value in Rust is of a certain data type, which tell rust how to work with that data*/
    let guess: i128 = "42".parse().expect("Not a number!");

    let decimal: i128 = 89_222_333_444_444_444_4444_444;
    println!("{}", decimal);

    let decimal = 100.0 / 2.4;
    println!("{}", decimal);

    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    /*Compound Types*/

    /*The Tuple Type*/

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    /*Can function return multiple value?*/
    println!("The value of y is: {}", y);

    /*The Array Type*/
    /*Arrays are useful when you want your data allocated on the stack rather than the heap*/
    /* An array isn’t as flexible as the vector type*/
    /**/
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];/*let a = [3, 3, 3, 3, 3];*/
}
