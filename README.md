# Rust Programming Language

## 8. Common Collections

Unlike the built-in array and tuple types, the data these collections point to is stored on the **heap**, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

#### 8.1 Vectors

```rust
let v: Vec<i32> = Vec::new();
```

Note that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.

```rust
let v = vec![1, 2, 3];
```

Because we’ve given initial `i32` values, Rust can infer that the type of `v` is `Vec<i32>`, and the type annotation isn’t necessary.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12)
];
```

The variants of an enum are defined under the same enum type, so when we need to store elements of a different type in a vector, we can define and use an enum.

#### 8.2 Strings

A `String` is a wrapper of `Vec<u8>`.

#### 8.3 Hash Maps

Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

# 9. Error Handling

Rust groups errors into two major categories: *recoverable* and *unrecoverable* errors. For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

#### 9.1 Unrecoverable Errors with panic!

 The key to reading the backtrace is to start from the top and read until you see files you wrote. That’s the spot where the problem originated. The lines above the lines mentioning your files are code that your code called; the lines below are code that called your code.

 #### 9.2 Recoverable Errors with Result

 *Match guard* is an extra condition on a match arm that further refines the arm’s pattern. This condition must be true for that arm’s code to be run; otherwise, the pattern matching will move on to consider the next arm in the `match`.

 The reason you use `ref` to create a reference in a pattern instead of `&` will be covered in detail in Chapter 18. In short, in the context of a pattern, `&` matches a reference and gives you its value, but `ref` matches a value and gives you a reference to it.

 Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier.

 Error values taken by `?` go through the `from` function, defined in the `From` trait in the standard library, which is used to convert errors from one type into another. When `?` calls the `from` function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.

 The `?` operator can only be used in functions that have a return type of `Result`, because it is defined to work in the same way as the match expression.

#### 9.3 To `panic!` or Not to `panic!`

 It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. 

 # 10. Generic Types, Traits, and Lifetimes

 #### 10.2 Traits: Defining Shared Behavior

 One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to our crate.

 #### 10.3 Validating References with Lifetimes

 ##### Lifetime Elision Rules:
 1. Each parameter that is a reference gets its own lifetime parameter.
 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
 3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters.

 Where we declare and use the lifetime parameters depends on whether they're related to the struct fields or the method parameters and return values.
 