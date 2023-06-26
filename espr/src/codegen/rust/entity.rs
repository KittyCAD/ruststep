use crate::ir::*;

use check_keyword::CheckKeyword;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;
use syn::parse_quote;

// Each component of Rust struct corresponding to ENTITY in EXPRESS
struct Field {
    name: syn::Ident,
    ty: syn::Type,
    attributes: Vec<syn::Attribute>,
}

/// Check if the field should use place holder
///
/// A field will not use place holder iff its type is one of:
///
/// - a simple type
/// - an enumeration
/// - a set or list whose base type use place holder
///
fn use_place_holder(ty: &TypeRef) -> bool {
    match ty {
        TypeRef::SimpleType(..) => false,
        TypeRef::Named { is_enumerate, .. } => !*is_enumerate,
        TypeRef::Set { base, .. } | TypeRef::List { base, .. } => use_place_holder(base),
        _ => true,
    }
}

impl From<EntityAttribute> for Field {
    fn from(attr: EntityAttribute) -> Self {
        let Variable {
            name,
            ty,
            optional,
            derived,
        } = attr.into_variable().unwrap();

        let name = format_ident!("{}", name.into_safe());
        let attributes = if derived {
            vec![parse_quote! { #[holder(derived)] }]
        } else if use_place_holder(&ty) {
            vec![parse_quote! { #[holder(use_place_holder)] }]
        } else {
            Vec::new()
        };
        let ty = if optional {
            parse_quote! { Option<#ty> }
        } else if derived {
            parse_quote! { Derived<#ty> }
        } else {
            parse_quote! { #ty }
        };

        Field {
            name,
            ty,
            attributes,
        }
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Field {
            name,
            ty,
            attributes,
        } = self;

        tokens.append_all(quote! {
            #( #attributes )*
            pub #name : #ty
        });
    }
}

// Additional functions to use in codegen/rust for ir::Entity.
impl Entity {
    fn name_ident(&self) -> syn::Ident {
        format_ident!("{}", self.name.to_pascal_case())
    }

    fn any_ident(&self) -> syn::Ident {
        // `Any` indentifier must be appears if the entity is supertype
        assert!(!self.constraints.is_empty());
        format_ident!("{}Any", self.name.to_pascal_case())
    }

    /// Field identifier
    fn field_ident(&self) -> syn::Ident {
        format_ident!("{}", self.name.as_str().into_safe())
    }

    /// Generate declaration of `XxxAny` enum
    fn generate_any_enum(&self, tokens: &mut TokenStream) {
        let any = self.any_ident();

        let mut fields = vec![format_ident!("{}", self.name.as_str().into_safe())];
        let mut variants = vec![format_ident!("{}", self.name.to_pascal_case())];
        let mut constraints = vec![format_ident!("{}", self.name.to_pascal_case())];

        for ty in &self.constraints {
            match ty {
                TypeRef::Entity {
                    name, is_supertype, ..
                } => {
                    fields.push(format_ident!("{}", name.as_str().into_safe()));
                    variants.push(format_ident!("{}", name.to_pascal_case()));
                    if *is_supertype {
                        constraints.push(format_ident!("{}Any", name.to_pascal_case()));
                    } else {
                        constraints.push(format_ident!("{}", name.to_pascal_case()));
                    }
                }
                _ => unreachable!(),
            }
        }

        tokens.append_all(quote! {
            #[derive(Debug, Clone, PartialEq, Holder)]
            #[holder(table = Tables)]
            #[holder(generate_deserialize)]
            pub enum #any {
                #(
                #[holder(use_place_holder)]
                #variants(Box<#constraints>)
                ),*
            }
        }); // tokens.append_all
    }

    fn derives(&self) -> Vec<syn::Path> {
        vec![
            syn::parse_str("Debug").unwrap(),
            syn::parse_str("Clone").unwrap(),
            syn::parse_str("PartialEq").unwrap(),
            syn::parse_str("::derive_new::new").unwrap(),
            syn::parse_str("Holder").unwrap(),
        ]
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name_ident();
        let field_name = self.field_ident();

        // Each component of struct is called "field" in Rust,
        // and "attribute" refers other items
        //
        // https://doc.rust-lang.org/std/keyword.struct.html
        let fields = self
            .attributes
            .iter()
            .filter(|attr| attr.is_variable())
            .map(|attr| Field::from(attr.clone()))
            .collect::<Vec<Field>>();

        let derive = self.derives();

        tokens.append_all(quote! {
            #( #[derive(#derive)] )*
            #[holder(table = Tables)]
            #[holder(field = #field_name)]
            #[holder(generate_deserialize)]
            pub struct #name {
                #(#fields,)*
            }
        });

        // Generate `Any` enum if this entity is a supertype of other entities
        if !self.constraints.is_empty() {
            self.generate_any_enum(tokens);
        }
    }
}
