use rapier2d::prelude::*;
use image::{Rgba, RgbaImage};

use std::{
    io::{Cursor, Read, Seek, SeekFrom},
    vec,
    fmt,
};

pub mod file_loader;
mod user;
mod utils;

const MAX_BOUNDS: f32 = 200.0;
const FRICTION: f32 = 0.6;
const SLOWEST_SPEED: f32 = 0.1;

#[derive(fmt::Debug,PartialEq)]
pub enum RendererError{
    Unspecified(String),
    ConfigError(String)
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RendererError::Unspecified(err_str) => write!(f, "Unspecified: {}", err_str),
            RendererError::ConfigError(err_str) => write!(f, "CongifError: {}", err_str)
        }
    }
}

pub struct Context {
    pub frame_width: u32,
    pub frame_height: u32,
    pub max_frames: u32,
    pub debug: bool
}

pub fn render(leveldata: file_loader::LevelDescriptor, output_file_name: String, ctx: Context) -> Result<(), RendererError> {
    if ctx.frame_width < 1 {
        return Err(RendererError::ConfigError("Frame Width must be greater than 0".to_string()));
    }
    if ctx.frame_height < 1 {
        return Err(RendererError::ConfigError("Frame Height must be greater than 0".to_string()));
    }

    // Video setup
    let config = openh264::encoder::EncoderConfig::new(ctx.frame_width, ctx.frame_height);

    let mut vid_encoder = openh264::encoder::Encoder::with_config(config).unwrap();
    let mut buf = Vec::new();

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    for block in leveldata.blocks.iter() {
        let collider = ColliderBuilder::cuboid(block.scale.x, block.scale.y)
            .position(vector![block.location.x, block.location.y].into())
            .build();
        collider_set.insert(collider);
    }

    let mut user_handles: vec::Vec<RigidBodyHandle> = vec::Vec::new();
    for user in leveldata.users.iter() {
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
        if count > ctx.max_frames {
            if ctx.debug {
                println!("ERROR: hit the limit of frames");
            }
            quit = true;
        } else if ctx.debug {
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
                if ctx.debug {
                    println!("\tUser body {} has stopped or moved out of bounds", idx);
                }
            } else {
                all_stopped = false;
            }
        }

        quit = quit || all_stopped;

        // Draw objects to a new frame
        let mut frame: RgbaImage = RgbaImage::new(ctx.frame_width, ctx.frame_height);
        // Set background to dark grey
        for x in 0..ctx.frame_width {
            for y in 0..ctx.frame_height {
                frame.put_pixel(x, y, Rgba([15, 15, 15, 255]));
            }
        }

        // -- Draw blocks to frame
        utils::draw_blocks(
            &mut frame,
            &leveldata.blocks,
            ctx.frame_width,
            ctx.frame_height,
        );

        for (index, user) in leveldata.users.iter().enumerate() {
            let user_body = &mut rigid_body_set[user_handles[index]];
            utils::draw_user(
                &mut frame,
                &file_loader::UserLoc {
                    x: user_body.translation().x,
                    y: user_body.translation().y,
                },
                Rgba([user.color.r, user.color.g, user.color.b, 255]),
                ctx.frame_width,
                ctx.frame_height,
            );
        }

        // Add frame to buffer
        let yuv = openh264::formats::YUVBuffer::with_rgb(
            ctx.frame_width as usize,
            ctx.frame_height as usize,
            utils::rgba8_to_rgb8(frame.clone()).as_raw(),
        );

        let bitstream = vid_encoder.encode(&yuv).unwrap();
        bitstream.write_vec(&mut buf);
    }

    let mut video_buffer = Cursor::new(Vec::new());
    let mut mp4muxer = minimp4::Mp4Muxer::new(&mut video_buffer);
    mp4muxer.init_video(
        ctx.frame_width as i32,
        ctx.frame_height as i32,
        false,
        "Generated Video",
    );
    mp4muxer.write_video(&buf);
    mp4muxer.close();

    let mut video_bytes = Vec::new();
    video_buffer.seek(SeekFrom::Start(0)).unwrap();
    video_buffer.read_to_end(&mut video_bytes).unwrap();

    std::fs::write(output_file_name, &video_bytes).unwrap();
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
