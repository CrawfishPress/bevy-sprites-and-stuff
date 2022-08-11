/*
This is basically experimentation with the Bevy game-engine, using a variety of
basic techniques and concepts.
*/

#[allow(unused_imports)]
use bevy::{prelude::*, window::WindowMode, sprite::MaterialMesh2dBundle, winit::WinitSettings};
use bevy::ecs::prelude::{Commands, Res};
#[allow(unused_imports)]
use bevy_egui::{egui, EguiContext, EguiPlugin};

mod bitmaps;
mod movers;
mod mousing;
mod data;
mod my_egui;

use bitmaps::*;
use movers::*;
use mousing::*;
use data::*;
use my_egui::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!("{} - v{}",
                           env!("CARGO_PKG_NAME"),
                           env!("CARGO_PKG_VERSION")
            ),
            width: WIDTH,
            height: HEIGHT,
            mode: WindowMode::Windowed,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND))
        // .insert_resource(WinitSettings::game()) // Hmmm...
        .insert_resource(GuiData { some_name: "".to_string(), my_value: 0, my_other_value: 0.0 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        .insert_resource(SpritesMovable { is_active: true })
        .insert_resource(BackgroundMap { cur_map: OneBackground::Map1 })
        // TODO: move these coords out - note the Hovercraft also use them
        .insert_resource(DragPoint { left_point: Vec2 { x: -300.0, y: -200.0 }, right_point: Vec2 { x: 500.0, y: -200.0 }})

        .add_system(do_ui_setup)
        .add_system(do_ui_read)
        .add_startup_system(setup_sprites)
        .add_startup_system(setup_hovercraft)
        .add_system(bevy::window::close_on_esc)

        .add_system(do_sprite_auto_move)
        .add_system(do_sprite_move_check)
        .add_system(do_movement_input)
        .add_system(do_background_swap)
        .add_system(check_cursor_for_drag)
        .run();
    println!("this is a test of the Bevy Engine - alas, this line is never reached, due to a bug");
}

fn setup_sprites(mut commands: Commands,
                 asset_server: Res<AssetServer>,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<ColorMaterial>>,
                 mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle1 = asset_server.load("ferris.png");
    let texture_atlas1 = TextureAtlas::from_grid
        (texture_handle1, Vec2::new(128.0, 85.0), 1, 2);
    let texture_atlas_handle1 = texture_atlases.add(texture_atlas1);

    commands  // Camera
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    add_background(&mut commands, &asset_server, OneBackground::Map1);

    commands // Center Pixel
        .spawn_bundle(MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
            mesh: meshes.add(shape::RegularPolygon::new(4.0, 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()});

    commands  // Tunnel
        .spawn().insert_bundle(SpriteBundle {
            transform: Transform::from_xyz(80.0, -200.0, 3.0),
            sprite: Sprite {
                    color: PADDLE_COLOR,
                    custom_size: Option::from(Vec2 { x: BLOCK_SIZE * 2.0, y: BLOCK_SIZE }),
                    ..default()
                },
                ..default()
            })
        .insert(KeyMover {is_movable: true});

    commands  // Rust-crab
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform::from_xyz(-100., -200., 2.0),
            texture_atlas: texture_atlas_handle1,
            ..default()
        })
        .insert(CrabDirection::Left);
}

fn setup_hovercraft(mut commands: Commands,
                    asset_server: Res<AssetServer>,
                    mut meshes: ResMut<Assets<Mesh>>,
                    mut materials: ResMut<Assets<ColorMaterial>>,
                    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle2 = asset_server.load("unsplash-IVAqc86tTu8_map.png");
    let texture_atlas2 = TextureAtlas::from_grid
        (texture_handle2, Vec2::new(256.0, 256.0), 1, 2);
    let texture_atlas_handle2 = texture_atlases.add(texture_atlas2);

    commands  // Starfish
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform::from_xyz(-300.0, -200.0, 1.0),
            texture_atlas: texture_atlas_handle2,
            ..default()
        })
        .insert(HoverCraft::LeftPoint)
        .insert(IsMousing { is_dragging: false});

    commands // Hexagon
        .spawn_bundle(MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(500.0, -200.0, 1.0)),
            mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BISQUE)),
            ..default()})
        .insert(HoverCraft::RightPoint)
        .insert(IsMousing { is_dragging: false});

}
