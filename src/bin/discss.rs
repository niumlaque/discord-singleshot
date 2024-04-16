use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use singleshot_discord::discord::{self, LengthBasedSplitter};
use std::io::{BufRead, Read};
use std::path::{Path, PathBuf};

/// Maximum limit of the text to be sent.
/// If it exceeds the limit, it will be sent in divided parts.
/// Counted in characters, not bytes.
const TEXT_LEN_LIMIT: usize = 2000;

#[derive(Debug, Parser)]
struct Cli {
    /// Text to send to discord
    /// if empty, read text from stdin
    text: Option<String>,

    /// Path to config file to read token and channel ID
    #[arg(short = 'f', long, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct Config {
    /// Discord bot token
    token: String,

    /// Channel ID
    channel_id: u64,
}

impl Config {
    /// Load config from file.
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        use std::fs;
        let text = fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }
}

fn get_config(cli: &Cli) -> Result<Config> {
    use std::env;
    if let Some(config_path) = cli.config.as_ref() {
        Config::load(config_path)
    } else {
        let mut path = env::current_exe()?;
        path.pop();
        path.push("ssdisc.toml");
        Config::load(path)
    }
}

/// Read some input until EOF
fn read<R: Read + BufRead>(mut reader: R) -> Result<Vec<String>> {
    let mut ret = Vec::default();
    loop {
        let mut buf = String::new();
        let size = reader.read_line(&mut buf)?;
        if size == 0 {
            // EOF
            break;
        }

        if buf.ends_with("\r\n") {
            buf.pop();
            buf.pop();
        } else if buf.ends_with('\n') || buf.ends_with('\r') {
            buf.pop();
        }

        ret.push(buf);
    }

    Ok(ret)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let text = if let Some(v) = cli.text.as_ref() {
        v.clone()
    } else {
        let lock = std::io::stdin().lock();
        read(lock)?.join("\n")
    };

    let config = get_config(&cli)?;

    discord::SendOnce::send(
        &config.token,
        config.channel_id,
        LengthBasedSplitter::new(text, TEXT_LEN_LIMIT),
    )
    .await?;
    Ok(())
}
