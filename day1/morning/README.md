# Day 1:Morning

## Memo

### Hello, World

- The main function is the entry point of the program like C language.
- Rust strings are UTF-8 encoded and contain any Unicode character.

### Benefit of Rust 

- Compile time memory safety (no NULL pointers!)
- No undefined runtime error behavior
- Zero-cost abstractions like C++

### Types and Values

- Variables
    - Type safety via static typing
    - Variable bindings are made with `let`

- Values
    - `iN`, `uN`, and `fN` are N bits wide
    - `isize` and `usize` are the width of a pointer
    - `char` is 32 bits wide
    - `bool` is 8 bits wide

- Arithmetic
    - Arithmetic in Rust is similar to other languages

- Strings
    - Rust has two types to represent strings
        - `String` modifiable, owned string
        - `&str` a read-only Strings
        - String literals have `&str` type
        - `&str = &'static str`
        - `&str` does not own the string slice

- Type Inference
    - Rust will look at how the variable is used to determine the type

- Exercise: Fibonacci
