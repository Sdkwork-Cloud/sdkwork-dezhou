//! Gateway assembly for sdkwork-dezhou.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM: preserve application-specific IAM and service-host exports.

mod bootstrap;
mod generated;
mod web_bootstrap;

pub mod route_manifest {
    include!(concat!(env!("OUT_DIR"), "/dezhou_http_routes.rs"));
}

pub use bootstrap::{
    assemble_api_router, assemble_api_router_with_service, assemble_business_routes, ApiAssembly,
};
pub use sdkwork_dezhou_service_host::{build_table_service, SharedTableService};
pub use web_bootstrap::{
    dezhou_public_path_prefixes, with_dezhou_app_request_context,
    with_dezhou_backend_request_context,
};

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
