use chrono::prelude::*;
use core::panic;
use renderer::*;
use std::str::FromStr;

mod cli;

fn main() {
    let args = cli::parse_args();

    if args.frame_width < 1 {
        panic!("Frame width must be 1 or greater");
    }
    if args.frame_height < 1 {
        panic!("Frame height must be 1 or greater");
    }

    // Setup
    let leveldata = match file_loader::get_level_from_file(args.filename) {
        Ok(level) => level,
        Err(err) => {
            panic!("{}", format!("Error loading from file {}", err))
        }
    };
    
    let cur_time = Utc::now();
    let cur_time_str = format!("{}", cur_time).replace(
        [
            char::from_str(":").unwrap(),
            char::from_str(" ").unwrap(),
            char::from_str("-").unwrap(),
        ],
        "",
    );

    let date_str = cur_time_str
        .split(char::from_str(".").unwrap())
        .collect::<Vec<_>>()[0];

    let output_file_name = if args.output_filename == "<blank>" {
        format!("outputs/output_{}.mp4", date_str)
    } else {
        args.output_filename.to_string()
    };

    let ctx = Context{
        frame_width: args.frame_width,
        frame_height: args.frame_height,
        max_frames: args.max_frames,
        debug: args.debug
    };

    match render(leveldata, output_file_name, ctx) {
        Ok(_) => {},
        Err(err) => panic!("{}", err)
    }
}
