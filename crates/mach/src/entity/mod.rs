//! SeaORM entities for Mach.
//!
//! Keep module paths stable so `db.get_schema_registry("machich::entity::*")`
//! can discover everything automatically.

pub mod config;
pub mod todo;

/// Convenience exports for downstream modules.
pub mod prelude {
    pub use super::config;
    pub use super::todo;
}
