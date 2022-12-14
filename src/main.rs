use axum::{
    body::Body,
    extract::MatchedPath,
    handler::Handler,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    Router,
    routing::{delete, get, post, put},
};
use handle::controller::*;
use handle::middleware::test::Test as auth;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

fn log_test() {
    tracing::debug!("debug  ok");
    tracing::info!("info  ok");
    tracing::warn!("warn  ok");
    tracing::trace!("trace  ok");
    tracing::error!("error  ok");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into()) //日志打印级别控制 none>err>warn>info>debug>trace 小于的包含大于的
                .parse_lossy(""),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    log_test();

    let test_router = Router::new()
        .route("/auth_header_token", delete(test::Test::auth))
            .layer(middleware::from_fn(auth::auth))//多个layer 例如包裹括号一样 (2 （1处理） ).layer(Extension(pool)))//共享变量 必须实现copy 或者 clone
        .route("/file", post(test::Test::form_file))
        .route("/form", post(test::Test::form_data)) //form
        .route("/json", put(test::Test::json_data)); //json 接收并用验证器验证(错误返回json错误) 返回json数据
    let user_router = Router::new()
        .route("/users/:user_id", get(user::UserController::user_message).put(user::UserController::user_modify))
        .route("/users/list", get(user::UserController::get_users));

    let app = Router::new()
        .merge(user_router)
        .merge(Router::new().nest("/test", test_router))
        .layer(middleware::from_fn(print_message)); //全局 通用打印

    let app = app.fallback(handler_404.into_service()); //使用了一个Handler的trait

    //tokio 本身对 panic 进行 recover
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_panic(info);
        std::process::exit(1);
    }));

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal()) //优雅关闭
        .await
        .unwrap();
}

async fn print_message(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let path = if let Some(path) = req.extensions().get::<MatchedPath>() {
        path.as_str()
    } else {
        req.uri().path()
    };
    let method = req.method().as_str();

    //所有权 req 被转移了，要复制一份字符串
    let method = String::from(method);
    let path = String::from(path);
    //request 打印 需要过滤掉类似文件上传
    let res = next.run(req).await;
    //response 打印
    tracing::info!("{} {}", method, path);

    Ok(res)
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
