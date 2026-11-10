//! The connection loop behind `main`.
//!
//! `axum::serve` builds its hyper connections with the default HTTP/1 settings, and hyper's
//! default treats a client that has finished sending (read EOF / FIN) as gone: it closes the
//! connection and drops the response still being computed. A client that shuts its write side
//! right after the request is legal HTTP/1.1 — `curl` behind a forward proxy, or a sandbox that
//! relays requests, does exactly that — and it got `curl: (52) Empty reply from server` from
//! every search that took longer than the EOF needed to arrive. Fast responses sometimes won the
//! race, so it looked intermittent while a plain client and the health check stayed green.
//!
//! This loop is `axum::serve` with one change: `half_close(true)`, so the response is written
//! after the client's EOF.

use std::{future::Future, time::Duration};

use axum::{Router, body::Body};
use hyper::{Request, body::Incoming, service::service_fn};
use hyper_util::{rt::TokioIo, server::graceful::GracefulShutdown};
use tokio::net::TcpListener;
use tower_service::Service;
use tracing::{debug, warn};

/// How long in-flight requests get to finish once shutdown starts.
const DRAIN: Duration = Duration::from_secs(30);

/// Serves `app` on `listener` until `shutdown` resolves, then drains in-flight requests.
pub async fn serve(listener: TcpListener, app: Router, shutdown: impl Future<Output = ()>) {
    let graceful = GracefulShutdown::new();
    tokio::pin!(shutdown);
    loop {
        let stream = tokio::select! {
            accepted = listener.accept() => match accepted {
                Ok((stream, _peer)) => stream,
                Err(err) => {
                    // EMFILE and friends are transient; back off instead of spinning.
                    warn!(%err, "accept failed");
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    continue;
                }
            },
            () = &mut shutdown => break,
        };
        let app = app.clone();
        let service = service_fn(move |request: Request<Incoming>| {
            let mut app = app.clone();
            async move { app.call(request.map(Body::new)).await }
        });
        let connection = hyper::server::conn::http1::Builder::new()
            .half_close(true)
            .serve_connection(TokioIo::new(stream), service);
        let connection = graceful.watch(connection);
        tokio::spawn(async move {
            if let Err(err) = connection.await {
                debug!(%err, "connection ended with an error");
            }
        });
    }
    drop(listener);
    if tokio::time::timeout(DRAIN, graceful.shutdown())
        .await
        .is_err()
    {
        warn!(
            secs = DRAIN.as_secs(),
            "shutdown drain timed out; closing remaining connections"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::routing::get;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpStream,
        sync::oneshot,
    };

    /// Slow enough that the client's EOF is always read before the handler returns.
    async fn slow() -> &'static str {
        tokio::time::sleep(Duration::from_millis(80)).await;
        "slow-body"
    }

    fn app() -> Router {
        Router::new().route("/slow", get(slow))
    }

    async fn spawn_server() -> (
        std::net::SocketAddr,
        oneshot::Sender<()>,
        tokio::task::JoinHandle<()>,
    ) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (stop, stopped) = oneshot::channel::<()>();
        let task = tokio::spawn(serve(listener, app(), async move {
            let _ = stopped.await;
        }));
        (addr, stop, task)
    }

    async fn request(addr: std::net::SocketAddr, half_close: bool) -> String {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream
            .write_all(b"GET /slow HTTP/1.1\r\nHost: x\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();
        if half_close {
            stream.shutdown().await.unwrap(); // FIN right after the request
        }
        let mut reply = Vec::new();
        stream.read_to_end(&mut reply).await.unwrap();
        String::from_utf8_lossy(&reply).into_owned()
    }

    #[tokio::test]
    async fn answers_a_client_that_half_closes_right_after_the_request() {
        let (addr, _stop, _task) = spawn_server().await;
        for _ in 0..5 {
            let reply = request(addr, true).await;
            assert!(
                reply.starts_with("HTTP/1.1 200"),
                "empty or wrong reply: {reply:?}"
            );
            assert!(reply.ends_with("slow-body"), "body missing: {reply:?}");
        }
    }

    #[tokio::test]
    async fn still_answers_a_client_that_keeps_its_side_open() {
        let (addr, _stop, _task) = spawn_server().await;
        let reply = request(addr, false).await;
        assert!(
            reply.starts_with("HTTP/1.1 200") && reply.ends_with("slow-body"),
            "{reply:?}"
        );
    }

    #[tokio::test]
    async fn shutdown_lets_an_in_flight_request_finish_and_then_returns() {
        let (addr, stop, task) = spawn_server().await;
        let in_flight = tokio::spawn(request(addr, true));
        tokio::time::sleep(Duration::from_millis(20)).await; // the handler is mid-sleep
        stop.send(()).unwrap();
        let reply = in_flight.await.unwrap();
        assert!(
            reply.ends_with("slow-body"),
            "in-flight request was cut: {reply:?}"
        );
        tokio::time::timeout(Duration::from_secs(5), task)
            .await
            .expect("serve did not return after the drain")
            .unwrap();
    }

    #[tokio::test]
    async fn stops_accepting_after_shutdown() {
        let (addr, stop, task) = spawn_server().await;
        stop.send(()).unwrap();
        tokio::time::timeout(Duration::from_secs(5), task)
            .await
            .unwrap()
            .unwrap();
        assert!(
            TcpStream::connect(addr).await.is_err(),
            "listener still open after shutdown"
        );
    }
}
