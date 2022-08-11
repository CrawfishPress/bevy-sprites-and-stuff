use bevy::prelude::*;
use bevy::ecs::{archetype::Archetypes, component::Components};
use crate::{DragPoint};

// Well, it *used* to be a car... Now it's a crab
#[derive(Component)]
pub enum CrabDirection {
    Left,
    Right,
}

// For a given Sprite, this determines if it can move, when arrow-keys are pressed.
// Not actually used any more, originally the spacebar would toggle.
#[derive(Component, Debug)]
pub struct KeyMover {
    pub is_movable: bool
}

// Resource - basically pauses all sprite-movement
pub struct SpritesMovable {
    pub is_active: bool
}

/*
The first pass, this function moved the Sprite (crab) between points A and B. I decided
to make it move between two draggable-Sprites. This resulted in two queries, both of
which used Transforms (which stores Sprite position). See lessons.md for why that's not good.
The original points A/B were also linear, so only had to change X coordinate - now have
to take a vector to the target. And as crazy as it sounds, I may have to *normalize* that
vector, and multiply by a chosen magnitude, before applying to the crab-Sprite. No, wait...
Bevy has LERPs! Note that lerp-ing, effectly adds easing in/out (because I'm basing it on
distance to target, as the sprite gets closer and closer, so it moves less), whereas just
moving between A/B (by adding X pixels), was a constant speed the whole way. I may want
to rethink the lerp. OTOH, it does sort of look crab-like...
*/
pub fn do_sprite_auto_move(drag_points: Res<DragPoint>,
                           move_active: Res<SpritesMovable>,
                           mut sprite_query: Query<(&mut Transform,
                                                &mut CrabDirection,)>,
) {
    if !move_active.is_active { return; }  // Spacebar pauses movement

    // TODO: there's now only one auto-moving sprite, the Crab - I could change these loops to single.
    for (mut sprite_pos, moving_dir) in &mut sprite_query {
        let target_pos;  // Why is this here? Because when I put it inside the match(), it goes out of scope!

        match *moving_dir {
            CrabDirection::Left => target_pos = drag_points.left_point,
            CrabDirection::Right => target_pos = drag_points.right_point,
        }

        let sprite_vec2 = sprite_pos.translation.truncate();
        let lerp_to_target = sprite_vec2.lerp(target_pos, 0.02);
        sprite_pos.translation = lerp_to_target.extend(sprite_pos.translation.z); // Upscale a Vec2 to Vec3
    }
}

/*
Checks if the ~car~, er, Crab has reached its target, changes direction. Also changes SpriteTexture,
to match left/right movement. Originally when this moved between points A and B, it was simpler
to check if it had reached the Bounds.  Now that it moves between two Draggable Sprites,
it's checking for being "close enough". I mean, those Sprites could be anywhere...
*/
// TODO: if there's only one auto-moving sprite, I can make the TextureAtlas not an Option - worth it?
pub fn do_sprite_move_check(drag_points: Res<DragPoint>,
                            mut sprite_position: Query<(&Transform,
                                                     &mut CrabDirection,
                                                     Option<&mut TextureAtlasSprite>)>,
) {
    for (sprite_pos, mut moving_dir, tas) in &mut sprite_position {
        let sprite_vec2 = sprite_pos.translation.truncate();

        match *moving_dir {
            CrabDirection::Left => {
                let target_dist = sprite_vec2.distance(drag_points.left_point);
                if target_dist < 50.0 {
                    *moving_dir = CrabDirection::Right;
                    if tas.is_some() { tas.unwrap().index = 1; }
                }
            },
            CrabDirection::Right => {
                let target_dist = sprite_vec2.distance(drag_points.right_point);
                if target_dist < 50.0 {
                    *moving_dir = CrabDirection::Left;
                    if tas.is_some() { tas.unwrap().index = 0; }
                }
            },
        }
    }
}

pub fn do_movement_input(keyboard_input: Res<Input<KeyCode>>,
                         move_active: Res<SpritesMovable>,
                         mut tunnel_pos: Query<(&mut Transform,
                                                &mut KeyMover)>,
) {
    if !move_active.is_active { return; }

    // TODO: for some reason, LControl not being picked up - is Linux eating it?
    if keyboard_input.any_pressed([KeyCode::LControl, KeyCode::RControl, KeyCode::LAlt, KeyCode::RAlt]) {
        println!("*** keys pressed: {:?}", keyboard_input);
        return;}

    for (mut tunnel_dir, tunnel_move) in &mut tunnel_pos {
        if tunnel_move.is_movable == false {
            continue;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            tunnel_dir.translation.x -= 7.5
        } else if keyboard_input.pressed(KeyCode::Down) {
            tunnel_dir.translation.y -= 7.5
        } else if keyboard_input.pressed(KeyCode::Up) {
            tunnel_dir.translation.y += 7.5
        } else if keyboard_input.pressed(KeyCode::Right) {
            tunnel_dir.translation.x += 7.5
        }
    }
}

/*
From the cheat-book, a way of listing all of the Resources that
Bevy adds, based on Plugins used. Runs stand-alone.

    .add_startup_system(print_resources)

*/
#[allow(dead_code)]
pub fn print_resources(archetypes: &Archetypes, components: &Components) {
    let mut some_res: Vec<_> = archetypes
        .resource()
        .components()
        .map(|id| components.get_info(id).unwrap())
        .map(|info| info.name())
        .collect();

    // sort list alphabetically
    some_res.sort();
    some_res.iter().for_each(|name| println!("{}", name));
}
