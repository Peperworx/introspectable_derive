use proc_macro::TokenStream;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;


#[proc_macro_derive(Introspectable)]
pub fn derive_introspectable(input: TokenStream) -> TokenStream {

    // Parse the input structure or enum
    let input = parse_macro_input!(input as syn::Item);

    // If it is either a struct or an enum, parse it, else panic
    if let syn::Item::Struct(item) = input {

        // Get the name of the struct as both an &Ident and as a string
        let name = &item.ident;
        let name_string = name.to_string();

        // Generate an array of fields and their types.

        // First get item.fields as an iterator
        let fields = item.fields.iter();

        // Map over each field, returning a tuple of (identifier, type)
        let fields = fields.map(|f| {
            (&f.ident, f.ty.clone())
        });

        // Now enumerate fields and map over it. If the identifier is None (such as in a tuple-like struct), then return its index.
        // Return in form of a TokenStream containing a key value pair
        let fields = fields.enumerate().map(| (i, (ident, ty)) | {
            let ident = match ident {
                Some(v) => v.to_string(),
                None => i.to_string()
            };

            quote! {
                (
                    #ident,
                    <#ty>::introspect()
                )
            }
        });

        // Create and return a token stream implementing Introspectable
        TokenStream::from(quote!{
            impl introspectable::Introspectable for #name {
                fn introspect() -> introspectable::info::TypeInfo {
                    introspectable::info::TypeInfo::Compound(
                        introspectable::info::CompoundType::Struct {
                            name: #name_string,
                            fields: std::collections::HashMap::from([#(#fields),*])
                        }
                    )
                }
            }
        })

    } else if let syn::Item::Enum(item) = input {

        // Get the name of the enum as both an &Ident and as a string
        let name = &item.ident;
        let name_string = name.to_string();

        // Generate an iterator of variants
        let variants = item.variants.iter();

        // Map over each variant to determine their key value pairs for the hashmap
        let variants = variants.map(|variant| {

            // Get the variant's identifier
            let ident = variant.ident.to_string();

            // Match on the type of the fields, returning the respective enum variant types
            match &variant.fields {
                syn::Fields::Named(fields) => {
                    // Map over each fields, and get a name, value pair for each
                    let fields = fields.named.iter().enumerate().map(|(i,field)| {
                        // If name is None (it should never be) then replace with index
                        let name = match &field.ident {
                            Some(v) => v.to_string(),
                            None => i.to_string(),
                        };

                        // Get the type value
                        let ty = &field.ty;

                        quote! {
                            (#name, <#ty>::introspect())
                        }
                    });

                    // Create and return a NamedVariant type
                    quote! {
                        (#ident, introspectable::info::EnumVarian::NamedVariant {
                                fields: std::collections::HashMap::from([#(#fields),*])
                            }
                        )
                    }
                },
                syn::Fields::Unnamed(fields) => {

                    // Create an iterator to map over fields, returning the introspected type
                    let fields = fields.unnamed.iter().map(|field| {
                        let ty = &field.ty;
                        quote!{
                            #ty::introspect()
                        }
                    });

                    // Return an Unnamed Variant Type
                    quote! {
                        (#ident, introspectable::info::EnumVarian::UnamedVariant {
                            fields: vec![#(#fields),*]
                        })
                    }
                },
                syn::Fields::Unit => quote! {
                    (#ident, introspectable::info::EnumVariant::UnitVariant)
                },
            }
        });
        
        TokenStream::from(quote! {
            impl introspectable::Introspectable for #name {
                fn introspect() -> introspectable::info::TypeInfo {
                    introspectable::info::TypeInfo::Compound(
                        introspectable::info::CompoundType::Enum {
                            name: #name_string,
                            variants: std::collections::HashMap::from([#(#variants),*]),
                        }
                    )
                }
            }
        })
    } else {
        panic!("Intospectable can currently only be derived on Structs and Enums");
    }
}



