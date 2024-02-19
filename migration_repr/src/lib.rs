use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Ty {
    Struct(BTreeMap<String, Box<Ty>>),
    U32,
    Bool,
}

impl Ty {
    pub fn rename(&self, name: &str, new_name: &str) -> Self {
        match self {
            Ty::Struct(fields) => todo!(),
            _ => panic!("Can only rename structs"),
        }
    }
}

// e.g. foo.bar.baz.0
type Path = Vec<String>;

pub enum Commands {
    /// Renames a struct field to a new name
    Rename(Path, String),
    // Wraps a type in Option<_>
    MakeOptional(Path),
    // Adds a field to a struct
    AddField(Path, String),
}

#[derive(Debug, Clone)]
pub enum Value {
    Struct(BTreeMap<String, Box<Value>>),
    U32(u32),
    Bool(bool),
}

pub trait Migratable {
    fn get_ty() -> Ty;
}

impl Migratable for u32 {
    fn get_ty() -> Ty {
        Ty::U32
    }
}

impl Migratable for bool {
    fn get_ty() -> Ty {
        Ty::Bool
    }
}

pub struct Migration {}

impl Migration {
    pub fn new(_ty: Ty) -> Self {
        Self {}
    }
    pub fn to_token_stream(&self) -> TokenStream {
        quote!(
            fn hello_world() -> &'static str {
              "hello world"
             }
        )
    }
}
