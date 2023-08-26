use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::prelude::*;
use rand::Rng;

pub struct StarsPlugin;

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_stars);
    }
}

#[derive(Component)]
pub struct Star;

const STAR_RANGE: i32 = 8000;
const BOX_SIZE: i32 = 64;
const LAYERS: i32 = 25;
// 1 is large star 8 is small


fn generate_stars(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle = assets.load("stars_sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 9, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut rng = rand::thread_rng();

    commands.spawn((Name::new("Stars"), SpatialBundle::default()))
        .with_children(|commands| {
            for layer in 1..LAYERS {
                for x in (-STAR_RANGE..STAR_RANGE).step_by(BOX_SIZE as usize) {
                    for y in (-STAR_RANGE..STAR_RANGE).step_by(BOX_SIZE as usize) {
                        if rng.gen_bool(layer as f64 / 100.) {
                            let (box_x, box_y) = (rng.gen_range(1.0..BOX_SIZE as f32), rng.gen_range(1.0..BOX_SIZE as f32));
                            commands.spawn(SpriteSheetBundle {
                                texture_atlas: texture_atlas_handle.clone(),
                                sprite: TextureAtlasSprite {
                                    color: Color::rgb(1.3 - (layer as f32 / 50.), 1.3 - (layer as f32 / 50.), 1.3 - (layer as f32 / 50.)),
                                    index: layer.clamp(0, 8) as usize,
                                    ..default()
                                },
                                transform: Transform::from_xyz(x as f32 + box_x, y as f32 + box_y, 0.),
                                ..default()
                            });
                        }
                    }
                }
            }
        });
}