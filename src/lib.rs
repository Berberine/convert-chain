//! # `convert-chain`
//! [![Crates.io](https://img.shields.io/crates/v/convert-chain)](https://crates.io/crates/convert-chain)
//! [![docs.rs](https://img.shields.io/docsrs/convert-chain)](https://docs.rs/convert-chain)
//! ![Crates.io](https://img.shields.io/crates/d/convert-chain)
//! ![Crates.io](https://img.shields.io/crates/l/convert-chain)
//!
//! A tiny crate for chain type converter. There is only one macro `convert_chian` in it
//!
//! Assuming there are 3 sturct with following relationship.
//! ```rust
//! struct A;
//! struct B;
//! struct C;
//!
//! impl From<B> for A { ... }
//! impl From<C> for B { ... }
//! ```
//!
//! You can use the macro to directly convert an expression or create a closure for the convert.
//!
//! + Directly convert
//!     ```rust
//!     let c = C;
//!     let a = convert_chain!(c; B, A);
//!     ```
//!
//! + Create a closure
//!     ```rust
//!     let c = C;
//!     let f = convert_chain!(B, A);
//!     let a = f(c);
//!     ```
//!
//! Both of the above methods are equivalent to
//! ```rust
//! let c = C;
//! let a = A::from(A::from(c));
//! ```

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{discouraged::Speculative, Parse},
    parse_macro_input, Expr, Token, Type,
};

struct ConvertChain {
    expr: Option<Expr>,
    tys: Vec<Type>,
}

impl Parse for ConvertChain {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut tys = Vec::new();
        let expr = {
            let fork = input.fork();
            let expr = fork.parse::<Expr>().ok();
            expr.and_then(|expr| {
                if fork.parse::<Token![;]>().is_ok() {
                    input.advance_to(&fork);
                    Some(expr)
                } else {
                    None
                }
            })
        };
        while {
            tys.push(input.parse()?);
            !input.is_empty()
        } {
            input.parse::<Token![,]>()?;
        }
        Ok(Self { expr, tys })
    }
}

impl ToTokens for ConvertChain {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.expr.is_none() {
            tokens.append_all(quote! {
                |input|
            })
        }
        let var = self.expr.as_ref().map_or(quote!(input), |expr| expr.to_token_stream());
        tokens.append_all(self.tys.iter().fold(var, |acc, ty| {
            quote! {
                <#ty as ::core::convert::From<_>>::from(#acc)
            }
        }));
    }
}

#[proc_macro]
pub fn convert_chain(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let convert_chain = parse_macro_input!(input as ConvertChain);
    convert_chain.to_token_stream().into()
}
