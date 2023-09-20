use rapier2d::prelude::*;

use super::FRICTION;
use crate::file_loader::*;

pub fn add_user(user_move: &UserMove, collider_set: &mut ColliderSet, rigid_body_set: &mut RigidBodySet) -> RigidBodyHandle {
    // let userloc1 = UserLoc{x: 5.0, y:5.0};
    // let user1 = UserMove{loc:userloc1, rot:std::f32::consts::PI*1.35, power: 0.5};
    let user1_collider = ColliderBuilder::ball(0.5)
        .position(vector![0.0,0.0].into())
        .restitution(0.7)
        .restitution_combine_rule(CoefficientCombineRule::Max);

    let user1_rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
        .translation(vector![user_move.loc.x, user_move.loc.y])
        .rotation(0.0)
        .linvel(
            vector![
                user_move.rot.sin() * (user_move.power * 40.0), 
                user_move.rot.cos() * (user_move.power * 40.0)
            ]
        )
        .linear_damping(FRICTION)
        .build();

    let user1_handle = rigid_body_set.insert(user1_rigid_body);
    collider_set.insert_with_parent(user1_collider, user1_handle, rigid_body_set);
    user1_handle
}