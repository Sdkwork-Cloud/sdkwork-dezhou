//! SDKWork dezhou table service contracts.

pub mod domain;
pub mod ports;
pub mod service;

pub use domain::models::{
    DezhouError, DezhouResult, DezhouTableItem, DezhouTablePage, DezhouTableQuery,
};
pub use ports::repository::DezhouTableRepository;
pub use service::DezhouTableService;
