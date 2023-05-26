# `convert-chain`
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
    let a = A;
    let C = convert_chain!(c; B, C);
    ```

+ Create a closure
    ```rust
    let a = A;
    let f = convert_chain!(B, C);
    let c = f(C);
    ```

Both of the above methods are equivalent to
```rust
let a = A;
let c = C::from(B::from(a));
```