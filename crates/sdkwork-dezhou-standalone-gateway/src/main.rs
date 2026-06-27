use sdkwork_dezhou_standalone_gateway::{build_router, build_table_service};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let bind_address = std::env::var("DEZHOU_API_BIND")
        .or_else(|_| std::env::var("SDKWORK_DEZHOU_APPLICATION_PUBLIC_INGRESS_BIND"))
        .unwrap_or_else(|_| "127.0.0.1:8096".to_owned());

    let table_service = build_table_service()
        .await
        .expect("dezhou table service bootstrap failed");
    let app = build_router(table_service).await;
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("bind dezhou standalone-gateway listener failed");
    tracing::info!("sdkwork-dezhou-standalone-gateway listening on {bind_address}");
    axum::serve(listener, app)
        .await
        .expect("serve dezhou standalone-gateway failed");
}
