use std::time::Instant;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn middleware_satu(req: Request, next: Next) -> Response {
    println!("MIDDLEWARE SATU");
    let response = next.run(req).await;
    response
}

pub async fn middleware_dua(req: Request, next: Next) -> Response {
    let current = Instant::now();
    let mut response = next.run(req).await;
    let elapsed = format!("Latency {:?}", current.elapsed());
    response.headers_mut().insert(
        "x-request-time", elapsed.as_str().parse().unwrap()
    );
    response
}