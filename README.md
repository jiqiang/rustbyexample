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
