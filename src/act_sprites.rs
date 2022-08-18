#[allow(unused_imports)]
use bevy::{prelude::*, window::WindowMode, sprite::MaterialMesh2dBundle, winit::WinitSettings};
use crate::*;

pub fn do_remove_sprites(mut commands: Commands,
                         game_status: ResMut<GameData>,
                         mut any_sprites: Query<Entity, With<IsActionScreen>>,
) {
    if ! game_status.is_changed() { return; }
    if game_status.action_status != ActionState::OffScreen { return; }

    // Theoretically, we are in state OffScreen, *and* game_status has just been changed.
    // println!("Removing any sprites");

    // If any existing Action-Screen Sprites, terminate with extreme prejudice
    for one_entity_id in any_sprites.iter_mut() {
        // println!("*** Removing: {:?}", one_entity_id);
        commands.entity(one_entity_id).despawn();
    }

    // game_status.action_status = ActionState::Loading;
    // println!("Done removing sprites, State: {:?}", game_status.action_status);
}

pub fn do_add_sprites(mut commands: Commands,
                      asset_server: Res<AssetServer>,
                      screen_mgr: Res<ScreenManager>,
                      mut game_status: ResMut<GameData>,
                      mut meshes: ResMut<Assets<Mesh>>,
                      mut materials: ResMut<Assets<ColorMaterial>>,
                      texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if screen_mgr.current_screen != CurScreen::ActionScreen { return; }
    if game_status.action_status != ActionState::Loading { return; }
    // println!("\nAdding tunnel, pixel, State: Loading");

    commands // Center Pixel
        .spawn_bundle(MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
            mesh: meshes.add(shape::RegularPolygon::new(4.0, 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()
        })
        .insert(IsActionScreen);

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
        .insert(KeyMover {is_movable: true})
        .insert(IsActionScreen);

    add_movers_action(commands, asset_server, meshes, materials, texture_atlases);

    game_status.action_status = ActionState::Running;
    // println!("Done adding tunnel, pixel, State: Running\n");

}

pub fn do_reset_movers(mut commands: Commands,
                       asset_server: Res<AssetServer>,
                       meshes: ResMut<Assets<Mesh>>,
                       materials: ResMut<Assets<ColorMaterial>>,
                       texture_atlases: ResMut<Assets<TextureAtlas>>,
                       mut move_active: ResMut<GameData>,
                       mut movers_query: Query<Entity, With<OneMover>>,
) {
    if move_active.action_status != ActionState::Resetting { return; }

    // Say goodby to all the Mover Sprites
    for one_sprite_id in movers_query.iter_mut() {
        // println!("\treset_movers, removing: {:?}", one_sprite_id);
        commands.entity(one_sprite_id).despawn();
    }

    // Reset all the variables in DragPoint - by replacing it?
    commands.remove_resource::<DragPoint>();
    commands.insert_resource(DragPoint::default());

    add_movers_action(commands, asset_server, meshes, materials, texture_atlases);

    move_active.action_status = ActionState::Running;
}

fn add_movers_action(mut commands: Commands,
                     asset_server: Res<AssetServer>,
                     mut meshes: ResMut<Assets<Mesh>>,
                     mut materials: ResMut<Assets<ColorMaterial>>,
                     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let texture_handle1 = asset_server.load("ferris.png");
    let texture_atlas1 = TextureAtlas::from_grid
        (texture_handle1, Vec2::new(128.0, 85.0), 1, 2);
    let texture_atlas_handle1 = texture_atlases.add(texture_atlas1);

    let texture_handle2 = asset_server.load("unsplash-IVAqc86tTu8_map.png");
    let texture_atlas2 = TextureAtlas::from_grid
        (texture_handle2, Vec2::new(256.0, 256.0), 1, 2);
    let texture_atlas_handle2 = texture_atlases.add(texture_atlas2);

    commands  // Rust-crab
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform::from_xyz(-100., -200., 2.0),
            texture_atlas: texture_atlas_handle1,
            ..default()
        })
        .insert(CrabDirection::Left)
        .insert(OneMover)
        .insert(IsActionScreen);

    commands  // Starfish
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform::from_xyz(-300.0, -200.0, 1.0),
            texture_atlas: texture_atlas_handle2,
            ..default()
        })
        .insert(HoverCraft::LeftPoint)
        .insert(IsMousing { is_dragging: false, is_hovering: false })
        .insert(OneMover)
        .insert(IsActionScreen);

    commands // Hexagon
        .spawn_bundle(MaterialMesh2dBundle {
            transform: Transform::from_translation(Vec3::new(500.0, -200.0, 1.0)),
            mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BISQUE)),
            ..default()})
        .insert(HoverCraft::RightPoint)
        .insert(IsMousing { is_dragging: false, is_hovering: false })
        .insert(OneMover)
        .insert(IsActionScreen);
}
