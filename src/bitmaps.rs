use std::fmt;
use std::fmt::Formatter;
use bevy::{prelude::*};
use bevy::ecs::prelude::{Commands, Res};

// Example of creating an Enum that maps to Strings (but *not* str)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OneBackground {
    Map1,
    Map2,
}

// See Lessons - this has to be converted to *str*, for use by asset-loader
impl fmt::Display for OneBackground {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OneBackground::Map1 => write!(f, "unsplash-Ai2TRdvI6gM.png"),
            OneBackground::Map2 => write!(f, "unsplash-cTeQyMstoDI.png"),
        }
    }
}

// Adding a cycler-function (see lessons.md), so I don't have to say things like:
// if map1, set target to map2 else set target to map1...
impl OneBackground {
    fn toggle(&self) -> Self {
        match *self {
            OneBackground::Map1 => OneBackground::Map2,
            OneBackground::Map2 => OneBackground::Map1,
        }
    }
}

// There should only be one Sprite with this marker, or all hell might break loose.
#[derive(Component)]
pub struct IsBackground;

// Resource
pub struct BackgroundMap {
    pub cur_map: OneBackground,
}

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
