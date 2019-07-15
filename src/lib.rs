// Copyright 2015-2018 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # enum-as-inner
//!
//! A deriving proc-macro for generating functions to automatically give access to the inner members of enum.
//!
//! ## Basic unnamed field case
//!
//! The basic case is meant for single item enums, like:
//!
//! ```rust
//! # #[macro_use] extern crate enum_as_inner;
//! # fn main() {
//!
//! #[derive(EnumAsInner)]
//! enum OneEnum {
//!     One(u32),
//! }
//!
//! let one = OneEnum::One(1);
//!
//! assert_eq!(*one.as_one().unwrap(), 1);
//! # }
//! ```
//!
//! the result is always a reference for inner items.
//!
//! ## Unit case
//!
//! This will return copy's of the value of the unit variant, as `isize`:
//!
//! ```rust
//! # #[macro_use] extern crate enum_as_inner;
//! # fn main() {
//!
//! #[derive(EnumAsInner)]
//! enum UnitVariants {
//!     Zero,
//!     One,
//!     Two,
//! }
//!
//! let unit = UnitVariants::Two;
//!
//! assert_eq!(unit.as_two().unwrap(), ());
//! # }
//! ```
//!
//! ## Mutliple, unnamed field case
//!
//! This will return a tuple of the inner types:
//!
//! ```rust
//! # #[macro_use] extern crate enum_as_inner;
//! # fn main() {
//!
//! #[derive(EnumAsInner)]
//! enum ManyVariants {
//!     One(u32),
//!     Two(u32, i32),
//!     Three(bool, u32, i64),
//! }
//!
//! let many = ManyVariants::Three(true, 1, 2);
//!
//! assert_eq!(many.as_three().unwrap(), (&true, &1_u32, &2_i64));
//! # }
//! ```
//!
//! ## Multiple, named field case
//!
//! This will return a tuple of the inner types, like the unnamed option:
//!
//! ```rust
//! # #[macro_use] extern crate enum_as_inner;
//! # fn main() {
//!
//! #[derive(EnumAsInner)]
//! enum ManyVariants {
//!     One{ one: u32 },
//!     Two{ one: u32, two: i32 },
//!     Three{ one: bool, two: u32, three: i64 },
//! }
//!
//! let many = ManyVariants::Three{ one: true, two: 1, three: 2 };
//!
//! assert_eq!(many.as_three().unwrap(), (&true, &1_u32, &2_i64));
//! # }
//! ```

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use heck::SnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use syn::DeriveInput;

/// returns first the types to return, the match names, and then tokens to the field accesses
fn unit_fields_return(
    name: &syn::Ident,
    variant_name: &syn::Ident,
    function_name: &Ident,
    doc: &str,
) -> TokenStream {
    quote!(
        #[doc = #doc ]
        pub fn #function_name(&self) -> Option<()> {
            match self {
                #name::#variant_name => {
                    Some(())
                }
                _ => None
            }
        }
    )
}

/// returns first the types to return, the match names, and then tokens to the field accesses
fn unnamed_fields_return(
    name: &syn::Ident,
    variant_name: &syn::Ident,
    function_name: &Ident,
    doc: &str,
    fields: &syn::FieldsUnnamed,
) -> TokenStream {
    let (returns, matches, accesses) = match fields.unnamed.len() {
        1 => {
            let field = fields.unnamed.first().expect("no fields on type");
            let field = field.value();

            let returns = &field.ty;
            let returns = quote!(&#returns);
            let matches = quote!(inner);
            let accesses = quote!(&inner);

            (returns, matches, accesses)
        }
        0 => (quote!(()), quote!(), quote!(())),
        _ => {
            let mut returns = TokenStream::new();
            let mut matches = TokenStream::new();
            let mut accesses = TokenStream::new();

            for (i, field) in fields.unnamed.iter().enumerate() {
                let rt = &field.ty;
                let match_name = Ident::new(&format!("match_{}", i), Span::call_site());
                returns.extend(quote!(&#rt,));
                matches.extend(quote!(#match_name,));
                accesses.extend(quote!(&#match_name,));
            }

            (quote!((#returns)), quote!(#matches), quote!((#accesses)))
        }
    };

    quote!(
        #[doc = #doc ]
        pub fn #function_name(&self) -> Option<#returns> {
            match self {
                #name::#variant_name(#matches) => {
                    Some(#accesses)
                }
                _ => None
            }
        }
    )
}

/// returns first the types to return, the match names, and then tokens to the field accesses
fn named_fields_return(
    name: &syn::Ident,
    variant_name: &syn::Ident,
    function_name: &Ident,
    doc: &str,
    fields: &syn::FieldsNamed,
) -> TokenStream {
    let (returns, matches, accesses) = match fields.named.len() {
        1 => {
            let field = fields.named.first().expect("no fields on type");
            let field = field.value();
            let match_name = field.ident.as_ref().expect("expected a named field");

            let returns = &field.ty;
            let returns = quote!(&#returns);
            let matches = quote!(#match_name);
            let accesses = quote!(&#match_name);

            (returns, matches, accesses)
        }
        0 => (quote!(()), quote!(), quote!(())),
        _ => {
            let mut returns = TokenStream::new();
            let mut matches = TokenStream::new();
            let mut accesses = TokenStream::new();

            for field in fields.named.iter() {
                let rt = &field.ty;
                let match_name = field.ident.as_ref().expect("expected a named field");

                returns.extend(quote!(&#rt,));
                matches.extend(quote!(#match_name,));
                accesses.extend(quote!(&#match_name,));
            }

            (quote!((#returns)), quote!(#matches), quote!((#accesses)))
        }
    };

    quote!(
        #[doc = #doc ]
        pub fn #function_name(&self) -> Option<#returns> {
            match self {
                #name::#variant_name{ #matches } => {
                    Some(#accesses)
                }
                _ => None
            }
        }
    )
}

fn impl_all_as_fns(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let enum_data = if let syn::Data::Enum(data) = &ast.data {
        data
    } else {
        panic!("{} is not an enum", name);
    };

    let mut stream = TokenStream::new();

    for variant_data in &enum_data.variants {
        let variant_name = &variant_data.ident;
        let function_name = Ident::new(
            &format!("as_{}", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc = format!(
        "Optionally returns references to the inner fields if this is a `{}::{}`, otherwise `None`",
        name, variant_name
        );

        let tokens = match &variant_data.fields {
            syn::Fields::Unit => unit_fields_return(name, variant_name, &function_name, &doc),
            syn::Fields::Unnamed(unnamed) => {
                unnamed_fields_return(name, variant_name, &function_name, &doc, &unnamed)
            }
            syn::Fields::Named(named) => {
                named_fields_return(name, variant_name, &function_name, &doc, &named)
            }
        };

        stream.extend(tokens);
    }

    quote!(
        impl #name {
            #stream
        }
    )
}

#[proc_macro_derive(EnumAsInner)]
pub fn enum_as_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // get a usable token stream
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let expanded: TokenStream = impl_all_as_fns(&ast);

    // Return the generated impl
    proc_macro::TokenStream::from(expanded)
}
