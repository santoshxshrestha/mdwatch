use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about,
    long_about = None,
)]
pub struct MdwatchArgs {
    /// Path to the markdown file
    pub file: PathBuf,

    /// IP address to bind the server
    #[clap(short, long, default_value = "127.0.0.1")]
    pub ip: String,

    /// Port number to serve on (If not provided, a random port will be used)
    #[clap(short, long)]
    pub port: Option<u16>,
}
