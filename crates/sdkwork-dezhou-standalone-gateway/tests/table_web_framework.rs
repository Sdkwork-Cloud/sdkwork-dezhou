use axum::body::Body;
use axum::http::{Request, StatusCode};
use sdkwork_dezhou_standalone_gateway::{
    build_router, build_table_service, with_dezhou_app_request_context,
};
use sdkwork_dezhou_table_repository_sqlx::{
    DezhouTableRepositoryKind, InMemoryDezhouTableRepository,
};
use sdkwork_dezhou_table_service::DezhouTableService;
use sdkwork_routes_table_app_api::build_table_app_router;
use std::sync::Arc;
use tower::ServiceExt;

const DEV_AUTH_TOKEN: &str =
    "Bearer tenant_id=demo-tenant;user_id=user-1;session_id=session-1;app_id=dezhou;auth_level=password";
const DEV_ACCESS_TOKEN: &str =
    "tenant_id=demo-tenant;user_id=user-1;session_id=session-1;app_id=dezhou;environment=dev;deployment_mode=saas";

type SharedTableService = Arc<DezhouTableService<DezhouTableRepositoryKind>>;

fn memory_table_service() -> SharedTableService {
    Arc::new(DezhouTableService::new(DezhouTableRepositoryKind::Memory(
        InMemoryDezhouTableRepository::with_seed(vec![]),
    )))
}

#[tokio::test]
async fn table_router_rejects_unauthenticated_requests() {
    let router = with_dezhou_app_request_context(build_table_app_router(memory_table_service()));

    let response = router
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

#[tokio::test]
async fn table_router_accepts_dev_inline_dual_tokens() {
    let router = with_dezhou_app_request_context(build_table_app_router(memory_table_service()));

    let response = router
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/tables")
                .header("Authorization", DEV_AUTH_TOKEN)
                .header("Access-Token", DEV_ACCESS_TOKEN)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn build_router_merges_health_and_table_routes() {
    let app = build_router(memory_table_service()).await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/system/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn memory_repository_mode_builds_table_service() {
    std::env::set_var("DEZHOU_REPOSITORY_MODE", "memory");
    let service = build_table_service().await.expect("table service");
    assert!(Arc::strong_count(&service) >= 1);
}
