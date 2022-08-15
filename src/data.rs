/*
I've just moved Resource/Enum/Structs here, too, haven't decided if
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

// Background stuff

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
    pub fn toggle(&self) -> Self {
        match *self {
            OneBackground::Map1 => OneBackground::Map2,
            OneBackground::Map2 => OneBackground::Map1,
        }
    }
}

// Resource for Background
pub struct BackgroundMap {
    pub cur_map: OneBackground,
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
    pub game_status: GameState,
    pub is_paused: bool,
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            game_status: GameState::Running,
            is_paused: true,
        }
    }
}

// Cycled by a GUI Reset button
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    Running,
    Reset,
}

// Adding a cycler-function to simplify changing states
impl GameState {
    pub fn cycle(&self) -> Self {
        match *self {
            GameState::Running => GameState::Reset,
            GameState::Reset => GameState::Running,
        }
    }
}
