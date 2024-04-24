// game / level_loader / systems.rs

use bevy::prelude::*;
use bevy::render::texture;
use bevy::transform::commands;
use bevy::window::PrimaryWindow;
//use bevy::asset::RecursiveDependencyLoadState;
//use bevy::reflect::TypePath;
//use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};
//use bevy_common_assets::json::JsonAssetPlugin;
use csv::Reader;

use super::LevelData;

pub fn level_setup(
    mut level_data: ResMut<LevelData>,
) {
    let mut row_number = 0;
    let mut col_number = 0;

    let mut rdr = Reader::from_path("assets/tile-based-game/simplified/Level_0/Walls.csv").unwrap();
    for result in rdr.records() {
        
        let record = result.expect("Error with the record");
        row_number += 1;
        
        //println!("fn level_setup: record: {:?}", record);

        for value in record.iter() {

            col_number += 1;

            // Turn to match case if there's more than just 1's and 0's
            if value == "1" {
                // store current row and column number into level data resource
                
                level_data.cell_values.push(1);
            }
            else {
                level_data.cell_values.push(0);
            }

            //println!("fn level_setup: value: {}", value);
        }

        level_data.col_number = col_number;
        col_number = 0;

    }

    level_data.row_number = row_number;
    row_number = 0;
    
}



pub fn level_loader(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
   level_data: Res<LevelData>,
) {
    let window = window_query.get_single().unwrap();
    let loaded_tileset_texture: Handle<Image> = asset_server.load("sprites/tilemaps/sand_packed.png");

    // These will come from csv_level_data eventually
    let tile_length_x_placeholder = 18.0;
    let tile_length_y_placeholder = 18.0;
    //let tileset_rows_placeholder = 56;
    //let tileset_columns_placeholder = 96;
    let tileset_rows = level_data.row_number as usize;
    let tileset_columns = level_data.col_number as usize;

    let mut current_row_placeholder = 0;
    let mut current_column_placeholder = 0;


    let tileset_texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(tile_length_x_placeholder, tile_length_y_placeholder), tileset_columns, tileset_rows, None, None);
    let tileset_texture_atlas_layout_handle = texture_atlas_layouts.add(tileset_texture_atlas);


    println!("fn level_loader: cell values: {:?}", level_data.cell_values);
    println!("fn level_loader: row_number: {}, column_number: {}", level_data.row_number, level_data.col_number);

    for (index, cell) in level_data.cell_values.iter().enumerate() {
        let inner_tileset_texture_atlas_layout_handle = tileset_texture_atlas_layout_handle.clone();
        let inner_tileset_texture = loaded_tileset_texture.clone();

        if *cell == 1 {
            // spawn a block at the correct row and col position
            let block_transform: Transform = Transform::from_xyz(current_row_placeholder as f32 * tile_length_y_placeholder, current_column_placeholder as f32  * tile_length_x_placeholder, 0.0);
            let block_entity = commands.spawn(
                SpriteSheetBundle {
                    transform: block_transform,
                    texture: inner_tileset_texture,
                    atlas: TextureAtlas {
                        layout: inner_tileset_texture_atlas_layout_handle,
                        index: 0 // <-- should this be 1??
                    },
                    ..default()
                }
            );

            //println!("fn level_loader: block_transform: {:?}", block_transform);
        }

        current_column_placeholder += 1;

        if current_column_placeholder >= tileset_columns {

            current_row_placeholder += 1;
            current_column_placeholder = 0;
        }

        println!("fn level_loader: current_column_placeholder: {}, current_row_placeholder: {}", current_column_placeholder, current_row_placeholder);
        println!("fn level_loader: index: {}", index);
    }
}

