use crate::Coin::{Dime, Quarter};
use crate::UsState::Louisiana;

fn learn_enum() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:#?}", four);
    println!("{:#?}", six);
    let home = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:#?}", home);
    println!("{:#?}", loopback);
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:#?}", home);
    println!("{:#?}", loopback);

    let home = IpAddrFormat::V4(127, 0, 0, 1);
    let loopback = IpAddrFormat::V6(String::from("::1"));
    println!("{:#?}", home);
    println!("{:#?}", loopback);

    //function of enum define in impl block
    let message = Message::Write(String::from("luannh"));
    message.print();


    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("{:#?}", some_number);
    println!("{:#?}", some_string);
    println!("{:#?}", absent_number);
    println!("{:#?}", absent_number.is_some());
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddrKind,
    address: String,
}

impl IpAddress {
    pub fn new(kind: IpAddrKind, address: String) -> Self {
        IpAddress { kind, address }
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrFormat {
    V4(u8, u8, u8, u8),
    V6(String),
}


#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        print!("{:#?}", self);
    }
}

fn learn_match_control() {
    let x = value_in_cents(Quarter(Louisiana));
    let x = value_in_cents(Dime);
    println!("{:?}", x);

    let x = plus_one(Some(1));
    println!("{:?}", x);
    let x = plus_one(x);
    println!("{:?}", x.unwrap());
    let x = plus_one(None);
    println!("{:?}", x);
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    Louisiana,
    NewJerSey,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        /*Coin::Dime => 10,*/
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        _ => 0
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn if_let() {
    let mut count = 0;
    let coin = Coin::Quarter(Louisiana);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("Count {:?}!", count);
}
fn test_match(){
    let a = 23;
    let b = 16;
    let c = generate_c(a, b);

    for &input in &[16, 23, 42, 43] {
        match input {
            //always match with
            a => println!("Input is equal to a"),
            b => println!("Input is equal to b"),
            c => println!("Input is equal to c"),
            _ => println!("Input does not equal any value"),
        }
    }
}
fn generate_c(a: i32, b: i32) -> i32 {
    a + b + 4
}
fn main() {
    //learn_enum();
    //learn_match_control();
    //if_let();
    test_match()
}
