use anyhow::Result;
use axum::Router;
use clap::{Parser, Subcommand};
use reqwest::Url;

use crate::{api, model::SynthesizeRequest, service::AppState};

#[derive(Parser)]
#[command(
    name = "yapper-server",
    about = "Yapper TTS server + standalone client CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Serve {
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value_t = 5100)]
        port: u16,
    },
    Health {
        #[arg(long, default_value = "http://127.0.0.1:5100")]
        base_url: String,
    },
    Models {
        #[arg(long, default_value = "http://127.0.0.1:5100")]
        base_url: String,
    },
    Synthesize {
        text: String,
        #[arg(long, default_value = "openai-tts-1-hd-alloy")]
        model_id: String,
        #[arg(long, default_value = "output.wav")]
        output: String,
        #[arg(long, default_value = "http://127.0.0.1:5100")]
        base_url: String,
    },
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Serve { host, port } => serve(&host, port).await,
        Commands::Health { base_url } => health(&base_url).await,
        Commands::Models { base_url } => models(&base_url).await,
        Commands::Synthesize {
            text,
            model_id,
            output,
            base_url,
        } => synthesize(&base_url, &text, &model_id, &output).await,
    }
}

async fn serve(host: &str, port: u16) -> Result<()> {
    let state = AppState::new();
    let models_loaded = state.models.len();
    let app: Router = api::router(state);
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;
    println!("Yapper server listening on http://{host}:{port} ({models_loaded} models configured)");
    if models_loaded == 0 {
        println!(
            "No API-backed models configured. Set OPENAI_API_KEY and/or ELEVENLABS_API_KEY (+ ELEVENLABS_VOICE_ID)."
        );
    }
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health(base_url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let url = Url::parse(base_url)?.join("/v1/health")?;
    let body = client.get(url).send().await?.text().await?;
    println!("{body}");
    Ok(())
}

async fn models(base_url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let url = Url::parse(base_url)?.join("/v1/models")?;
    let body = client.get(url).send().await?.text().await?;
    println!("{body}");
    Ok(())
}

async fn synthesize(base_url: &str, text: &str, model_id: &str, output: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let synth_url = Url::parse(base_url)?.join("/v1/synthesize")?;
    let req = SynthesizeRequest {
        text: text.into(),
        model_id: model_id.into(),
        stream: true,
        format: "wav".into(),
    };

    let synth: serde_json::Value = client
        .post(synth_url)
        .json(&req)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let stream_path = synth
        .get("stream_url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing stream_url in synth response"))?;

    let audio_url = Url::parse(base_url)?.join(stream_path)?;
    let bytes = client
        .get(audio_url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    tokio::fs::write(output, bytes).await?;
    println!("Saved audio to {output}");
    Ok(())
}
