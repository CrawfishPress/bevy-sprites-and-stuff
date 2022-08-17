/*
This is basically experimentation with the Bevy game-engine, using a variety of
basic techniques and concepts.
*/

#[allow(unused_imports)]
use bevy::{prelude::*, window::WindowMode, sprite::MaterialMesh2dBundle, winit::WinitSettings};
use bevy::ecs::prelude::Commands;
#[allow(unused_imports)]
use bevy_egui::{egui, EguiContext, EguiPlugin};

mod bitmaps;
mod movers;
mod mousing;
mod data;
mod my_egui;
mod screens;
mod my_sprites;

use bitmaps::*;
use movers::*;
use mousing::*;
use data::*;
use my_egui::*;
use screens::*;
use my_sprites::*;

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
        .add_system(do_ui_setup)

        .add_system(do_change_screen)
        .add_system(do_update_background)
        .add_system(do_background_swap_action)

        .add_system(do_remove_sprites_action)
        .add_system(do_add_sprites_action)
        .add_system(do_reset_movers_action)

        .add_system(do_sprite_auto_move)
        .add_system(do_sprite_move_check)
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

