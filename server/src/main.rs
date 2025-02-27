use anyhow::{anyhow, Context};
use dotenv::dotenv;
use std::net::TcpListener;

use libcm::ChatManager;
use server::Config;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cfg = envy::from_env::<Config>().with_context(|| "Failed to load conf from Env")?;

    println!("Using model: {}", cfg.model_name);
    let chat_manager = ChatManager::new(cfg.model_name, None);

    let actix_future = server::http_server::serve(
        TcpListener::bind(&cfg.http_server_addr)
            .with_context(|| format!("Failed to bind to port: {0}", cfg.http_server_addr))?,
        chat_manager,
    )
    .map_err(|e| anyhow!("Failed to create future: {e:?}"))?;

    actix_future.await.with_context(|| "Server failed")?;
    Ok(())
}
