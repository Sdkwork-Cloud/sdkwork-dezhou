use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/tables",
        "dezhou",
        "dezhou.table.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/tables/{tableId}",
        "dezhou",
        "dezhou.table.retrieve",
    ),
];

pub fn gateway_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
