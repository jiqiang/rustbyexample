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