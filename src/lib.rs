//! A tiny crate for chain type converter. There is only one macro `convert-chian` in it. You can use it to directly
//! convert an expression or create a closure for the convert.
//!
//! type relation
//! ```rust
//! struct A;
//! struct B;
//! struct C;
//!
//! impl From<B> for A { ... }
//! impl From<C> for B { ... }
//! ```
//!
//! directly convert:
//! ```rust
//! let c = C;
//! let a = convert_chain!(c; B, A);
//! ```
//!
//! create a closure:
//! ```rust
//! let c = C;
//! let f = convert_chain!(B, A);
//! let a = f(c);
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
                #ty::from(#acc)
            }
        }));
    }
}

#[proc_macro]
pub fn convert_chain(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let convert_chain = parse_macro_input!(input as ConvertChain);
    convert_chain.to_token_stream().into()
}
