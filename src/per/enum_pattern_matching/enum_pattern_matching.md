**Enumerations** referred to enums.

Enums allow you to define a type enumerating its **possible** variants

The match expression makes it easy to run different code for different values of an enum

_if let construct_ is convenient and concise idiom available to you to handle enums in your code.

Note that the variants of the enum are namespaced under its identifier,
 and we use a **double colon** to separate the two.
```rust
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:#?}", four);
    println!("{:#?}", six);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
```
define enum with data
```rust
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}
```
You can put any kind of data inside an enum variant: strings, numeric types, or structs,
You can even include another enum

We can define methods on structs using impl,

**The Option Enum and Its Advantages Over Null Values**
Rust doesn’t have the null feature => 
Option type is build-in, the standard library of Rust
```rust
enum Option<T> {
    Some(T),
    None,
}
```
**Option<T>** have variants what we can used **Some** and **None**, you can use directly without **Option::**

We can proceed confidently without having to check for null before using that value.
Only when we have an Option<i8> (or whatever type of value we’re working with)
do we have to worry about possibly not having a value, and the compiler will make 
sure we handle that case before using the value.

In other words, you have to convert an Option<T> to a T before you can perform T operations with it.
 Generally, this helps catch one of the most common issues with null: assuming 
 that something isn’t null when it actually is.
 
Not having to worry about incorrectly assuming a not-null value helps you to be more confident in your code
. In order to have a value that can possibly be null, you must explicitly opt in by making the type of 
that value Option<T>. Then, when you use that value, you are required to explicitly handle the case
 when the value is null. Everywhere that a value has a type that isn’t an Option<T>,
  you can safely assume that the value isn’t null. This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.
