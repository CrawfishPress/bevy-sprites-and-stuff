use bevy::{prelude::*};
use bevy::ecs::prelude::{Commands, Res};
use crate::{BackgroundMap, IsBackground, OneBackground};

/*
do_background_swap(): when spacebar is pressed:
 - finds active Background-map sprite
 - despawns it
 - respawns the next Map in the Enum
*/
pub fn do_background_swap(mut commands: Commands,
                          asset_server: Res<AssetServer>,
                          keyboard_input: Res<Input<KeyCode>>,
                          mut cur_backmap: ResMut<BackgroundMap>,
                          mut sprite_map_qry: Query<Entity, With<IsBackground>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // This should never happen - originally, there was another step, where there
        // was no background-sprite, and the next spacebar refreshed to another map.
        // I guess I could maybe just delete it, or something...
        if sprite_map_qry.is_empty() { return; }

        let entity_id = sprite_map_qry.get_single_mut().unwrap();  // Gosh, I sure hope there's only one.
        // println!("spacebar hit. removing background Sprite: {:?}", entity_id);
        commands.entity(entity_id).despawn();
        cur_backmap.cur_map = cur_backmap.cur_map.toggle();
        add_background(&mut commands, &asset_server, cur_backmap.cur_map);
    }
}

pub fn add_background(commands: &mut Commands, asset_server: &Res<AssetServer>, some_bitmap: OneBackground)
{
    let background_string = &*some_bitmap.to_string();
    commands  // Background Map
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load(background_string),
            ..default()
        })
        .insert(IsBackground);
}
