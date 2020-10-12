The **use** keyword that brings a path into scope.

The **pub** keyword to make items public.

_**Modules**_ let us organize code within a crate into groups for readability and easy reuse.

Modules also control the privacy of items, which is whether an item can be used by outside code (public) 
or is an internal implementation detail and not available for outside use (private).

We define a module by starting with the **mod** keyword and then specify the name of the module.
Inside modules, we can have other modules. 

Modules can also hold definitions for other items, such as struct, enums, constants, traits

that _**src/main.rs**_ and **_src/lib.rs_** are called crate roots. 

Notice that the entire module tree is rooted under the implicit module named _**restaurant**_.
```
restaurant
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
