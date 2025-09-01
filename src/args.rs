use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "mdserve",
    author = "santoshxshrestha",
    version,
    about = "A CLI tool to live-preview Markdown files in your browser using a local web server"
)]
pub struct MdwatchArgs {
    /// Path to the markdown file
    pub file: String,

    /// IP address to bind the server (default: 127.0.0.1)
    #[clap(short, long, default_value = "127.0.0.1")]
    pub ip: String,

    /// Port number to serve on (default: 3000)
    #[clap(short, long, default_value_t = 3000)]
    pub port: u16,
}
