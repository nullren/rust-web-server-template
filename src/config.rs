use clap::Parser;

/// Web server configuration
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Port to run the web server
    #[clap(short, long, env, default_value = "3000")]
    pub port: String,
}
