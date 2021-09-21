mod config;
mod file;
mod folder;

use clap::{crate_authors, crate_description, crate_version, Clap};
use pcloud::http::HttpClient;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(about = crate_description!(), author = crate_authors!(), version = crate_version!())]
struct Command {
    #[clap(
        short,
        long,
        about = "Path to load the configuration file. Default to ~/.config/pcloud.json. If not found, loading from environment."
    )]
    config: Option<PathBuf>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Command {
    fn config(&self) -> PathBuf {
        if let Some(ref cfg) = self.config {
            cfg.clone()
        } else if let Some(cfg_dir) = dirs::config_dir() {
            cfg_dir.join("pcloud.json")
        } else {
            PathBuf::from(".pcloud.json")
        }
    }
}

#[derive(Clap)]
enum SubCommand {
    #[clap(about = "Folder related sub command")]
    Folder(folder::Command),
    #[clap(about = "File related sub command")]
    File(file::Command),
}

impl Command {
    async fn execute(&self, pcloud: HttpClient) {
        match &self.subcmd {
            SubCommand::Folder(sub) => sub.execute(pcloud).await,
            SubCommand::File(sub) => sub.execute(pcloud).await,
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let cmd = Command::parse();
    let pcloud = config::Config::from_path(&cmd.config())
        .map(|cfg| cfg.build())
        .unwrap_or_else(|_| HttpClient::from_env());
    cmd.execute(pcloud).await;
}
