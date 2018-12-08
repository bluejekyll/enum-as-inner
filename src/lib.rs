// Copyright 2015-2018 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::{Ident, Span, TokenStream};
use syn::DeriveInput;

/// returns first the types to return, the match names, and then tokens to the field accesses
fn unit_fields_return(
    name: &syn::Ident,
    variant_name: &syn::Ident,
    function_name: &Ident,
) -> TokenStream {
    quote!(
        impl #name {
            pub fn #function_name(&self) -> Option<isize> {
               match self {
                   #name::#variant_name => {
                       Some(#name::#variant_name as isize)
                   }
                   _ => None
               }
            }
        }
    )
}

/// returns first the types to return, the match names, and then tokens to the field accesses
fn unnamed_fields_return(
    name: &syn::Ident,
    variant_name: &syn::Ident,
    function_name: &Ident,
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
        impl #name {
            pub fn #function_name(&self) -> Option<#returns> {
               match self {
                   #name::#variant_name(#matches) => {
                       Some(#accesses)
                   }
                   _ => None
               }
            }
        }
    )
}

/// returns first the types to return, the match names, and then tokens to the field accesses
fn named_fields_return(
    name: &syn::Ident,
    variant_name: &syn::Ident,
    function_name: &Ident,
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
        impl #name {
            pub fn #function_name(&self) -> Option<#returns> {
               match self {
                    #name::#variant_name{ #matches } => {
                        Some(#accesses)
                    }
                   _ => None
               }
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
            &format!("as_{}", variant_name).to_lowercase(),
            Span::call_site(),
        );

        let tokens = match &variant_data.fields {
            syn::Fields::Unit => unit_fields_return(name, variant_name, &function_name),
            syn::Fields::Unnamed(unnamed) => {
                unnamed_fields_return(name, variant_name, &function_name, &unnamed)
            }
            syn::Fields::Named(named) => {
                named_fields_return(name, variant_name, &function_name, &named)
            }
        };

        stream.extend(tokens);
    }

    stream
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
