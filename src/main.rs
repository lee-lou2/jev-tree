use std::env;
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            env::var("RUST_LOG").unwrap_or_else(|_| "jev_tree=info,tower_http=info".into()),
        )
        .init();
    let port = env::var("PORT").unwrap_or_else(|_| "8768".into());
    let db = env::var("JEV_TREE_DB").unwrap_or_else(|_| "data/demo.db".into());
    let seed = env::var("JEV_TREE_SEED").unwrap_or_else(|_| "data/seed.json".into());
    let static_dir = env::var("JEV_TREE_STATIC_DIR").unwrap_or_else(|_| "static".into());
    if !std::path::Path::new(&static_dir)
        .join("index.html")
        .exists()
    {
        return Err(format!(
            "JEV_TREE_STATIC_DIR={static_dir} has no index.html; run from the repo root or \
             point it at the bundled `static` directory"
        )
        .into());
    }
    let state = jev_tree::build_state(&db, &seed)?;
    let bind = env::var("JEV_TREE_BIND").unwrap_or_else(|_| "127.0.0.1".into());
    if let Some(reason) =
        jev_tree::refuse_open_public_bind(&bind, state.store.has_server_key().unwrap_or(false))
    {
        return Err(reason.into());
    }
    let app = jev_tree::http::router(state.clone(), static_dir)
        .layer(CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(format!("{bind}:{port}")).await?;
    info!(%bind, %port, evaluator = state.jev.label(), "jev-tree listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    // SIGTERM too: `docker stop` and Kubernetes send it, and an unhandled one
    // would cut in-flight runs instead of draining the listener.
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
