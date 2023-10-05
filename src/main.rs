// use rapier2d::prelude::*;
use chrono::prelude::*;
use core::panic;
use image::{Rgba, RgbaImage};
use rapier2d::prelude::*;

use std::str::FromStr;
use std::{
    io::{Cursor, Read, Seek, SeekFrom},
    vec,
};

mod cli;
mod file_loader;
mod user;
mod utils;

const MAX_BOUNDS: f32 = 200.0;
const FRICTION: f32 = 0.6;
const SLOWEST_SPEED: f32 = 0.1;

fn main() {
    let args = cli::parse_args();

    if args.frame_width < 1 {
        panic!("Frame width must be 1 or greater");
    }
    if args.frame_height < 1 {
        panic!("Frame height must be 1 or greater");
    }

    // Video setup
    let config = openh264::encoder::EncoderConfig::new(args.frame_width, args.frame_height);

    let mut vid_encoder = openh264::encoder::Encoder::with_config(config).unwrap();
    let mut buf = Vec::new();

    // Setup
    let level = match file_loader::get_level_from_file(args.filename) {
        Ok(level) => level,
        Err(err) => {
            panic!("{}", format!("Error loading from file {}", err))
        }
    };

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    for block in level.blocks.iter() {
        let collider = ColliderBuilder::cuboid(block.scale.x, block.scale.y)
            .position(vector![block.location.x, block.location.y].into())
            .build();
        collider_set.insert(collider);
    }

    let mut user_handles: vec::Vec<RigidBodyHandle> = vec::Vec::new();
    for user in level.users.iter() {
        user_handles.push(user::add_user(user, &mut collider_set, &mut rigid_body_set));
    }

    // Set up simulation parameters
    let gravity = vector![0.0, 0.0]; // -9.81
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();

    // Main loop
    let mut count = 0;
    let mut quit = false;
    while !quit {
        count += 1;
        if count > args.max_frames {
            if args.debug {
                println!("ERROR: hit the limit of frames");
            }
            quit = true;
        } else if args.debug {
            println!("Frame: {}", count);
        }

        // Step simulation on
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            None,
            &(),
            &(),
        );

        // Get user objects and check if still moveing
        let mut all_stopped = true;
        for (idx, user_handle) in user_handles.iter_mut().enumerate() {
            let user_body = rigid_body_set.get_mut(*user_handle).unwrap();

            let vel = user_body.linvel();
            let speed_total = vel[0].abs() + vel[1].abs();
            if speed_total < SLOWEST_SPEED {
                // println!("User body {} fell below slowest speed", idx);
                user_body.reset_forces(false);
                user_body.set_linvel(vector![0.0, 0.0], false);
                user_body.set_angvel(0.0, false);
            }

            if user_body.translation().x > MAX_BOUNDS
                || user_body.translation().x < -MAX_BOUNDS
                || user_body.translation().y > MAX_BOUNDS
                || user_body.translation().y < -MAX_BOUNDS
                || !user_body.is_moving()
            {
                if args.debug {
                    println!("\tUser body {} has stopped or moved out of bounds", idx);
                }
            } else {
                all_stopped = false;
            }
        }

        quit = quit || all_stopped;

        // Draw objects to a new frame
        let mut frame: RgbaImage = RgbaImage::new(args.frame_width, args.frame_height);
        // Set background to dark grey
        for x in 0..args.frame_width {
            for y in 0..args.frame_height {
                frame.put_pixel(x, y, Rgba([15, 15, 15, 255]));
            }
        }

        // -- Draw blocks to frame
        utils::draw_blocks(
            &mut frame,
            &level.blocks,
            args.frame_width,
            args.frame_height,
        );

        for (index, user) in level.users.iter().enumerate() {
            let user_body = &mut rigid_body_set[user_handles[index]];
            utils::draw_user(
                &mut frame,
                &file_loader::UserLoc {
                    x: user_body.translation().x,
                    y: user_body.translation().y,
                },
                Rgba([user.color.r, user.color.g, user.color.b, 255]),
                args.frame_width,
                args.frame_height,
            );
        }

        // Add frame to buffer
        let yuv = openh264::formats::YUVBuffer::with_rgb(
            args.frame_width as usize,
            args.frame_height as usize,
            utils::rgba8_to_rgb8(frame.clone()).as_raw(),
        );

        let bitstream = vid_encoder.encode(&yuv).unwrap();
        bitstream.write_vec(&mut buf);
    }

    let mut video_buffer = Cursor::new(Vec::new());
    let mut mp4muxer = minimp4::Mp4Muxer::new(&mut video_buffer);
    mp4muxer.init_video(
        args.frame_width as i32,
        args.frame_height as i32,
        false,
        "Generated Video",
    );
    mp4muxer.write_video(&buf);
    mp4muxer.close();

    let mut video_bytes = Vec::new();
    video_buffer.seek(SeekFrom::Start(0)).unwrap();
    video_buffer.read_to_end(&mut video_bytes).unwrap();

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
        format!("outputs/{}.mp4", args.output_filename)
    };

    std::fs::write(output_file_name, &video_bytes).unwrap();
}
