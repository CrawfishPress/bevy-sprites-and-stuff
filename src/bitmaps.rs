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

// There should only be one Sprite with this marker, or all hell might break loose.
#[derive(Component)]
pub struct IsBackground;

// Resource
pub struct BackgroundMap {
    pub cur_map: OneBackground,
}

pub fn add_background(commands: &mut Commands, asset_server: &Res<AssetServer>, some_bitmap: &str)
{
    commands  // Background Map
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load(some_bitmap),
            ..default()
        })
        .insert(IsBackground);
}

/*
do_background_swap(): does two things, when spacebar is pressed:

 - queries for any active Map Sprite, and de-spawns it - this will make
   the screen-background disappear (game is also paused on spacebar).

 - if no active Map Sprite, toggles the current BackgroundMap, then
   spawns a new Sprite, with the current BackgroundMap.
*/
pub fn do_background_swap(mut commands: Commands,
                          asset_server: Res<AssetServer>,
                          keyboard_input: Res<Input<KeyCode>>,
                          mut cur_backmap: ResMut<BackgroundMap>,
                          mut sprite_map_qry: Query<Entity, With<IsBackground>>,
){
    if keyboard_input.just_pressed(KeyCode::Space) {
        if ! sprite_map_qry.is_empty() {
            let entity_id = sprite_map_qry.get_single_mut().unwrap();  // Gosh, I sure hope there's only one.
            // println!("spacebar hit. removing background Sprite: {:?}", entity_id);
            commands.entity(entity_id).despawn();
            return;
        }

        if cur_backmap.cur_map == OneBackground::Map1 {
            cur_backmap.cur_map = OneBackground::Map2;
        } else {
            cur_backmap.cur_map = OneBackground::Map1;
        }
        // println!("Background empty? wow... Adding background: [{}]", cur_backmap.cur_map);
        add_background(&mut commands, &asset_server, &*cur_backmap.cur_map.to_string());
    }
}
