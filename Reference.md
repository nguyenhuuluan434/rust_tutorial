The opposite of **referencing** by using **&** is **dereferencing**,
 which is accomplished with the dereference operator, *

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
 Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.
We don’t drop what the reference points to when it goes out of scope because we don’t have ownership

**We call having references as function parameters borrowing**

You can't change value of reference,Just as variables are immutable by default, so are references.
 We’re not allowed to modify something we have a reference to.
 => mutable reference 
```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
first the variable must be must mutable ```let mut s = String::from("hello");```

you can have only one mutable reference to a particular piece of data in a particular scope. 

you can have multiple immutable reference, because immutable is readonly

you can't have multiple mutable reference, mutable variable what can read/write => **data race** 

error **immutable borrow occurs here, mutable borrow occurs here**

**Dangling References**

A pointer that references a location in memory that may have been given to someone else,
 by freeing some memory while preserving a pointer to that memory.
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
```
error => this function's return type contains a borrowed value, but there is no value
         for it to be borrowed from.

The solution is return direct value.

**The Rules of References**

   - At any given time, you can have either one mutable reference or any number of immutable references.
   - References must always be valid.






