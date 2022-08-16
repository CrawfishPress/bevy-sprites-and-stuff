/*
bitmaps::add_background():
Originally this was not a System - it was just a function, that random Systems called,
whenever there was a background-change. Then I moved to a multiple-Screen configuration,
where the Loading Screen had one background, and the Action Screen could have multiple
backgrounds. As a result, I promoted add_background() to a System, that just monitored the current
Backgroundmap Resource, for changes. When it found a change, it swapped out maps (via Sprites).

So this essentially *decouples* the Game Data structure, which has an "abstract" BackgroundMap,
from the visual bitmap (Sprite) that is displayed, using Systems and Resources.

For instance, do_background_swap_action() was a lot longer, and originally worked like this:
  when spacebar is pressed:
   - find active Background-map sprite
     - despawn it
   - respawn the next Map in the Enum

Now it works like this:
  when spacebar is pressed:
   - toggles the next Map in the BackgroundMap Resource

And it's up to some other System, to be aware that BackgroundMap Resource has changed,
and despawn/respawn the Bitmap Sprites to handle it.
*/

use bevy::{prelude::*};
use bevy::ecs::prelude::{Commands, Res};
use crate::*;


pub fn do_background_swap_action(keyboard_input: Res<Input<KeyCode>>,
                                 screen_mgr: Res<ScreenManager>,
                                 mut background_mgr: ResMut<BackgroundMapVisible>,
) {
    if screen_mgr.current_screen != CurScreen::ActionScreen { return; }

    if keyboard_input.just_pressed(KeyCode::Space) {
        background_mgr.cur_map = background_mgr.cur_map.toggle();
    }
}

pub fn do_update_background(mut commands: Commands,
                            asset_server: Res<AssetServer>,
                            background_mgr: Res<BackgroundMapVisible>,
                            mut sprite_map_qry: Query<Entity, With<IsBackground>>,
){

    if ! background_mgr.is_changed() { return; }

    // Remove any existing Backgrounds - but *only* if they exist...
    let maybe_entity_id = sprite_map_qry.get_single_mut();  // Returns a Result<Entity, Error>
    if maybe_entity_id.is_ok() {
        let entity_id = maybe_entity_id.unwrap();
        commands.entity(entity_id).despawn();
    }

    // Create new Background Map/Sprite
    let background_string = &*background_mgr.cur_map.to_string();
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(-25.0, -40.0, 0.0),
            texture: asset_server.load(background_string),
            ..default()
        })
        .insert(IsBackground);
}
