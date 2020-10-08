fn main() {
    let s1 = String::from("hello");
    println!("{:p}", &s1);
    let len = calculate_length(&s1);
    println!("{:p}", &s1);
    println!("The length of '{}' is {}.", s1, len);
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{:p}, {:p},", r1, r2);
    let r3 = &mut s; // no problem
    println!("{:p}", r3);
}

fn calculate_length(s: &String) -> usize {
    println!("{:p}", s);
    s.len()
}
