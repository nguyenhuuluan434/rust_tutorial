fn main() {
    let mut s = String::from("hello worl");
    let x = first_word(&s);
    println!("{}", x);
    //s.clear();
    let s = "luannh";
    println!("{}", x);
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item != b' ' {
            continue;
        }
        return &s[0..i];
    }
    &s[..]
}
