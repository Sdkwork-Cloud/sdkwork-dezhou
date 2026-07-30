use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use sdkwork_api_dezhou_assembly::assemble_api_router_with_service;
use sdkwork_dezhou_table_repository_sqlx::{
    DezhouTableRepositoryKind, InMemoryDezhouTableRepository,
};
use sdkwork_dezhou_table_service::DezhouTableService;
use sdkwork_web_bootstrap::{
    AlwaysReady, ComposedApiAssembly, DefaultWebRequestContextResolver, WebFrameworkBuilder,
};
use tower::ServiceExt;

fn test_service() -> Arc<DezhouTableService<DezhouTableRepositoryKind>> {
    Arc::new(DezhouTableService::new(DezhouTableRepositoryKind::Memory(
        InMemoryDezhouTableRepository::with_seed(Vec::new()),
    )))
}

#[tokio::test]
async fn full_contribution_hosts_contract_and_infrastructure_routes() {
    let contribution = assemble_api_router_with_service(test_service(), Arc::new(AlwaysReady))
        .await
        .expect("test contribution");
    let hosted = ComposedApiAssembly::try_compose("SDKWork Dezhou API", vec![contribution])
        .expect("test composition")
        .into_hosted(WebFrameworkBuilder::new(
            DefaultWebRequestContextResolver::default(),
        ));

    for path in ["/healthz", "/livez", "/readyz", "/metrics", "/openapi.json"] {
        let response = hosted
            .router
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{path}");
    }

    let response = hosted
        .router
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/tables")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
