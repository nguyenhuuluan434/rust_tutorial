 We can bring a path into a scope once and then call the items in that path as 
 if they’re local items with the _**use**_ keyword.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {}

```
By using pub use, external code can now call the add_to_waitlist function using hosting::add_to_waitlist.
 If we hadn’t specified pub use, the eat_at_restaurant function could call hosting::add_to_waitlist in 
 its scope, but external code couldn’t take advantage of this new path.


Adding _**use**_ and a path in a scope is similar to creating a symbolic link in the filesystem or
alias in linux cli

```rust

#![allow(unused)]
fn main() {
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}
}
```
by different scope will use different level to isolate the scope, Rust need distinguish the two type.


_**Providing New Names with the as Keyword**_

```rust

#![allow(unused)]
fn main() {
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
}
```
we can specify **_as_** and a new local name, or alias, for the type.


**Re-exporting Names with pub _use_**

_**Using External Packages**_
Cargo.toml
```toml
[dependencies]
rand = "0.5.5"
```

_**Grouping dependencies**_

```rust
use rand::Rng;
// --snip--
use std::{cmp::Ordering, io};
// --snip--

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```
from
```rust
use std::io;
use std::io::Write;
```
to
```rust

#![allow(unused)]
fn main() {
use std::io::{self, Write};
}
```

import all sub module 
```rust

#![allow(unused)]
fn main() {
use std::collections::*;
}
```
