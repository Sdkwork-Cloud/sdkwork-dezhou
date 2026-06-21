//! SQLx-backed dezhou table repository.

mod kind;
mod memory;
mod sqlx;

pub use kind::DezhouTableRepositoryKind;
pub use memory::InMemoryDezhouTableRepository;
pub use sqlx::SqlxDezhouTableRepository;
