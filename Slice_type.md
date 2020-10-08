**The Slice Type**

The data type what does not have ownership.
Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

Rather than a reference to the entire String, it’s a reference to a portion of the String.

slice used for reference to a variable and compiler will check to prevent edit original variable 
if the reference used after edit.

```text
3 |     let x = first_word(&s);
  |                        -- immutable borrow occurs here
4 |     println!("{}", x);
5 |     s.clear();
  |     ^^^^^^^^^ mutable borrow occurs here
6 |     let s = "luannh";
7 |     println!("{}", x);
  |                    - immutable borrow later used here
```
Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference.
 Because clear needs to truncate the String,

_String Literals Are Slices_



**Summary**

The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work, so we’ll talk about these concepts further throughout the rest of the book. Let’s move on to Chapter 5 and look at grouping pieces of data together in a struct.
