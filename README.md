# `convert-chain`
A tiny crate for chain type converter. There is only one macro `convert-chian` in it. You can use it to directly convert an expression or create a closure for the convert.

```rust
struct A;
struct B;
struct C;

impl From<B> for A { ... }
impl From<C> for B { ... }
```

## Directly convert
```rust
let c = C;
let a = convert_chain!(c; B, A);
```

## Create a closure
```rust
let c = C;
let f = convert_chain!(B, A);
let a = f(c);
```