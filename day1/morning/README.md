# Day 1: Morning

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
    - The first and second Fibonacci numbers are both 1
    - For n>2, the n'th Fibonacci number is calculated recursively as the sum of (n-1)'th and (n-2)'th Fibonacci numbers

### Control Flow Basics

- Conditionals
    - You use `if expressions` exactly like `if` statement in other languages
    - You can use `if` as an expression
        - The last expression of each block becomes the value of the `if` expression

- Loops
    - The `loop` statement just loops forever, until a `break`

- break and continue
    - If you want to exit any kind of loop early, use `break`
    - For `loop`, this can take an optional expression that becomes the value of the `loop` expression
    - If you want to immediately start the next iteration, use `continue`
    - Both `continue` and `break` can optionally take a label argument which is used to break out of nested loops

- Blocks and Scopes
    - Blocks
        - A block in Rust contains a sequence of expressions, enclosed by braces `{}`
        - Each block has a value and a type, which are those of the last expression of the block
        - If the last expression ends with `;`, then the resulting value and type is `()`

    - Scopes and Shadowing
        - A variable's scope is limited to the enclosing block
        - You can shadow variables, both those from outer scopes and variables from the same scope

- Functions
    - Declaration parameters are followed by a type, then a return type
    - The last expression in a function body becomes the return value
        - Simply omit the `;` at the end of the expression
    - The `return` keyword can be used for early return

- Macros
    - Macros are expanded into Rust code during compilation
    - They are distinguished by a `!` at the end
    - `println!(format, ..)` prints a line to standard output
    - `format!(format, ..)` works just like `println!` but returns the result as a string
    - `dbg!(expression)` logs the value of the expression and returns it
    - `todo!()` marks a bit of code as not-yet-implemented
        - If executed, it will panic
    - `unreachable!()` marks a bit of code as unreachable
        - If executed, it will panic

- Exercise: Collatz Sequence
    - The Collatz Sequence is defined as follows, for an arbitrary $n_1$ greater than zero
        - If $n_i$ is 1, then the sequence terminates at $n_i$
        - If $n_i$ is even, then $n_{i+1} = n_i / 2$
        - If $n_i$ is odd, then $n_{i+1} = 3\times n_i + 1$
