/*
I've just moved Resource/Enum/Structs here, too, still haven't decided if
this is a good idea, or not.
*/

use std::fmt;
use std::fmt::Formatter;
use bevy::prelude::*;

pub const WIDTH: f32 = 1870.0;
pub const HEIGHT: f32 = 950.0;
pub const BACKGROUND: Color = Color::rgb(0.50, 0.50, 1.0);

pub const BLOCK_SIZE: f32 = 150.0;
pub const PADDLE_COLOR: Color = Color::rgba(0.3, 0.1, 0.9, 0.9);

// GUI Resource
#[derive(Debug, Default)]
pub struct GuiData {
    pub some_name: String,
    pub my_value: i32,
    pub my_other_value: f64,
}

// Load-Screen Background

pub const LOAD_SCREEN_BACKGROUND: &str = "unsplash-MS7KD9Ti7FQ.png";

// Action-Screen Background stuff

// Example of creating an Enum that maps to Strings (but *not* str)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BackgroundAction {
    Map1,
    Map2,
}

// See Lessons - this has to be converted to *str*, for use by asset-loader
impl fmt::Display for BackgroundAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BackgroundAction::Map1 => write!(f, "unsplash-Ai2TRdvI6gM.png"),
            BackgroundAction::Map2 => write!(f, "unsplash-cTeQyMstoDI.png"),
        }
    }
}

// Adding a cycler-function (see lessons.md), so I don't have to say things like:
// if map1, set target to map2 else set target to map1...
impl BackgroundAction {
    pub fn toggle(&self) -> Self {
        match *self {
            BackgroundAction::Map1 => BackgroundAction::Map2,
            BackgroundAction::Map2 => BackgroundAction::Map1,
        }
    }
}

// Resource for Action-Screen Background
pub struct BackgroundActionMap {
    pub cur_map: BackgroundAction,
    pub cursor_over_map: bool,
    pub cursor_on_map: Vec2,
}

impl Default for BackgroundActionMap {
    fn default() -> Self {
        BackgroundActionMap {
            cur_map: BackgroundAction::Map1,
            cursor_over_map: false,
            cursor_on_map: Vec2 { x: 0.0, y: 0.0 }
        }
    }
}

// There should only be one Sprite with this marker, or all hell might break loose.
#[derive(Component)]
pub struct IsBackground;

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

// General Game status-data

// Resource
pub struct GameData {
    pub action_status: ActionState,
    pub is_paused: bool,
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            action_status: ActionState::Running,
            is_paused: true,
        }
    }
}

// Cycled by a GUI Reset button
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionState {
    Running,
    Reset,
}

// Adding a cycler-function to simplify changing states
impl ActionState {
    pub fn cycle(&self) -> Self {
        match *self {
            ActionState::Running => ActionState::Reset,
            ActionState::Reset => ActionState::Running,
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
