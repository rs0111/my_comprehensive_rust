# Day 1: Afternoon

## Memo

### Tuples and Arrays

- Tuples and Arrays
    - Tuples and Arrays are the `compound` types
    - All elements of an array have the same type
    - Tuples can accommodate different types
    - Both types have a size fixed at compile time

- Array Iteration
    - The `for` statement supports iterating arrays (but not tuples)

- Pattern Matching
    - The `match` keyword lets you match a value against one or more *patterns*
    - The comparisons are done from top to bottom and the first match wins
    - The `_` pattern is a wildcard pattern which matches any value
        - `_` is often used as the final catch-all case
    - each match arm must have the same type

- Destructuring
    - Destructuring is way of extracting data from a data structure
    - You can destructure tuples and arrays by matching on their elements

- Exercise: Nested Arrays
    - Arrays can contain other arrays

### References

- Shared References
    - A reference provides a way to access another value without responsibility for the value, and is also called "borrowing"
    - Shared references are read-only, and the referenced data cannot change
    - A shared reference to a type `T` has type `&T`
    - A reference value is made with the `&` operator
    - The `*` operator "dereference" a reference, yielding its value

- Exclusive References
    - Exclusive references, also known as mutable references, allow changing the value they refer to
    - They have type `&mut T`

- Exercise: Geometry
    - Use the `sqrt()` method to calculate the square root

### User-Defined types

- Named Structs
