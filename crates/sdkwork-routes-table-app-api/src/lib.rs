mod error;
mod paths;
mod routes;

pub use error::{map_dezhou_error, ok_envelope};
pub use paths::{TABLES_LIST_PATH, TABLE_DETAIL_PATH};
pub use routes::{build_table_app_router, respond_list, DezhouListQuery};
