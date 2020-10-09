To define a struct, we enter the keyword struct and name the entire struct. 
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

sử dụng 
```rust
 let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```

The struct is own all of its data
