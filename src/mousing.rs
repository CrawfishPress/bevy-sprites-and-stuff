/*
Some Sprites are mouse-reactive:
 - currently they can be dragged by the mouse when the LMB is held down.
*/

use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::camera;
use crate::{DragPoint, HoverCraft, IsMousing};

// fn print_type_of<T>(_: &T) {print!("{}", std::any::type_name::<T>())} // Unstable.

// Used to identify the main camera - granted, there's only one at the moment...
#[derive(Component)]
pub struct MainCamera;

/*
check_cursor_for_drag(): when the LMB is pressed, does:
 - gets the camera location
 - gets the Window data
   - using magic, maps the camera-location to Window, to determine world-coordinates
 - gets any Draggable Sprites
 - checks if the adjusted mouse-coordinates are more-or-less on top of a Sprite
   - while the mouse is moving, update the Sprite coordinates to follow the mouse
 - when LMB released, clears all Draggable Sprites
*/
pub fn check_cursor_for_drag(
    windows: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut drag_points: ResMut<DragPoint>,
    my_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut any_hovercraft: Query<(Entity, &HoverCraft, &mut IsMousing, &mut Transform)>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        // println!("*** LMB was just released! Clear all Hovercraft!");
        for (_, _, mut mouse_action, _) in  any_hovercraft.iter_mut() {
            mouse_action.is_dragging = false;
        }
        return;
    }

    if ! mouse_button_input.pressed(MouseButton::Left) {
        // println!("*** LMB is NOT being held down. We're done here.");
        return;
    }
    // println!("*** LMB is being held down. There's work to be done.");

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = my_camera.single();
    let some_window = windows.get_primary().unwrap();  // Here's hoping there's always a window...

    // get the window that the camera is displaying to (or the primary window)
    // Should always be at least one camera on this window, but hey, who knows?
    //let some_window = if let RenderTarget::Window(id) = camera.target {
    //    windows.get(id).unwrap()
    //} else {
    //    windows.get_primary().unwrap()
    //};

    let maybe_camera_pos = parse_camera_loc(camera, camera_transform, some_window);
    if maybe_camera_pos.is_none() {
        // println!("*** no camera_pos? I did *not* see that coming.");
        return;
    }
    let camera_pos = maybe_camera_pos.unwrap();
    drag_some_sprite(camera_pos, &mut drag_points, &mut any_hovercraft);
}

/*
drag_some_sprite(): when LMB is held down,
 - checks all Hovercraft-type Sprites for mouse-coordinates over them
 - for any Sprite with either (mouse over it), or `is_dragging` flag set:
   - set the Sprite's coordinates to match the mouse (dragging it)
   - copy the Sprite's coordinates to a DragPoint Resource, for use by other sprites
 The `is_dragging` flag is checked, because the mouse can "out-run" the sprite, or get
 outside of its bounding-box faster than the sprite can move. I want the sprite to keep
 dragging, until the LMB is released.
*/
// TODO: currently only one sprite at a time can be dragged, don't need the loop any more, could remove it.
// TODO: correction - only one sprite *should* be dragged... If you mouse-drag one over the other, well...
fn drag_some_sprite(camera_pos: Vec2,
                    drag_points: &mut ResMut<DragPoint>,
                    any_hovercraft: &mut Query<(Entity, &HoverCraft, &mut IsMousing, &mut Transform)>,
) {
    for (_entity_id, hover_side, mut hovering, mut some_sprite_pos) in any_hovercraft.iter_mut() {
        if (check_mouse_over_sprite(&some_sprite_pos, camera_pos) == true) ||
           (hovering.is_dragging == true)
        {
            hovering.is_dragging = true;
            some_sprite_pos.translation = camera_pos.extend(some_sprite_pos.translation.z); // Upscaling Vec2 to Vec3

            match hover_side {
                HoverCraft::LeftPoint => {
                    drag_points.left_point = camera_pos;
                }
                HoverCraft::RightPoint => {
                    drag_points.right_point = camera_pos;
                }
            }
        }
    }
}

// Very kludgy - eventually I plan to check the sprite's bounding-box.
fn check_mouse_over_sprite(some_pos: &Transform, camera_pos: Vec2
) -> bool {
    let x_diff = (some_pos.translation.x - camera_pos.x).abs();
    let y_diff = (some_pos.translation.y - camera_pos.y).abs();

    return if (x_diff < 100.0) && (y_diff < 100.0) {
        true
    } else
        { false }
}

// check if the cursor is inside the window, and get its position.
// I don't actually understand much of this code, but hey, it seems to work - copy/paste FTW!
fn parse_camera_loc(camera: &camera::Camera,
                    camera_transform: &GlobalTransform,
                    some_window: &Window) -> Option<Vec2>
{
    if some_window.cursor_position().is_none() {return None};
    let screen_pos = some_window.cursor_position().unwrap();

    // get the size of the window
    let window_size = Vec2::new(some_window.width() as f32, some_window.height() as f32);

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();

    // println!("\nWorld coords: {}/{}", world_pos.x, world_pos.y);
    return Some(world_pos);
}
