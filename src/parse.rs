
use quote::ToTokens;
use syn::Type;



/// Parses a type, returning a TokenStream of its evaluation function
pub fn parse_type(ty: Type) -> proc_macro2::TokenStream {
    
    // Match on various types
    match ty {
        Type::Array(array) => {

            let aty = array.elem;
            let len = array.len;

            quote! {
                introspectable::info::TypeInfo::Compound(
                    introspectable::info::CompoundType::Array{
                        type_info: Box::new(#aty::introspect()),
                        length: #len
                    }
                )
            }
        },
        Type::Paren(internal) => {
            parse_type(*internal.elem)
        },
        Type::Never(_) => {
            quote! {
                introspectable::info::TypeInfo::Never
            }
        },
        Type::Path(path) => {
            quote! {
                #path::introspect()
            }
        },
        Type::Ptr(ptr) => {
            quote! {
                #ptr::introspect()
            }
        },
        Type::ImplTrait(imp) => {

            let types = imp.bounds.iter().map(|v| v.into_token_stream().to_string()).collect::<Vec<String>>();

            quote! {
                introspectable::info::TypeInfo::Impl(Vec::new(#(#types),*))
            }
        },
        Type::TraitObject(imp) => {

            let types = imp.bounds.iter().map(|v| v.into_token_stream().to_string()).collect::<Vec<String>>();

            quote! {
                introspectable::info::TypeInfo::Dyn(Vec::new(#(#types),*))
            }
        },
        Type::Reference(reference) => {

            let is_mutable = reference.mutability.is_some();

            let lifetime = reference.lifetime.to_token_stream().to_string();

            let ty = reference.elem;

            quote! {
                introspectable::info::TypeInfo::Pointer(
                    introspectable::info::PointerType::Reference {
                        lifetime: #lifetime,
                        type_info: Box::new(#ty::introspect()),
                        mutable: #is_mutable,
                    }
                )
            }
        },
        Type::Slice(slice) => {
            let ty = slice.elem;
            quote! {
                introspectable::info::TypeInfo::Compound(
                    introspectable::info::CompoundType::Slice{
                        type_info: Box::new(#ty::introspect()),
                    }
                )
            }
        },
        Type::Tuple(tuple) => {
            let types = tuple.elems.iter().collect::<Vec<_>>();

            quote! {
                introspectable::info::TypeInfo::Compound(
                    introspectable::info::CompoundType::Tuple{
                        fields: vec![#(#types::introspect()),*],
                    }
                )
            }
        },
        _ => todo!(),
        
    }
}