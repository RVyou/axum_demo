use axum::{
    handler::Handler,
    body::{Body, Bytes},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, put},
    Router,
}; //extract::{Extension},
use tokio::signal;

use handle::controller::*;

#[tokio::main]
async fn main() {
    // let _poll = POOL.get_connet(); //lazy_static 运行到代码时才进行初始化
    let app = Router::new()
        .route(
            "/users/:user_id",
            get(user::UserController::users_teams_show),
        )
        // .route("/users/list", get(user::UserController::users_teams_show))
        // .route("/test/auth_header_token", delete(route::users_teams_show))//.layer(Extension(pool)))//共享变量 必须实现copy 或者 clone
        // .route("/test/jwt", delete(route::users_teams_show))
        // .route("/test/file", post(route::users_teams_show))
        // .route("/test/form", delete(route::users_teams_show))
        .route("/test/json", put(test::Test::json_data))//json 接收并用验证器严重(错误返回json错误) 返回json数据
        .layer(middleware::from_fn(print_request_response));

    let app = app.fallback(handler_404.into_service());//使用了一个Handler的trait

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal()) //优雅关闭
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "this is 404 page")
}
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect(" Ctrl+C 优雅关闭");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("unix 优雅关闭")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("优雅关闭 end");
}

async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{} body = {:?}", direction, body);
    }

    Ok(bytes)
}
