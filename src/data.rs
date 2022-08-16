/*
I've just moved Resource/Enum/Structs here, too, still haven't decided if
this is a good idea, or not.

ActionState: note that two of the three States are meant to be transient:
any state other than Running, will do some setup and move to another State.

BackgroundAction.toggle():
Added a cycler-function (see lessons.md), so I don't have to say things like:
    if map1, set target to map2 else set target to map1...
Note: toggle() should only be called on Action-Screen, so MapLoad1 shouldn't be
used - but match() wants to be exhaustive...

ScreenManager/CurScreen:

Originally it was separate (like it is now), then I moved it to Gamedata,
then I moved it *back* out - when I realized I was doing an is_changed()
test on CurScreen, and I didn't want the system/function, firing on
any other changes. Still not sure it should be separate, but for now, there it is.

*/

use std::fmt;
use std::fmt::Formatter;
use bevy::prelude::*;

pub const WIDTH: f32 = 1870.0;
pub const HEIGHT: f32 = 950.0;
pub const BACKGROUND: Color = Color::rgb(0.50, 0.50, 1.0);

pub const BLOCK_SIZE: f32 = 150.0;
pub const PADDLE_COLOR: Color = Color::rgba(0.3, 0.1, 0.9, 0.9);

// Misc *******************************************************

// GUI Resource
#[derive(Debug, Default)]
pub struct GuiData {
    pub some_name: String,
    pub my_value: i32,
    pub my_other_value: f64,
}

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

// Resource
#[derive(Debug)]
pub struct DragPoint {
    pub left_point: Vec2,
    pub right_point: Vec2,
}

impl Default for DragPoint {
    fn default() -> Self {
        DragPoint {
            left_point: Vec2 { x: -300.0, y: -200.0 },
            right_point: Vec2 { x: 500.0, y: -200.0 },
        }
    }
}

#[derive(Component, Debug)]
pub enum HoverCraft {
    LeftPoint,
    RightPoint,
}

#[derive(Component)]
pub struct OneMover;

#[derive(Component)]
pub struct IsMousing {
    pub is_dragging: bool,
    pub is_hovering: bool,
}

// Screen Background stuff *********************************************

// Example of creating an Enum that maps to Strings (but *not* str)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BackgroundBitmaps {
    MapLoad1,
    MapAction1,
    MapAction2,
}

// See Lessons - this has to be converted to *str*, for use by asset-loader
impl fmt::Display for BackgroundBitmaps {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BackgroundBitmaps::MapLoad1 => write!(f, "unsplash-MS7KD9Ti7FQ.png"),
            BackgroundBitmaps::MapAction1 => write!(f, "unsplash-Ai2TRdvI6gM.png"),
            BackgroundBitmaps::MapAction2 => write!(f, "unsplash-cTeQyMstoDI.png"),
        }
    }
}

impl BackgroundBitmaps {
    pub fn toggle(&self) -> Self {
        match *self {
            BackgroundBitmaps::MapLoad1 => BackgroundBitmaps::MapLoad1,
            BackgroundBitmaps::MapAction1 => BackgroundBitmaps::MapAction2,
            BackgroundBitmaps::MapAction2 => BackgroundBitmaps::MapAction1,
        }
    }
}

// Resource for Background of both LoadScreen and ActionScreen, now
pub struct BackgroundMapVisible {
    pub cur_map: BackgroundBitmaps,
    pub cursor_over_map: bool,
    pub cursor_on_map: Vec2,
}

impl Default for BackgroundMapVisible {
    fn default() -> Self {
        BackgroundMapVisible {
            cur_map: BackgroundBitmaps::MapAction1,
            cursor_over_map: false,
            cursor_on_map: Vec2 { x: 0.0, y: 0.0 }
        }
    }
}

// There should only be one Sprite with this marker, or all hell might break loose.
#[derive(Component)]
pub struct IsBackground;

// General Game status-data ************************************

// Resource
pub struct GameData {
    pub action_status: ActionState,
    pub is_paused: bool,
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            action_status: ActionState::OffScreen,
            is_paused: true,
        }
    }
}

// Cycled by a GUI Reset button
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionState {
    OffScreen,
    Loading,
    Running,
    Resetting,
}

// Adding a cycler-function to simplify changing states
impl ActionState {
    pub fn cycle(&self) -> Self {
        match *self {
            ActionState::OffScreen => ActionState::OffScreen,
            ActionState::Loading => ActionState::Loading,
            ActionState::Running => ActionState::Resetting,
            ActionState::Resetting => ActionState::Running,
        }
    }
}

// Screen-state Resource, pretending to be a Screen manager
pub struct ScreenManager {
    pub current_screen: CurScreen,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CurScreen {
    LoadScreen,
    ActionScreen,
}

#[derive(Component)]
pub struct IsActionScreen;
