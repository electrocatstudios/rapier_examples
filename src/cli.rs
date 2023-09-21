use clap::Parser;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Activate debug mode
    // short and long flags, (-d, --debug) derived from the field name
    #[arg(short, long)]
    pub debug: bool,

    /// The maximum number of frames to render
    #[clap(default_value_t = 600)]
    #[arg(short=char::from_str("m").unwrap(), long)]
    pub max_frames: u32,

    /// The input file
    #[clap(default_value = "inputs/box.json")]
    #[arg(short=char::from_str("f").unwrap(), long="file")]
    pub filename: String,
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    args
}
