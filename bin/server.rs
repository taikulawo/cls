use std::collections::HashMap;

use anyhow::bail;
use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{any, get},
    Router,
};
use cls::config::OVERRIDE;
use reqwest::StatusCode;
const PORT: i32 = 6789;
#[tokio::main]
async fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    // build our application with a single route
    let app = Router::new()
        .route("/cls-converter", get(convert))
        .route("/", any(|| async { StatusCode::NOT_FOUND }));

    // run it with hyper on localhost:6789
    log::info!("cls-server listening at {}", PORT);
    axum::Server::bind(&format!("127.0.0.1:{PORT}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn convert(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    match do_convert(params).await {
        Ok(res) => (StatusCode::OK, res).into_response(),
        Err(err) => {
            log::error!("cls converter error at {:?}", err);
            (StatusCode::BAD_REQUEST, err.to_string()).into_response()
        }
    }
}
async fn do_convert(params: HashMap<String, String>) -> anyhow::Result<String> {
    let url = params.get("url");
    if let Some(u) = url {
        let config = reqwest::get(u).await?.text().await?;
        // server无法获得本地dns配置，无法涉及dns/tun的配置，
        // 所以 tun 永远关闭， 如需要开启，用二次cls处理
        let result = cls::clash::perform_merge(OVERRIDE.clone(), config, true)?;
        serde_yaml::to_string(&result).map_err(anyhow::Error::from)
    } else {
        bail!("url required")
    }
}
