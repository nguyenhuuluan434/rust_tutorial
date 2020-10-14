So far, all the examples in this chapter defined multiple modules in one file.
When modules get large, you might want to move their definitions to a separate
 file to make the code easier to navigate.

This is called crate root file (lib.rs or main.rs)
Using a _**semicolon**_ (;) after mod _**front_of_house**_ rather than using a block tells 
Rust to load the contents of the module from another file with _**the same name as the module**_. 

src/lib.rs
```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

src/front_of_house.rs
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

Dòng đầu tiên trong file lib.rs hoặc main.rs là **mod module_name**,
Tạo 1 file với tên là **module_name.rs**, Để tạo sub module cho **module_name**
thì tạo một thư mục có tên là **module_name**, trong thư mục mới này tạo 
file mới ứng với sub module muốn tạo **module_name/sub_module_name.rs**.
Để có thể expose submodule thì bên trong file **module_name.rs** phải định nghĩa
**pub mod sub_module_name**

Xem ví dụ **restaurant_v2** để clean 
