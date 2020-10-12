Two forms to find a time in a module tree:
   - An absolute path starts from a crate root by using a crate name or a literal crate.
   - A relative path starts from the current module and uses self, super, or an identifier in the current module.

Both of those forms use double colons (::) to identifier separated.

In the absolute path, we start with _**crate**_, the root of our crate’s module tree

Then the _**front_of_house**_ module is defined in the crate root.

Adding _**pub**_ keyword before the definition of what we want to publish module/function.

_**Starting Relative Paths with super**_

We can also construct relative paths that begin in the parent module by using _**super**_ at the start of the path

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

fn main() {}

```

_**Making Structs and Enums Public**_

```rust

#![allow(unused)]
fn main() {
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
}
```

public enums
```rust

#![allow(unused)]
fn main() {
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
}

```

