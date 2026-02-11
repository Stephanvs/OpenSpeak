mod api;
mod cli;
mod model;
mod service;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await
}
