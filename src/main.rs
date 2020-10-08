fn main() {
    /*The ownership it enable Rust make memory safety*/
    /*features: borrowing, slices*/
    {                           // s is not valid here, it’s not yet declared
        let s = set_string();   // s is valid from this point forward
        // do stuff with s
        println!("{:p}", &s);
        println!("{}", s);
    }                           // this scope is now over, and s is no longer valid

    let x = 5;
    let y = x;
    println!("{:p}", &x);
    println!("{:p}", &y);

    let s1 = String::from("hello");
    //it's known as move s1 to s2
    //with only s2 available
    let s2 = s1;
    //println!("{:p}", &s1);
    println!("{:p}", &s2);
    /*The "String" type í allocated on the heap */
    let mut s = String::from("hello");
    println!("{:p}", &s);
    println!("{}", s);
    s.push_str(", world! ");
    /*The double colon (::) is an operator that allows us to namespace this particular from function under the String, discuss in Method Syntax*/
    println!("{}", s);

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);
    let mut x: i32;
    for mut val in &vec {
        println!("{}", val);
        x = val.clone();
        val = &(x * 2);
    }
    for val in &vec {
        println!("{}", val)
    }
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {:p}, s2 = {:p}", &s1, &s2);
    let s1 = String::from("hello");
    println!("s1 = {:p}", &s1);
    let s2 = get_string(String::from(s1));
    println!("s2 = {:p}", &s2);
    foo(&s2);
    println!("s2 = {:p}", &s2);
    let mut s2 = String::from("hello");
    println!("s2 before move = {:p}", &s2);
    mutable(&mut s2);
    println!("s2 after move = {:p}", &s2);
}

fn set_string() -> String {
    let mut s = String::from("1234");
    s.push_str("test");
    println!("{:p}", &s);
    return s;
}

fn get_string(string: String) -> String {
    let result = String::from(string);
    // gives ownership to invoker
    return result;
}

fn foo(string: &String) {
    println!("{:p}", string.as_ptr());
    bar(&string)
}

fn bar(string: &String) {
    println!("{:p}", string.as_ptr());
    baz(&string)
}

fn baz(string: &String) {
    println!("{:p}", string.as_ptr())
}

//mutable reference
fn mutable(string: &mut String) {
    println!("begin move : {:p}", string.as_ptr());
    string.push_str("1234")
}
