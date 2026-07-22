use axum::body::Body;
use axum::http::{Request, StatusCode};
use sdkwork_api_dezhou_standalone_gateway::{
    build_router, build_table_service, with_dezhou_app_request_context,
};
use sdkwork_dezhou_table_repository_sqlx::{
    DezhouTableRepositoryKind, InMemoryDezhouTableRepository,
};
use sdkwork_dezhou_table_service::DezhouTableService;
use sdkwork_routes_table_app_api::build_table_app_router;
use sdkwork_web_core::{access_token_jwt, auth_token_jwt};
use std::sync::Arc;
use tower::ServiceExt;

static DEV_AUTH_ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn dev_tokens() -> (String, String) {
    (
        format!(
            "Bearer {}",
            auth_token_jwt("100001", "user-1", "session-1", "dezhou")
        ),
        access_token_jwt("100001", "user-1", "session-1", "dezhou"),
    )
}

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
    let _env_guard = DEV_AUTH_ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    std::env::set_var("SDKWORK_IAM_ALLOW_DEV_AUTH_FALLBACK", "true");
    let (auth_token, access_token) = dev_tokens();
    let router = with_dezhou_app_request_context(build_table_app_router(memory_table_service()));

    let response = router
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/tables")
                .header("Authorization", auth_token)
                .header("Access-Token", access_token)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(Some(0), payload["code"].as_i64());
    assert!(payload["traceId"].as_str().is_some());
    assert!(payload["data"]["items"].is_array());
    assert_eq!(Some("offset"), payload["data"]["pageInfo"]["mode"].as_str());
    std::env::remove_var("SDKWORK_IAM_ALLOW_DEV_AUTH_FALLBACK");
}

#[tokio::test]
async fn table_router_rejects_invalid_pagination_with_problem_detail() {
    let _env_guard = DEV_AUTH_ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    std::env::set_var("SDKWORK_IAM_ALLOW_DEV_AUTH_FALLBACK", "true");
    let (auth_token, access_token) = dev_tokens();
    let router = with_dezhou_app_request_context(build_table_app_router(memory_table_service()));

    let response = router
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/tables?page_size=201")
                .header("Authorization", auth_token)
                .header("Access-Token", access_token)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        Some("application/problem+json"),
        response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok())
    );
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(Some(40_003), payload["code"].as_i64());
    assert!(payload["traceId"].as_str().is_some());
    std::env::remove_var("SDKWORK_IAM_ALLOW_DEV_AUTH_FALLBACK");
}

#[tokio::test]
async fn build_router_mounts_infrastructure_and_table_routes() {
    let app = build_router(memory_table_service()).await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
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
    std::env::remove_var("DEZHOU_REPOSITORY_MODE");
}
