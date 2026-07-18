pub mod bootstrap;

pub use bootstrap::{build_router, build_router_from_business, build_table_service};
pub use sdkwork_dezhou_gateway_assembly::route_manifest::{
    DEZHOU_APP_HTTP_ROUTES, DEZHOU_BACKEND_HTTP_ROUTES,
};
pub use sdkwork_dezhou_gateway_assembly::{
    dezhou_public_path_prefixes, with_dezhou_app_request_context,
    with_dezhou_backend_request_context,
};
