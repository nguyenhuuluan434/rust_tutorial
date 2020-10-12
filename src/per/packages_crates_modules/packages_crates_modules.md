When we write large programs, you need import and keep track.  

As a project grows, you can organize code by splitting it into **multiple modules** and then **multiple files**.

A package can contain **multiple binary** crates and optionally one **library crate**.

As a package grows, you can extract parts into separate crates that become **external dependencies**.

   - **Packages**: A Cargo feature that lets you build, test, and share crates
   - **Crates**: A tree of modules that produces a library or executable
   - **Modules** and **use**: Let you control the organization, scope, and privacy of paths
   - **Paths**: A way of naming an item, such as a struct, function, or module

Code Organization
   - Functions
   - Modules
    Can be mapped to a
        - Inline module
        - File
        - Directory hierarchy
   - Crates:
    Can be mapped to a
        - lib.rs file on the same executable crate
        - Dependency crate specified on Cargo.toml

   - Dependency can be specified from
        - path
        - git repository
        - crates.io
   - Workspaces:
    Helps to manage multiple crates as a single project

This API is used for a query a merge media task with a special id. 
