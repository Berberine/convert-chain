# `convert-chain`
[![Crates.io](https://img.shields.io/crates/v/convert-chain)](https://crates.io/crates/convert-chain)
[![docs.rs](https://img.shields.io/docsrs/convert-chain)](https://docs.rs/convert-chain)
![Crates.io](https://img.shields.io/crates/d/convert-chain)
![Crates.io](https://img.shields.io/crates/l/convert-chain)

A tiny crate for chain type converter. There is only one macro `convert_chian` in it

Assuming there are 3 sturct with following relationship.
```rust
struct A;
struct B;
struct C;

impl From<B> for A { ... }
impl From<C> for B { ... }
```

You can use the macro to directly convert an expression or create a closure for the convert.

+ Directly convert
    ```rust
    let c = C;
    let a = convert_chain!(c; B, A);
    ```

+ Create a closure
    ```rust
    let c = C;
    let f = convert_chain!(B, A);
    let a = f(c);
    ```

Both of the above methods are equivalent to
```rust
let c = C;
let a = A::from(B::from(c));
```