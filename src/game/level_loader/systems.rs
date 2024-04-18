// game / level_loader / systems.rs

use bevy::prelude::*;
use bevy::render::texture;
use bevy::transform::commands;
use bevy::window::PrimaryWindow;



pub fn level_loader(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    //csv_level_data: resource for the csv level data
) {
    let window = window_query.get_single().unwrap();
    let loaded_tileset_texture: Handle<Image> = asset_server.load("sprites/tilemaps/sand_packed.png");

    let tile_length_x_placeholder = 0.1;
    let tile_length_y_placeholder = 0.1;
    let tileset_rows_placeholder = 56;
    let tileset_columns_placeholder = 96;

    let tileset_texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(tile_length_x_placeholder, tile_length_y_placeholder), tileset_columns_placeholder, tileset_rows_placeholder, None, None);
    let tileset_texture_atlas_layout_handle = texture_atlas_layouts.add(tileset_texture_atlas);


}