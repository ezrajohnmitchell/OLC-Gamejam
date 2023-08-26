use bevy::app::{App, Plugin};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::na::Translation;
use bevy_rapier2d::prelude::*;
use crate::CameraController;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_ship);
        app.add_systems(Update, move_player_ship);
        app.add_systems(Update, follow_cam);
    }
}

#[derive(Component)]
pub struct Spaceship;

fn spawn_player_ship(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn((
        SpatialBundle::default(),
        Spaceship,
        RigidBody::Dynamic,
        GravityScale(0.),
        Velocity::default(),
        ExternalForce::default(),
        Name::new("Player Spaceship")))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: asset.load("player_ship.png"),
                ..default()
            });
            parent.spawn(Collider::capsule_y(15., 5.));
        });
}

fn move_player_ship(keys: Res<Input<KeyCode>>, mut query: Query<(&mut ExternalForce, &Velocity, &Transform), With<Spaceship>>) {
    let (mut force, velocity, transform) = query.single_mut();

    let mut forward_force = 0.;
    let mut sideways_force = 0.;
    let mut turn_force = 0.;


    if keys.pressed(KeyCode::Space) {
        // movement braking
        let magnitude = velocity.linvel.length();
        if magnitude > f32::EPSILON {
            let brake_force = (magnitude + 1.).log(10.);
            force.force = velocity.linvel.clone().normalize() * -brake_force * 10.;
        }

        // turn braking
        if velocity.angvel.abs() > 0. {
            let ang_brake_force = (velocity.angvel.abs() + 1.).log(10.);
            force.torque = velocity.angvel.signum() * -1. * ang_brake_force;
        }
    } else {
        let movement_direction = transform.rotation * Vec3::Y;
        let right_vec = transform.rotation * Vec3::X;

        if keys.pressed(KeyCode::W) {
            if keys.pressed(KeyCode::ShiftLeft) {
                forward_force += 15.;
            } else {
                forward_force += 10.;
            }
        }
        if keys.pressed(KeyCode::S) {
            forward_force -= 5.;
        }

        if keys.pressed(KeyCode::ControlLeft) {
            if keys.pressed(KeyCode::A) {
                sideways_force -= 5.;
            }
            if keys.pressed(KeyCode::D) {
                sideways_force += 5.;
            }
        } else {
            if keys.pressed(KeyCode::A) {
                turn_force += 0.05;
            }
            if keys.pressed(KeyCode::D) {
                turn_force -= 0.05;
            }
        }

        let final_force = (movement_direction.xy() * forward_force) + (right_vec.xy() * sideways_force);

        force.force = final_force;
        force.torque = turn_force;
    }
}

fn follow_cam(mut cam_q: Query<&mut Transform, (With<CameraController>, Without<Spaceship>)>, player_q: Query<&Transform, (With<Spaceship>, Without<CameraController>)>) {
    let mut cam_transform = cam_q.single_mut();
    let player_transform = player_q.single();

    cam_transform.translation = cam_transform.translation.lerp(player_transform.translation, 0.2);
}