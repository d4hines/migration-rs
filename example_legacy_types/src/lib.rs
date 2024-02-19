use migration_macro::Migration_repr;

// We define the initial types in a separate
// crate and derive data structures to reprent
// them. In a seperate crate, we'll use these
// derived structures to generate new types and
// migrations between them.

#[derive(Migration_repr)]
pub struct LegacyFoo {
    some_field: bool,
}

#[derive(Migration_repr)]
pub struct LegacyBar {
    pub some_other_field: u32,
    pub foo: LegacyFoo,
}

