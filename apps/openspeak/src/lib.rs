mod app;
pub mod cli;
pub mod commands;
pub mod config;
pub mod player;
pub mod providers;

use std::process::ExitCode;

use clap::Parser;

use crate::{
    app::App,
    cli::{Cli, Command},
    config::Config,
};

/// Parse the command line, compose the application, and execute one command.
pub async fn run() -> ExitCode {
    let cli = Cli::parse();

    if let Command::Devices(args) = cli.command {
        return exit_code(commands::devices::execute(args).await);
    }

    let config = match Config::from_env() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error:#}");
            return ExitCode::FAILURE;
        }
    };

    let app = App::new(config);
    let result = tokio::select! {
        result = app.execute(cli.command) => result,
        signal = tokio::signal::ctrl_c() => match signal {
            Ok(()) => Err(anyhow::anyhow!("Operation cancelled.")),
            Err(error) => Err(error.into()),
        },
    };

    exit_code(result)
}

fn exit_code(result: anyhow::Result<()>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}
