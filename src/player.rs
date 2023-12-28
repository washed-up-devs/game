use std::f32::consts::TAU;

use crate::fps_controller::*;
use bevy::{math::vec3, prelude::*};
use bevy_rapier3d::prelude::*;

pub fn player_init(mut commands: Commands) {
    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    let logical_entity = commands
        .spawn((
            Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(Transform::from_translation(vec3(0.0, 4.0, 0.0))),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                air_acceleration: 80.0,
                ..default()
            },
        ))
        .insert(CameraConfig {
            height_offset: 0.0,
            radius_scale: 0.75,
        })
        .id();

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: TAU / 5.0,
                ..default()
            }),
            ..default()
        },
        RenderPlayer { logical_entity },
    ));
}
