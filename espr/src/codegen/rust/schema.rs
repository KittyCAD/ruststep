use crate::ir::*;

use check_keyword::CheckKeyword;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CratePrefix {
    Internal,
    External,
}

impl CratePrefix {
    pub fn as_path(&self) -> syn::Path {
        match self {
            CratePrefix::Internal => syn::parse_str("crate").unwrap(),
            CratePrefix::External => syn::parse_str("::ruststep").unwrap(),
        }
    }
}

impl IR {
    pub fn to_token_stream(&self, prefix: CratePrefix) -> TokenStream {
        let schemas: Vec<_> = self
            .schemas
            .iter()
            .map(|schema| schema.to_token_stream(prefix))
            .collect();
        quote! { #(#schemas)* }
    }
}

impl Entity {
    fn expand(&self, entities: &[Entity]) -> Entity {
        let name = self.name.clone();
        let constraints = self.constraints.clone();
        let supertypes = self.supertypes.clone();
        let mut attributes = vec![];

        fn recurse(entity: &Entity, entities: &[Entity], attributes: &mut Vec<EntityAttribute>) {
            if !entity.supertypes.is_empty() {
                for supertype in &entity.supertypes {
                    match supertype {
                        TypeRef::Entity { name, .. } => {
                            let super_entity = entities
                                .iter()
                                .find(|&e| &e.name == name)
                                .expect("supertype not found");
                            recurse(super_entity, entities, attributes);
                        }
                        _ => eprintln!("unhandled case"),
                    }
                }
            }
            for attribute in &entity.attributes {
                if !attributes.contains(attribute) {
                    attributes.push(attribute.clone());
                }
            }
        }

        recurse(self, entities, &mut attributes);

        Entity {
            name,
            attributes,
            constraints,
            supertypes,
        }
    }
}

impl Schema {
    pub fn to_token_stream(&self, prefix: CratePrefix) -> TokenStream {
        let name = format_ident!("{}", self.name);
        let types = &self.types;
        let entities = &self.entities;
        let type_decls = self.types.iter().filter(|e| match e {
            TypeDecl::Enumeration(_) => false,
            _ => true,
        });

        let expanded_entities: Vec<Entity> = entities.iter().map(|e| e.expand(entities)).collect();

        let entity_types: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}", e.name.to_pascal_case()))
            .chain(
                type_decls
                    .clone()
                    .map(|e| format_ident!("{}", e.id().to_pascal_case())),
            )
            .collect();
        let holder_name: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}", e.name.as_str().into_safe()))
            .chain(
                type_decls
                    .clone()
                    .map(|e| format_ident!("{}", e.id().into_safe())),
            )
            .collect();
        let holders_name: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}_holders", e.name))
            .chain(type_decls.map(|e| format_ident!("{}_holders", e.id())))
            .collect();

        let ruststep_path = prefix.as_path();

        quote! {
            pub mod #name {
                use #ruststep_path::{as_holder, Holder, TableInit, primitive::*, derive_more::*};
                use std::collections::HashMap;

                #[derive(Debug, Clone, PartialEq, Default, TableInit)]
                pub struct Tables {
                    #(
                    #holder_name: HashMap<u64, as_holder!(#entity_types)>,
                    )*
                }

                impl Tables {
                    #(
                    pub fn #holders_name(&self) -> &HashMap<u64, as_holder!(#entity_types)> {
                        &self.#holder_name
                    }
                    )*
                }

                #(#types)*
                #(#expanded_entities)*
            }
        }
    }
}
