//! Generate Rust code using proc-macro utility crates

mod entity;
mod format;
mod schema;
mod simple_type;
mod type_decl;
mod type_ref;

pub use entity::*;
pub use format::rustfmt;
pub use schema::*;

// HACK: fix special cases properly.
pub fn make_name(ident: &str) -> String {
    use inflector::Inflector;
    let mut name = ident.to_screaming_snake_case();
    match name.as_str() {
        "AXIS_1_PLACEMENT" => name = "AXIS1_PLACEMENT".to_string(),
        "AXIS_2_PLACEMENT_2D" => name = "AXIS2_PLACEMENT_2D".to_string(),
        "AXIS_2_PLACEMENT_3D" => name = "AXIS2_PLACEMENT_3D".to_string(),
        _ => {}
    };
    name
}
