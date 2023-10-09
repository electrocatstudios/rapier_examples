use rapier2d::prelude::*;

use super::FRICTION;
use crate::file_loader::*;

pub fn add_user(
    user_move: &UserMove,
    collider_set: &mut ColliderSet,
    rigid_body_set: &mut RigidBodySet,
) -> RigidBodyHandle {
    let user_collider = ColliderBuilder::ball(0.5)
        .position(vector![0.0, 0.0].into())
        .restitution(0.7)
        .restitution_combine_rule(CoefficientCombineRule::Max);

    let user_rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
        .translation(vector![user_move.location.x, user_move.location.y])
        .rotation(0.0)
        .linvel(vector![
            user_move.rotation.sin() * (user_move.power * 40.0),
            user_move.rotation.cos() * (user_move.power * 40.0)
        ])
        .linear_damping(FRICTION)
        .angular_damping(0.0)
        .build();

    let user_handle = rigid_body_set.insert(user_rigid_body);
    collider_set.insert_with_parent(user_collider, user_handle, rigid_body_set);
    user_handle
}
