// بسم الله الرحمن الرحيم وبه نستعين

use std::io::{stdin, Error};

// "Result<T, E>" is an "enum" type with 2 variants: "Ok(T)" is success type with value "T", and "Err(E)" is error type with value "E"
fn main() -> Result<(), Error> {
    let mut n = String::new(); // Mutable variable "n" owns the data (value) "String::new()" and its allocated memory address
    stdin().read_line(&mut n)?; // Pass in a mutable ***reference "&" (points)*** to "n" into the method to ***borrow***
    let m = &n; // Assign "m" a *view (reference)* to the variable "n", otherwise ownership will ***move*** from "n" to "m", and "n" will no longer be accessible!
    print!("n: {n}, m: {m}");

    let text1 = String::from("hello");
    let text2 = &text1[0..2]; // "text2" ***borrows*** a subset of "text1"
    println!("text2: {text2}");
    let concat_text = text1 + " world"; // "concat_text" now *owns* "text1", hence; we can't call "text1" afterwards, unless we've used "text1.clone()"
    println!("{concat_text}");
    // println!("Error: {text2}"); // error because "text1" was moved out to "concat_text" but "text2" still points to "text1" which has moved out (dangling reference)
    return Ok(()); // Return "Ok" variant of type success with value "()" --> unit value (no meaningful value to return on success)
}

/*
In rust, every bit of allocated memory has exactly ONE owner only, and as soon as the owner gets deleted or is out of scope,
its memory is immediately freed, and any variables holding a reference to it will be invalid (dangling references).
*/

/*
Additionally, when you pass a variable to a function/method/loop or assign it to another variable,
the default behavior is to ***move*** the ownership of that variable into the function/method/loop or the new variable,
and as such, the old variable will no longer be accessible since its ownership of that piece of memory has been *moved*!
Moreover, any variable that used to point (reference) to that moved variable will no longer be accessible (dangling reference).
*/

/*
Thus, we had to pass a mutable ***reference (&)*** to "n" instead of "n" directly into the "read_line()" method.
Because otherwise, the ownership of "n" would have been ***moved*** to the "read_line()" method, and then we would
not be able to access the "n" variable after the method call.
*/

/*
Note: primitive datatypes: {integers, floats, booleans, tuples, arrays, and characters} all implement the "Copy" trait;
meaning they can be passed as arguments directly without using "&", and their data will automatically be copied rather than moved!
Hence, the original variables will still be accessible (valid) after passing them around.
(because it's a relatively cheap operation to copy these primitive datatypes)
*/

/*
*** BIG QUESTION ***
Q.) Should I *move* the ownership of a variable, *copy* a variable, or pass a *reference* to a variable?

Ans:
    When dealing with simple datatypes (e.g., small "structs", "enums", or "vecs"), moving their ownership by passing them directly (pass by value)
    is more efficient, because transferring their ownership is cheap and doesn't involve any copying of the data, especially
    if the function/method needs to mutate the value, and access to the variable is ***unneeded*** afterwards.

    When dealing with primitive datatypes that implement the "Copy" trait, you should pass their variables by value (it wont be moved), because
    copying their data is faster than passing a reference since they're trivially copyable, and there is no need to manage their ownership or
    borrow them by passing them as references using the "&".

    When dealing with large or complex datatypes (e.g., hashmaps), it's more efficient to pass a *reference* "&"; which essentially is a
    view of the original value, because it avoids the cost of copying (cloning) the data.
*/

/*
Note: using a reference "&" essentially points to an address in memory, and it's automatically dereferenced, i.e., you directly access the
data stored in that memory address, and when used in an x64 CPU architecture, it's size is 64 bits (8 bytes)!
 */

/*
Note: you CAN'T have a mutable reference and an immutable reference to a variable at the same time!
Because one function/method can change the mutable reference to the variable while the other function/method expects it to remain the same.
 */
