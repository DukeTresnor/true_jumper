// main_menu / systems / interactions.rs

use bevy::prelude::*;
use crate::main_menu::{styles::*, MainMenuState};

// Function to handle button presses
pub fn interaction_button(
    //
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    main_menu_state: Res<State<MainMenuState>>,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    // 
    /*
        app_state: Res<State<AppState>>,
    // mutable resource to access the next app state
    mut next_app_state: ResMut<NextState<AppState>>,
    // mutable resource to access the next simulation state
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    
     */
    let mut should_be_in_play_state = false;
    let mut should_be_in_options_state = false;
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        // The tutorial I followed needed this to be mutable??
        let text = text_query.get_mut(children[0]).unwrap();
        // Match against the Interaction reference from the interaction_query
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = "Press".to_string();
                println!("I pressed a button");
                if text.sections[0].value == "Play".to_string() {
                    println!("__ I pressed Play");
                    // Transition to MainMenu::Play
                    should_be_in_play_state = true;
                }
                if text.sections[0].value == "Options".to_string() {
                    println!("__ I pressed Options");
                    // Transition to MainMenu::Options
                    should_be_in_options_state = true;
                }
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON_COLOR.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
    if main_menu_state.get() == &MainMenuState::Menu {
        if should_be_in_play_state {
            next_main_menu_state.set(MainMenuState::Play);
            println!("skhfbjshdvbf");
        }
        if should_be_in_options_state {
            next_main_menu_state.set(MainMenuState::Options);
            println!("skksjfnskdjnkdsjcnksdjcnksdjcnsjkcnsdjn");
        }
    }
    
}