pub mod bootstrap;
mod web_bootstrap;

pub use bootstrap::{build_router, build_table_service};
pub use route_manifest::{DEZHOU_APP_HTTP_ROUTES, DEZHOU_BACKEND_HTTP_ROUTES};
pub use web_bootstrap::{
    dezhou_public_path_prefixes, with_dezhou_app_request_context,
    with_dezhou_backend_request_context,
};

pub mod route_manifest {
    include!(concat!(env!("OUT_DIR"), "/dezhou_http_routes.rs"));
}
