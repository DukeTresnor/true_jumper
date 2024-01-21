// game / player / systems.rs


use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::components::*;
use crate::game::player::components::*;


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query.get_single().unwrap();

    let main_transform_player: Transform = Transform::from_xyz(window.width() / 4.0, window.height() / 2.0, 0.0);
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/ball_blue_large.png"),
        Vec2::new(64.0, 64.0), 1, 1, None, None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indeces = AnimationIndices { first: 0, last: 0 };


    commands.spawn(
        (
            SpriteSheetBundle {
                transform: main_transform_player,
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indeces.first),
                ..default()
            },
            Player {},
            
        )
    );
}
    