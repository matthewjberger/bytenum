use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(Bytenum)]
pub fn derive_bytenum(input: TokenStream) -> TokenStream {
    // See https://doc.servo.org/syn/derive/struct.DeriveInput.html
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // get enum name
    let ref name = input.ident;
    let ref data = input.data;

    let mut match_arms;

    match data {
        Data::Enum(data_enum) => {
            match_arms = TokenStream2::new();

            for (index, variant) in data_enum.variants.iter().enumerate() {
                // Check if enum has more variants than a u8 can represent
                let index = {
                    let result = u8::try_from(index);
                    if result.is_ok() {
                        result.unwrap()
                    } else {
                        return derive_error!(
                            "Bytenum can only support enums with a maximum of 256 variants"
                        );
                    }
                };

                let ref variant_name = variant.ident;

                // Variant can only be a named Unit like `Variant`
                if !matches!(&variant.fields, Fields::Unit) {
                    return derive_error!("Bytenum is only implemented for named unit enum fields");
                }

                match_arms.extend(quote_spanned! {
                    variant.span()=>
                        #index => #name::#variant_name,
                });
            }
        }
        _ => return derive_error!("Bytenum is only implemented for enums"),
    };

    let expanded = quote! {
        impl TryFrom<u8> for #name {
            type Error = &'static str;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                let variant = match value {
                    #match_arms
                    _ => return Err("Failed to convert enum to u8!")
                };
                Ok(variant)
            }
        }
    };

    TokenStream::from(expanded)
}
