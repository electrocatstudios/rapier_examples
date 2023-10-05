use std::str::FromStr;
use clap::Parser;

#[derive(Parser,Debug)]
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

    /// The output filename (without file extension)
    #[clap(default_value = "<blank>")]
    #[arg(short=char::from_str("o").unwrap(), long)]
    pub output_filename: String,

    /// The output video width
    #[clap(default_value_t = 1920)]
    #[arg(short=char::from_str("w").unwrap(), long)]
    pub frame_width: u32,

    /// The output video height
    #[clap(default_value_t = 1080)]
    #[arg(short=char::from_str("l").unwrap(), long)]
    pub frame_height: u32,
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    args
}