use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

use crate::ir::*;

impl ToTokens for TypeDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeDecl::Simple(simple) => simple.to_tokens(tokens),
            TypeDecl::Rename(rename) => rename.to_tokens(tokens),
            TypeDecl::Enumeration(e) => e.to_tokens(tokens),
            TypeDecl::Select(select) => select.to_tokens(tokens),
        }
    }
}

impl ToTokens for Simple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_name = format_ident!("{}", &self.id.to_snake_case());
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let ty = &self.ty;
        let (derive, _) = simple_meta(&field_name);
        tokens.append_all(quote! {
            #derive
        #[holder(from = #ty)]
            pub struct #id(pub #ty);
        });
    }
}

impl ToTokens for Rename {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_name = format_ident!("{}", &self.id.to_snake_case());
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let ty = &self.ty;
        let (derive, use_place_holder) = match ty {
            TypeRef::SimpleType(_) => simple_meta(&field_name),
            TypeRef::Named { is_enumerate, .. } => {
                // Enumeration does not have Holder.
                if *is_enumerate {
                    simple_meta(&field_name)
                } else {
                    rename_meta(&field_name)
                }
            }
            _ => rename_meta(&field_name),
        };

        tokens.append_all(quote! {
            #derive
            pub struct #id(#use_place_holder pub #ty);
        });
    }
}

fn simple_meta(field_name: &syn::Ident) -> (TokenStream, TokenStream) {
    (
        quote! {
            #[derive(Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, ::ruststep_derive::Holder)]
            #[holder(table = Tables)]
            #[holder(field = #field_name)]
            #[holder(generate_deserialize)]
        },
        quote! {},
    )
}

fn rename_meta(field_name: &syn::Ident) -> (TokenStream, TokenStream) {
    (
        quote! {
            #[derive(Clone, Debug, PartialEq, AsRef, Deref, DerefMut, Into, From, ::ruststep_derive::Holder)]
            #[holder(table = Tables)]
            #[holder(field = #field_name)]
            #[holder(generate_deserialize)]
        },
        quote! {#[holder(use_place_holder)]},
    )
}

impl ToTokens for Enumeration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let items: Vec<_> = self
            .items
            .iter()
            .map(|i| format_ident!("{}", i.to_pascal_case()))
            .collect();
        tokens.append_all(quote! {
            #[derive(Debug, Clone, PartialEq, ::ruststep_derive::ToData, ::serde::Deserialize)]
            pub enum #id {
                #( #items ),*
            }
        });
    }
}

impl ToTokens for Select {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let mut entries = Vec::new();
        let mut entry_types = Vec::new();
        let mut use_place_holder = Vec::new();
        for ty in &self.types {
            match ty {
                TypeRef::Entity {
                    name, is_supertype, ..
                } => {
                    entries.push(format_ident!("{}", name.to_pascal_case()));
                    if *is_supertype {
                        entry_types.push(quote! { #ty });
                    } else {
                        entry_types.push(quote! { Box<#ty> });
                    }
                    use_place_holder.push(quote! { #[holder(use_place_holder)] });
                }
                TypeRef::Named {
                    name, is_enumerate, ..
                } => {
                    entries.push(format_ident!("{}", name.to_pascal_case()));
                    if *is_enumerate {
                        entry_types.push(quote! { #ty });
                        use_place_holder.push(quote! {});
                    } else {
                        entry_types.push(quote! { Box<#ty> });
                        use_place_holder.push(quote! { #[holder(use_place_holder)] });
                    }
                }
                _ => unimplemented!(),
            }
        }
        tokens.append_all(quote! {
            #[derive(Debug, Clone, PartialEq, Holder)]
            #[holder(table = Tables)]
            #[holder(generate_deserialize)]
            pub enum #id {
                #(
                #use_place_holder
                #entries(#entry_types)
                ),*
            }
        });
    }
}
