use crate::{
    convert_case::{Case, Casing},
    proc_macro::TokenStream,
    proc_macro2::TokenStream as TokenStream2,
    quote::quote,
    syn::{
        parse::{Parse, ParseBuffer},
        spanned::Spanned,
        Data, DeriveInput, Error as SynError, Fields, Ident,
    },
};

pub struct EnumMatch(pub TokenStream);

impl Parse for EnumMatch {
    fn parse(input: &ParseBuffer) -> Result<Self, SynError> {
        let input: DeriveInput = input.parse::<DeriveInput>()?;
        if let Data::Enum(my_enum) = input.data {
            let enum_name = input.ident;
            let enum_name_string = enum_name.to_string();
            let mut ts = TokenStream2::new();
            'iter_fields: for variant in my_enum.variants {
                for attribute in &variant.attrs {
                    if let Some(ident) = attribute.path.get_ident() {
                        if ident.to_string().eq("enum_match") {
                            if let Ok(tokens) = attribute.parse_args::<Ident>() {
                                if tokens.to_string().eq("ignore") {
                                    break 'iter_fields;
                                } else {
                                    return Err(SynError::new(
                                        attribute.span(),
                                        format!(
                                            "Unexpected argument `{}` to attribute `enum_match`",
                                            tokens
                                        ),
                                    ));
                                }
                            } else {
                                return Err(SynError::new(
                                    attribute.span(),
                                    "Cannot parse arguments to attribute `enum_match` as identifier",
                                ));
                            }
                        }
                    }
                }
                let variant_name = variant.ident;
                let variant_name_string = variant_name.to_string();
                let variant_snake = variant_name_string.to_case(Case::Snake);
                let am_function_string = format!("am_{}", variant_snake);
                let am_function_ident = Ident::new(&am_function_string, variant_name.span());

                match variant.fields {
                    Fields::Unnamed(_) => {
                        let doc = format!(
                            "Returns [`true`] if [`{0}`] has value of [`{1}`]({0}::{1}).",
                            enum_name_string, variant_name_string,
                        );
                        ts.extend(quote! {
                            #[doc = #doc ]
                            pub fn #am_function_ident(&self) -> bool {
                                matches!(self, #enum_name::#variant_name(_))
                            }
                        });
                    }
                    Fields::Named(_) => {
                        let doc = format!(
                            "Returns `[true`] if [`{0}`] has value of [`{1}`]({0}::{1}).",
                            enum_name_string, variant_name_string
                        );
                        ts.extend(quote! {
                            #[doc = #doc ]
                            pub fn #am_function_ident(&self) -> bool {
                                matches!(self, #enum_name::#variant_name { .. })
                            }
                        });
                    }
                    Fields::Unit => {
                        let doc = format!(
                            "Returns [`true`] if [`{0}`] has value of [`{1}`]({0}::{1}).",
                            enum_name_string, variant_name_string
                        );
                        ts.extend(quote! {
                            #[doc = #doc ]
                            pub fn #am_function_ident(&self) -> bool {
                                matches!(self, #enum_name::#variant_name)
                            }
                        });
                    }
                }
            }
            let ts = quote! {
                impl #enum_name {
                    #ts
                }
            }
            .into();
            return Ok(EnumMatch(ts));
        } else {
            return Err(SynError::new(
                input.span(),
                "Cannot derive `EnumMatch` on anything that is not an enum",
            ));
        }
    }
}
