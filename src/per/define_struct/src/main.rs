fn main() {
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user);
    user.email = String::from("nguyenhuuluan434@gmail.com");
    println!("{:?}", user);

    let user = build_user(String::from("luannh@lsa.cn"), String::from("luannh"));
    println!("{:?}", user);

    //spread copy rest of properties of ..var
    let user = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user
    };
    println!("{:?}", user);

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_rectangle(&rect1)
    );
    println!("rect1 is {:#?}", rect1);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
