**match**  is an extremely powerful control flow operator. This operator is allow to compare 
a value against a series of patterns then execute code based on which pattern matches.
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
The pattern can be  literal values, variable names, wildcards, ...

follow the match is **pattern** and block code. There are many value in block code, if the 
pattern matched with the value the block code after the **=>** will be executed.

Another useful feature of match arms (value in match) is that they can bind to the parts of the values that match the pattern.
 This is how we can extract values out of enum variants.

default value
```rust
_ => {} //compile error if match's the left hand is require
```
rust is not accept null value.

**Matching with Option<T>**

In rust code, the patter **match** against an enum are used a lot,
 bind a variable to the data inside, and then execute code based on it. 

**Matches Are Exhaustive**

If we didn't handle the **variant** in the enum or there is no any the default value Rust will compile error.

**The _ Placeholder**

The _ pattern will match any value. Putting it after all the value as default value.

**Type of pattern matching**
   -   Literals
   -   Destructured arrays, enums, structs, or tuples
   -   Variables
   -   Wildcards
   -   Placeholders

If the pattern is a variable it will match anything => use "const" for pattern 
