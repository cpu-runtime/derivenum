use {
    crate::{
        convert_case::{Case, Casing},
        proc_macro::TokenStream,
        proc_macro2::TokenStream as TokenStream2,
        quote::quote,
        syn::{
            parse::{Parse, ParseBuffer},
            spanned::Spanned,
            Data, DeriveInput, Error as SynError, Fields, FieldsUnnamed, Ident,
        },
    },
    std::str::FromStr,
};

pub struct EnumTake(pub TokenStream);

impl Parse for EnumTake {
    fn parse(input: &ParseBuffer) -> Result<Self, SynError> {
        let input: DeriveInput = input.parse::<DeriveInput>()?;
        if let Data::Enum(my_enum) = input.data {
            let enum_name = input.ident;
            let enum_name_string = enum_name.to_string();
            let mut ts = TokenStream2::new();
            'iter_fields: for variant in my_enum.variants {
                for attribute in &variant.attrs {
                    if let Some(ident) = attribute.path.get_ident() {
                        if ident.to_string().eq("enum_take") {
                            if let Ok(tokens) = attribute.parse_args::<Ident>() {
                                if tokens.to_string().eq("ignore") {
                                    break 'iter_fields;
                                } else {
                                    return Err(SynError::new(
                                        attribute.span(),
                                        format!(
                                            "Unexpected argument `{}` to attribute `enum_take`",
                                            tokens
                                        ),
                                    ));
                                }
                            } else {
                                return Err(SynError::new(
                                    attribute.span(),
                                    "Cannot parse arguments to attribute `enum_take` as identifier",
                                ));
                            }
                        }
                    }
                }
                let variant_name = variant.ident.clone();
                let variant_name_string = variant_name.to_string();
                let variant_snake = variant_name_string.to_case(Case::Snake);
                let variant_snake_ident = Ident::new(&variant_snake, variant_name.span());
                let take_function_string = format!("take_{}", variant_snake);
                let take_function_ident = Ident::new(&take_function_string, variant_name.span());

                if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = variant.fields {
                    if unnamed.len() > 1 {
                        let mut tuple = TokenStream2::new();
                        for i in 0..unnamed.len() {
                            let ident_string = format!("field_{}", i);
                            tuple.extend(TokenStream2::from_str(&ident_string));
                            if i < unnamed.len() - 1 {
                                tuple.extend(quote!(,));
                            }
                        }
                        let tuple_with_braces = quote! {
                            (#tuple)
                        };
                        let doc = format!("Takes values out of enum value [`{0}`]({1}::{0}) as tuple.\n\n# Panics\n\nThis function panics if the wrong value if encountered.", variant_name_string, enum_name_string);
                        ts.extend(quote! {
                            #[doc = #doc ]
                            pub fn #take_function_ident(self) -> (#unnamed) {
                                if let #enum_name::#variant_name(#tuple) = self {
                                    return #tuple_with_braces;
                                } else {
                                    panic!("Tried to take values out of {} as tuple, but value was not {}", #enum_name_string, #variant_name_string);
                                }
                            }
                        });
                    } else if unnamed.len() == 1 {
                        let doc = format!("Takes value out of enum value [`{0}`]({1}::{0}).\n\n# Panics\n\nThis function panics if the wrong value if encountered.", variant_name_string, enum_name_string);
                        ts.extend(quote! {
                            #[doc = #doc ]
                            pub fn #take_function_ident(self) -> #unnamed {
                                if let #enum_name::#variant_name(#variant_snake_ident) = self {
                                    return #variant_snake_ident;
                                } else {
                                    panic!("Tried to take value out of {}, but value was not {}", #enum_name_string, #variant_name_string);
                                }
                            }
                        });
                    } else {
                        return Err(SynError::new(unnamed.span(), "encountered empty unnamed enum value, please add `#[enum_take(ignore)]` above that variant."));
                    }
                }
            }
            let ts = quote! {
                impl #enum_name {
                    #ts
                }
            }
            .into();
            return Ok(EnumTake(ts));
        } else {
            return Err(SynError::new(
                input.span(),
                "Cannot derive `EnumTake` on anything that is not an enum",
            ));
        }
    }
}
