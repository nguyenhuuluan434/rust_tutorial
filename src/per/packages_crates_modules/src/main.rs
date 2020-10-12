
fn main() {
    use std::fmt;
    use std::io;
    println!("Hello, world!");
    fn function1() -> fmt::Result {
        // --snip--
        Ok(())
    }

    fn function2() -> io::Result<()> {
        // --snip--
        Ok(())
    }
}
