use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ValueObject)]
pub fn derive_valueobject(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl ::shared_kernel::domain::IValueObject for #name {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl ::shared_kernel::domain::IEntity for #name {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(AggregateRoot)]
pub fn derive_aggregate_root(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.clone().ident;

    if let Data::Struct(data) = input.data {
        if let Fields::Named(_) = data.fields {
            let expanded = quote! {
                impl ::shared_kernel::domain::IEntity for #name {}
                impl ::shared_kernel::domain::IAggregateRoot for #name {}
            };

            return TokenStream::from(expanded);
        }
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive `AggregateRoot`",
        )
        .to_compile_error(),
    )
}

#[proc_macro_derive(DomainEvent)]
pub fn derive_domain_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.clone().ident;

    let event_type = name.clone().to_string();

    if let Data::Struct(data) = input.data {
        if let Fields::Named(_) = data.fields {
            let expanded = quote! {
                impl ::shared_kernel::domain::IDomainEvent for #name {
                    fn version(&self) -> i64 {
                        self.version
                    }

                    fn event_type(&self) -> &'static str {
                        #event_type
                    }

                    fn aggregate_id(&self) -> String {
                        self.aggregate_id.to_string()
                    }

                    fn occurred_on(&self) -> ::shared_kernel::domain::date::Date {
                        self.occurred_on.clone()
                    }

                    fn as_any(&self) -> &dyn std::any::Any {
                        self
                    }
                }
            };

            return TokenStream::from(expanded);
        }
    } else if let Data::Enum(data_enum) = input.data {
        let match_event_type = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(event) => event.event_type(),
            }
        });

        let match_aggregate_id = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(event) => event.aggregate_id(),
            }
        });

        let match_occurred_on = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(event) => event.occurred_on(),
            }
        });

        let match_version = data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #name::#variant_name(event) => event.version(),
            }
        });

        let expanded = quote! {
            impl ::shared_kernel::domain::IDomainEvent for #name {
                fn version(&self) -> i64 {
                    match self {
                        #(#match_version)*
                    }
                }

                fn event_type(&self) -> &'static str {
                    match self {
                        #(#match_event_type)*
                    }
                }

                fn aggregate_id(&self) -> String {
                    match self {
                        #(#match_aggregate_id)*
                    }
                }

                fn occurred_on(&self) -> ::shared_kernel::domain::date::Date {
                    match self {
                        #(#match_occurred_on)*
                    }
                }

                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }
            }
        };

        return TokenStream::from(expanded);
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive `AggregateRoot`",
        )
        .to_compile_error(),
    )
}

#[proc_macro_derive(Command)]
pub fn derive_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl ::shared_kernel::application::commands::ICommand for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Query)]
pub fn derive_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl ::shared_kernel::application::queries::IQuery for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    TokenStream::from(expanded)
}
