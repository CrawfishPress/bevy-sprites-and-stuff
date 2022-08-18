/*
This is basically experimentation with the Bevy game-engine, using a variety of
basic techniques and concepts.
*/

#[allow(unused_imports)]
use bevy::{prelude::*, window::WindowMode, sprite::MaterialMesh2dBundle, winit::WinitSettings};
use bevy::ecs::prelude::Commands;
use bevy_egui::*;

mod screens;
mod bitmaps;
mod data;
mod my_egui;
mod act_moving;
mod act_mousing;
mod act_sprites;

use screens::*;
use bitmaps::*;
use data::*;
use my_egui::*;
use act_sprites::*;
use act_moving::*;
use act_mousing::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!("{} - v{}",
                           env!("CARGO_PKG_NAME"),
                           env!("CARGO_PKG_VERSION")
            ),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            mode: WindowMode::Windowed,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND))
        // .insert_resource(WinitSettings::game()) // Hmmm...
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        .insert_resource(GuiData::default())
        .insert_resource(GameData::default())
        .insert_resource(DragPoint::default())
        .insert_resource(BackgroundMapVisible::default())
        .insert_resource(ScreenManager { current_screen: CurScreen::LoadScreen })

        .add_startup_system(setup_camera)
        .add_system(bevy::window::close_on_esc)

        .add_system(do_display_ui)
        .add_system(do_change_screen)
        .add_system(do_update_background)
        .add_system(do_background_swap)

        // Load-Screen systems
        // okay, none yet - maybe someday...

        // Action-Screen systems
        .add_system(do_remove_sprites)
        .add_system(do_add_sprites)
        .add_system(do_reset_movers)
        .add_system(do_sprite_auto_move)
        .add_system(do_sprite_move_check)

        // More Action-Screen systems
        .add_system(do_movement_input)
        .add_system(check_cursor_for_drag)
        .add_system(check_cursor_for_hover)
        .add_system(apply_any_hovers)
        .add_system(get_cursor_map_coords)
        .run();
    println!("this is a test of the Bevy Engine - alas, this line is never reached, due to a bug");
}

fn setup_camera(mut commands: Commands)
{
    commands  // Camera
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);
}

