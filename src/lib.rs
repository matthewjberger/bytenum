use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(bytenum), forward_attrs(allow, doc, cfg))]
struct BytenumOptions {
    repr: Option<Ident>,
}

#[proc_macro_derive(Bytenum, attributes(bytenum))]
pub fn derive_bytenum(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let options =
        BytenumOptions::from_derive_input(&input).expect("Invalid bytenum options specified.");
    let repr = options
        .repr
        .unwrap_or(Ident::new(&"u8".to_string(), Span::call_site()));

    if !["u8".to_string(), "u16".to_string(), "u32".to_string()].contains(&repr.to_string()) {
        return derive_error!("Enum representation must be either u8, u16, or u32");
    }

    let ref name = input.ident;
    let ref data = input.data;

    let mut match_arms;

    match data {
        Data::Enum(data_enum) => {
            match_arms = TokenStream2::new();

            for variant in data_enum.variants.iter() {
                let ref variant_name = variant.ident;

                // Variant can only be a named Unit like `Variant`
                if !matches!(&variant.fields, Fields::Unit) {
                    return derive_error!("Bytenum is only implemented for named unit enum fields");
                }

                match_arms.extend(quote_spanned! {
                    variant.span()=>
                        x if x == #name::#variant_name as #repr => Ok(#name::#variant_name),
                });
            }
        }
        _ => return derive_error!("Bytenum is only implemented for enums"),
    };

    let expanded = quote! {
        impl TryFrom<#repr> for #name {
            type Error = &'static str;

            fn try_from(value: #repr) -> Result<Self, Self::Error> {
                match value {
                    #match_arms
                    _ => Err("Failed to convert enum to numeric value!")
                }
            }
        }
    };

    TokenStream::from(expanded)
}
