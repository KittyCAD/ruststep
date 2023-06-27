use super::{namespace::*, scope::*, type_ref::*, *};
use crate::ast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// Name of entity in snake_case
    pub name: String,
    pub attributes: Vec<EntityAttribute>,

    /// List of constraints corresponding to `SUBTYPE_CONSTRAINTS`
    /// and `SUPERTYPE OF` declaration in EXPRESS schema
    pub constraints: Vec<TypeRef>,

    /// List of types to be inherited by this entity
    ///
    /// When this entity is `sub` defined like:
    ///
    /// ```text
    /// ENTITY sub SUBTYPE OF (base);
    /// END_ENTITY;
    /// ```
    ///
    /// then this `supertypes` is `[base]`.
    ///
    pub supertypes: Vec<TypeRef>,

    /// List of attributes whose values are derived.
    pub derived_attributes: Vec<String>,
}

impl Entity {
    pub fn expand(&self, entities: &[Entity]) -> Entity {
        let name = self.name.clone();
        let constraints = self.constraints.clone();
        let supertypes = self.supertypes.clone();
        let derived_attributes = self.derived_attributes.clone();
        let mut output = vec![];

        fn has_variable(variable: &Variable, others: &[EntityAttribute]) -> bool {
            for attr in others.iter().filter_map(EntityAttribute::as_variable) {
                if attr.name == variable.name {
                    return true;
                }
            }
            false
        }

        fn recurse(
            root: &Entity,
            entity: &Entity,
            entities: &[Entity],
            output: &mut Vec<EntityAttribute>,
        ) {
            if !entity.supertypes.is_empty() {
                for supertype in &entity.supertypes {
                    match supertype {
                        TypeRef::Entity { name, .. } => {
                            let super_entity = entities
                                .iter()
                                .find(|&e| &e.name == name)
                                .expect("supertype not found");
                            recurse(root, super_entity, entities, output);
                        }
                        _ => eprintln!("unhandled case"),
                    }
                }
            }
            for variable in entity
                .attributes
                .iter()
                .cloned()
                .filter_map(|x| x.into_variable())
            {
                if !has_variable(&variable, output) {
                    let mut final_variable = variable.clone();
                    for derived_attribute in &root.derived_attributes {
                        if *derived_attribute == variable.name {
                            final_variable.derived = true;
                        }
                    }
                    if root as *const _ != entity as *const _ {
                        final_variable.supertype = Some(entity.name.clone());
                    }
                    output.push(EntityAttribute::Variable(final_variable));
                }
            }
        }

        recurse(self, self, entities, &mut output);

        Entity {
            name,
            attributes: output,
            constraints,
            supertypes,
            derived_attributes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub ty: TypeRef,
    pub supertype: Option<String>,
    pub optional: bool,
    // HACK: used by codegen and nowhere else.
    pub derived: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Derivation {
    /// Like `\point`
    group: String,
    /// Like `.x`
    attribute: String,
    /// For [redeclared_attribute]
    rename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityAttribute {
    Variable(Variable),
    Derivation(Derivation),
}

impl EntityAttribute {
    pub fn is_variable(&self) -> bool {
        self.as_variable().is_some()
    }

    pub fn as_variable(&self) -> Option<&Variable> {
        match self {
            EntityAttribute::Variable(ref variable) => Some(variable),
            _ => None,
        }
    }

    pub fn into_variable(self) -> Option<Variable> {
        match self {
            EntityAttribute::Variable(variable) => Some(variable),
            _ => None,
        }
    }
}

impl Legalize for EntityAttribute {
    type Input = ast::EntityAttribute;

    fn legalize(
        ns: &Namespace,
        ss: &Constraints,
        scope: &Scope,
        attr: &Self::Input,
    ) -> Result<Self, SemanticError> {
        let ty = TypeRef::legalize(ns, ss, scope, &attr.ty)?;
        Ok(match attr.name.clone() {
            ast::AttributeDecl::Reference(name) => {
                EntityAttribute::Variable(Variable {
                    name,
                    ty,
                    supertype: None, // updated later during expansion
                    optional: attr.optional,
                    derived: false, // updated later during expansion
                })
            }
            ast::AttributeDecl::Qualified {
                group,
                attribute,
                rename,
            } => EntityAttribute::Derivation(Derivation {
                group,
                attribute,
                rename,
            }),
        })
    }
}

impl Legalize for Entity {
    type Input = ast::Entity;

    fn legalize(
        ns: &Namespace,
        ss: &Constraints,
        scope: &Scope,
        entity: &ast::Entity,
    ) -> Result<Self, SemanticError> {
        let name = entity.name.clone();

        let attributes = entity
            .attributes
            .iter()
            .map(|attr| EntityAttribute::legalize(ns, ss, scope, attr))
            .collect::<Result<Vec<_>, _>>()?;

        let mut derived_attributes = Vec::new();
        if let Some(derive_clause) = entity.derive_clause.as_ref() {
            for item in &derive_clause.attributes {
                match &item.attr {
                    ast::AttributeDecl::Qualified {
                        group: _,
                        attribute,
                        rename: _,
                    } => {
                        derived_attributes.push(attribute.clone());
                    }
                    _ => {}
                }
            }
        }

        // HACK: named_unit has a single field "dimensions" which is derived by
        // one of its many subtypes. It is difficult (but not impossible) to
        // handle this edge case.
        match name.as_str() {
            "named_unit"
            | "plane_angle_unit"
            | "length_unit"
            | "solid_angle_unit"
            | "conversion_based_unit" => {
                derived_attributes.push("dimensions".to_owned());
            }
            _ => {}
        }

        let supertypes = if let Some(supertypes) = &entity.subtype_of {
            supertypes
                .entity_references
                .iter()
                .map(|sup| TypeRef::from_path(ns, ss, &ns.resolve(scope, sup)?.0))
                .collect::<Result<Vec<TypeRef>, _>>()?
        } else {
            Vec::new()
        };

        let path = Path::entity(scope, &entity.name);
        let constraints = if let Some(instantiables) = ss.instantiables.get(&path) {
            instantiables
                .iter()
                .filter_map(|pce| match pce.len() {
                    // FIXME ignore complex entity case
                    1 => Some(TypeRef::from_path(ns, ss, &pce[0])),
                    _ => None,
                })
                .collect::<Result<Vec<TypeRef>, SemanticError>>()?
        } else {
            Vec::new()
        };

        Ok(Entity {
            name,
            attributes,
            constraints,
            supertypes,
            derived_attributes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legalize() {
        let example = SyntaxTree::example();
        let ns = Namespace::new(&example);
        let ss = Constraints::new(&ns, &example).unwrap();
        dbg!(&ns);
        let entity = &example.schemas[0].entities[0];
        let scope = Scope::root().pushed(ScopeType::Schema, &example.schemas[0].name);
        let entity = Entity::legalize(&ns, &ss, &scope, entity).unwrap();
        dbg!(&entity);
    }
}
