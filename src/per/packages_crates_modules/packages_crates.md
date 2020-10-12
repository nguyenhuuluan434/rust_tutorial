A **crate** is binary or library.

The **crate root** is a _source file_ that the Rust compiler starts from 
and makes up the root module of your crate.

A package is one or more crates that provide a set of functionality.
A package must contain zero or one library crates, and no more.

A package contains a **Cargo.toml** file that describes how to build those crates.

```shell script
$ cargo new my-project
     Created binary (application) `my-project` package
#When we entered the command, Cargo created a Cargo.toml file, giving us a package.
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs

```
there’s no mention of **src/main.rs** because Cargo follows a convention 
that src/main.rs is the crate root of a binary crate with the same name as the package.

Likewise, Cargo knows that if the **package directory** contains src/lib.rs,
the package contains a library crate with the same name as the package,
and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.

Here, we have a package that only contains src/main.rs, meaning it only contains a binary crate named my-project

 If a package contains src/main.rs and src/lib.rs, it has two crates:
  a library and a binary, both with the same name as the package

Create new library restaurant 
```shell script
cargo new --lib restaurant
```

