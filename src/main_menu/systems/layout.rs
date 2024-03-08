// main_menu / systems / layout.rs

use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>> // <-- this works because out build_main_menu function returns an entity, so it will show up with this query if it exists
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        // Remove the entity and all its children from the app world
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

// Will be a regular rust function, not a system
pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    // Spawn entity that fills 100% of screen during main menu state
    let main_menu_entity = commands.spawn(
    (
            NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                row_gap: Val::Px(8.0),
                column_gap: Val::Px(8.0),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        },
        // For Query calls
        MainMenu {},
    )
    )
    .with_children(|parent| {
        // Title ==
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(300.0),
                    height: Val::Px(120.0),
                    ..default()
                },
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            }
        ).with_children(|title_parent| {
            title_parent.spawn(
                // Image 1 <-- might not have

                // Text
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Jumper", 
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                }
                            )
                        ],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                }


                // Image 2 <-- might not have
            );
        })
        ;
        // Play Button ==
        // using the parent closure lets us spawn using parent as the instance that calls spawn
        parent.spawn(
            (
                ButtonBundle {  
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..default()
                    },
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                PlayButton {},
            )
        )
        .with_children(|parent_button| {
            parent_button.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Play",
                                TextStyle {
                                    // look at tutorial later
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                }
                            )
                        ],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
        parent.spawn(
            (
                ButtonBundle {  
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..default()
                    },
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                OptionsButton {},
            )
        )
        .with_children(|parent_button| {
            parent_button.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Options",
                                TextStyle {
                                    // look at tutorial later
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                }
                            )
                        ],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        })
        ;

        // Options Button == 
    })
    .id(); // <-- the id() method makes the spawn command return an entity, which is what we want to return with our build_main_menu function
    main_menu_entity
}