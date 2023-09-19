// use rapier2d::prelude::*;
use chrono::prelude::*;
use rapier2d::prelude::*;
use std::{io::{Cursor, Read, Seek, SeekFrom}, char::MAX};
use image::{RgbaImage,Rgba};

use minimp4;
use openh264;

mod block_loader;
mod utils;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const MAX_FRAME_LIMIT: u32 = 3000;
const MAX_BOUNDS: f32 = 200.0;
const FRICTION: f32 = 0.6;
const SLOWEST_SPEED: f32 = 0.05;

struct UserMove {
    loc: UserLoc,
    rot: f32,
    power: f32
}

pub struct UserLoc {
    x: f32, 
    y: f32
}

fn main() {
    // Video setup    
    let config = openh264::encoder::EncoderConfig::new(WIDTH, HEIGHT);
    let mut vid_encoder = openh264::encoder::Encoder::with_config(config).unwrap();
    let mut buf = Vec::new();

    
    // Setup
    let blocks =  match block_loader::get_blocks_from_file("inputs/box.json".to_string()) {   
        Ok(blocks) => blocks,
        Err(err) => {
            panic!("{}", format!("Error loading from file {}", err))
        }
    };

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    for block in blocks.blocks.iter() {
        let collider = ColliderBuilder::cuboid(
                block.scale.x,
                block.scale.y
            )
            .position(
                vector![block.location.x, block.location.y].into()
            )
            .build();
        collider_set.insert(collider);
    }

    // Add user 1
    let userloc1 = UserLoc{x: 5.0, y:5.0};
    let user1 = UserMove{loc:userloc1, rot:std::f32::consts::PI*1.35, power: 0.5};
    let user1_collider = ColliderBuilder::ball(0.5)
        .position(vector![0.0,0.0].into())
        .restitution(0.7)
        .restitution_combine_rule(CoefficientCombineRule::Max);

    let user1_rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
        .translation(vector![user1.loc.x, user1.loc.y])
        .rotation(0.0)
        .linvel(
            vector![
                user1.rot.sin() * (user1.power * 40.0), 
                user1.rot.cos() * (user1.power * 40.0)
            ]
        )
        .linear_damping(FRICTION)
        .build();

    let user1_handle = rigid_body_set.insert(user1_rigid_body);
    collider_set.insert_with_parent(user1_collider, user1_handle, &mut rigid_body_set);

    // Add user 2
    let userloc2 = UserLoc{x: -5.0, y:5.0};
    let user2 = UserMove{loc:userloc2, rot:std::f32::consts::PI*0.75, power: 0.5};
    let user2_collider = ColliderBuilder::ball(0.5)
        .position(vector![0.0,0.0].into())
        .restitution(0.7)
        .restitution_combine_rule(CoefficientCombineRule::Max);

    let user2_rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
        .translation(vector![user2.loc.x, user2.loc.y])
        .rotation(0.0)
        .linvel(
            vector![
                user1.rot.sin() * (user1.power * 40.0), 
                user1.rot.cos() * (user1.power * 40.0)
            ]
        )
        .linear_damping(FRICTION)
        .build();

    let user2_handle = rigid_body_set.insert(user2_rigid_body);
    collider_set.insert_with_parent(user2_collider, user2_handle, &mut rigid_body_set);

    let gravity = vector![0.0, 0.0]; // -9.81
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    // Main loop 
    // let mut last_pos = UserLoc{x: user1.loc.x, y: user1.loc.y};
    let mut count = 0;
    let mut quit = false;
    while !quit {
        count += 1;
        if count > MAX_FRAME_LIMIT {
            println!("ERROR: hit the limit of frames");
            quit = true;
        }else{
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
            &physics_hooks,
            &event_handler,
        );

        // Get result objects
        {
            let mut user_body1 = &mut rigid_body_set[user1_handle];

            let vel = user_body1.linvel();
            let speed_total = vel[0].abs()  + vel[1].abs();
            if speed_total < SLOWEST_SPEED {
                user_body1.set_linvel(vector![0.0,0.0], false);
            }

            let mut user_body2 = &mut rigid_body_set[user2_handle];

            let vel = user_body2.linvel();
            let speed_total = vel[0].abs()  + vel[1].abs();
            if speed_total < SLOWEST_SPEED {
                user_body2.set_linvel(vector![0.0,0.0], false);
            }
        }
        let user_body1 = &rigid_body_set[user1_handle];
        let user_body2 = &rigid_body_set[user2_handle];

        // Boundary Check all users
        let user1_finished = if user_body1.translation().x > MAX_BOUNDS || user_body1.translation().x < -MAX_BOUNDS 
                                        || user_body1.translation().y > MAX_BOUNDS || user_body1.translation().y < -MAX_BOUNDS {
                                            println!("User1 moved outside of bounds");
                                            true
                                    } else if !user_body1.is_moving() {
                                        println!("User1 not moving");
                                        true
                                    } else {
                                        false
                                    };
        let user2_finished = if user_body2.translation().x > MAX_BOUNDS || user_body2.translation().x < -MAX_BOUNDS 
                                        || user_body2.translation().y > MAX_BOUNDS || user_body2.translation().y < -MAX_BOUNDS {
                                            println!("User2 moved outside of bounds");
                                            true
                                    } else if !user_body2.is_moving() {
                                        println!("User2 not moving");
                                        true
                                    } else {
                                        false
                                    };
        
        quit = quit || (user1_finished && user2_finished);

        // last_pos.x = user_body.translation().x;
        // last_pos.y = user_body.translation().y;

        // Draw objects to a new frame
        let mut frame: RgbaImage = RgbaImage::new(WIDTH, HEIGHT);
        // Set background to dark grey
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                frame.put_pixel(x,y,Rgba([15,15,15,255]));
            }
        }

        // -- Draw blocks to frame
        utils::draw_blocks(&mut frame, &blocks);
      
        utils::draw_user(
            &mut frame, 
            &UserLoc { 
                x: user_body1.translation().x, 
                y: user_body1.translation().y 
            },
            Rgba([0,128,255,255])
        );
        utils::draw_user(
            &mut frame, 
            &UserLoc { 
                x: user_body2.translation().x, 
                y: user_body2.translation().y 
            },
            Rgba([128,0,255,255])
        );

        // Add frame to buffer
        let yuv = openh264::formats::YUVBuffer::with_rgb(WIDTH as usize, HEIGHT as usize,&utils::rgba8_to_rgb8(frame.clone()).as_raw());
        let bitstream = vid_encoder.encode(&yuv).unwrap();
        bitstream.write_vec(&mut buf);
    }

    let mut video_buffer = Cursor::new(Vec::new());
    let mut mp4muxer = minimp4::Mp4Muxer::new(&mut video_buffer);
    mp4muxer.init_video(WIDTH as i32, HEIGHT as i32, false, "Generated Video");
    mp4muxer.write_video(&buf);
    mp4muxer.close();

    let mut video_bytes = Vec::new();
    video_buffer.seek(SeekFrom::Start(0)).unwrap();    
    video_buffer.read_to_end(&mut video_bytes).unwrap();
    
    let cur_time =  Utc::now();
    let cur_time_str = format!("{}", cur_time).replace(":", "").replace(" ", "").replace("-", "");
    let date_str = cur_time_str.split(".").collect::<Vec<_>>()[0];
    let output_file_name = format!("outputs/output_{}.mp4", date_str);
    
    std::fs::write(output_file_name, &video_bytes).unwrap();

}
