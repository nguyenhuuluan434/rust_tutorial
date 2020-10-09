Methods are similar to functions: they’re declared with the **fn** keyword and their name,
they can have parameters and a return value, and they contain some code that is run 
when they’re called from somewhere else. However, methods are different from functions
in that they’re defined within the context of a struct (or an enum or a trait object,
which we cover in Chapters 6 and 17, respectively), and their first parameter is always self,
which represents the instance of the struct the method is being called on.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

To define the function within the context of structure, we start an **impl** keyword before structure name.
Then create function within **impl** block 

The first parameter to be **self** in the signature.

We can instead use method syntax to call the area method on our Structure **instance**.
 The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.

We still need to use the & before self, just as we did in structure

 If we wanted to change the instance that we’ve called the method on as part of what the method does,
  we’d use **&mut self** as the first parameter.

**Associated Functions**

Define function without **&self** , those functions are called **associated functions** because they are 
associated with the struct. usage **structure_name::function_name(params)**

