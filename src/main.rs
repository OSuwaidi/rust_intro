// بسم الله الرحمن الرحيم وبه نستعين

/*
Cargo is similar to pip; it is a package and dependency manager
To initialize a new rust project, use $cargo new "project_name"$ --> creates a new directory, "Cargo.toml" file, a
    "src" directory, and initializes a Git repository.
Then, run "$ cargo build" in order to set up all the dependencies and create an executable file (binary) in the "target/debug" directory
*/

/* In order to use your compiled source code in production, you need to build a "release version" of your end binary file
    instead of a "debug version", and to do that, you run "$ cargo build --release" */

// To check if your code has any errors or issues, run: $cargo check$ (without compiling the code)

// To run your "main.rs" source code in terminal, you run: "$ cargo run" in the directory where the "src" folder exits

// To compile the source code into an executable binary file (".exe" for example in windows) use: "$ rustc main.rs"

// To run test rust scripts, you must store them in the "examples" folder, then run: "$ cargo run --example file_name"

// To automatically format your source code inside of your project's directory run: "$ rustfmt main.rs"
// To apply "$ rustfmt" on all ".rs" files, run: "$ rustfmt *.rs"

// To update Rust, run: "$ rustup update"

// To update all your dependencies and update the "Cargo.lock" file, "run: $ cargo update"

// A "crate" in rust is similar to a package/library in python

// The operator "::" is called the path seperator operator

fn main() {
    const MAGIC_NUM: i32 = 7;
    let x: i32 = 10; // you can either declare a variable's type using ":"
    let x = 10_i32; // or by appending "_datatype" at the end of a variable
    let x = 10; // rust also comes with *type inference* which is usually more efficient than type annotation!

    let y: u32 = 5;
    let z = 10_f32;
    let _result = z + (y as f32); // can only apply operations on same datatypes, hence we cast datatypes accordingly

    let ans = some_fun(x, y as i32);
    println!("Answer is: {}", ans);

    let arr: [i32; 3] = [1, 2, 3];

    let res: u64 = 2_u64.pow(32);
    println!("x is: {} and arr[0] is {} and res is {}", x, arr[0], res);
}

fn some_fun(x: i32, y: i32) -> i32 {
    let sum = x + y;
    if sum > 10 {
        return 0;
    }
    return sum;
}
