use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;

use tracing::info;

use rest_api::{AppConfig, ApplicationServer, Database, Logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .env 파일 로드 아마도 std::env 에
    dotenv().ok();

    // Parse from std::env >> config.rs
    let config = Arc::new(AppConfig::parse());

    // 로거 생성 logger.rs
    let _guard = Logger::init(config.cargo_env);

    // DB 접속 및 초기화(.env 에서 설정 가능)
    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let db = Database::connect(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");

    ApplicationServer::serve(config, db)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
