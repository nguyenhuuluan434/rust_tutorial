Programming language with Garbage collection (GC)  that look for no longer used memory.

Programming language with explicitly allocate and free memory. 

Rust manage memory through ownership with set of rule what will be 
checked by the compiler at compile time. 

**The Stack and the Heap**

Both the stack and the heap are parts of memory that are available to your code to use at runtime,
 but they are structured in different ways.

   - The stack stores values in the order it gets them and removes the values in the opposite order.
    This is referred to as last in, first out. Adding data is called pushing onto the stack,
     and removing data is called popping off the stack.
   All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
     
   -  The heap is less organized: when you put data on the heap, you request a certain amount of space.
    The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. 
    This process is called allocating on the heap and is sometimes abbreviated as just allocating. 
     Because the pointer is a known, fixed size, you can store the pointer on the stack,
      but when you want the actual data, you must follow the pointer.
    
Pushing to the stack is faster than allocating on the heap because the allocator never has to search
 for a place to store new data; that location is always at the top of the stack.
  Comparatively, allocating space on the heap requires more work,
   because the allocator must first find a big enough space to hold the data and then perform
    bookkeeping to prepare for the next allocation.

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

**Ownership Rules** (important)
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

**Variable Scope**

   - When a variable comes into scope, it is valid.
   - It remains valid until it goes out of scope.


String object vs String literal
   - String Literal: is hardcoded directly into the final executable,
    string literal, it usually shows up as a reference, 
    meaning that we use a pointer to a string stored in permanent memory
    and it’s guaranteed to be valid for the duration of the entire program
   - String object: memory must be requested from the memory allocator at runtime


Types that have a “copy trait,” (i.e. types whose data can be stored on the stack),
the ownership model behaves similarly to other languages that may use a different paradigm,
like garbage collection. But for types without this trait,
 we needed to be more conscious of the ownership rules.


**Giving Ownership**
Just as ownership is taken by calling another function and passing in a variable,
 a function can be given ownership via a return from a different function:

```rust
fn main {
    let value = get_string();
}
fn get_string() ->String{
    let result = String::from("hello");
    // gives ownership to invoker
    return result;
}
```
the result variable in get_string function have same memory address with value when the get_string return to main,
but the ownership will be changed for get_string function to main function

Returning values can also transfer ownership

**Give & Take**


**References & Borrowing**

Rust has a feature what is called references. The references will take care ownership.

Borrowing: main() function give string to foo() function but main() still owner of the string
,the string will not be dropped from memory 
```rust
fn main(){
    let string= String::from("hello");
    println!("{}",string.as_ptr());
    foo(&string);
    println!("{}",string.as_ptr())
}
fn foo(string:&String){
    println!("{}",string.as_ptr());
}
```
main() passes a reference of string to foo() , and foo() accept a String type reference
The **&** indicate the reference. 

Nếu 1 function có tham số nhận vào không phải là reference thì nó function này sẽ là owner của tham số đó.
**value borrowed here after move**


