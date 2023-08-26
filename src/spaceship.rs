use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_ship);
        app.add_systems(Update, move_player_ship);
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

fn move_player_ship(keys: Res<Input<KeyCode>>, mut query: Query<(&mut ExternalForce, &Velocity), With<Spaceship>>) {
    let (mut force, velocity) = query.single_mut();

    let mut forward_force = 0.;
    let mut sideways_force = 0.;
    let mut turn_force = 0.;


    if keys.pressed(KeyCode::W) {
        forward_force += 5.;
    }
    if keys.pressed(KeyCode::S) {
        forward_force -= 5.;
    }

    if keys.pressed(KeyCode::ControlLeft) {
        if keys.pressed(KeyCode::A) {
            sideways_force -= 3.;
        }
        if keys.pressed(KeyCode::D) {
            sideways_force += 3.;
        }
    } else {
        if keys.pressed(KeyCode::A) {
            turn_force -= 1.;
        }
        if keys.pressed(KeyCode::D) {
            turn_force += 1.;
        }
    }

}