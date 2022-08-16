/*
Simulating a Screen-manager, with Buttons, that are simulating a Tab-panel...

Much changes went on in the process of making separate Screens/levels.

*/

use bevy::prelude::*;
use crate::{BackgroundBitmaps, BackgroundMapVisible, CurScreen, GameData, ScreenManager};


pub fn do_change_screen_background(screen_mgr: Res<ScreenManager>,
                                   mut background_mgr: ResMut<BackgroundMapVisible>,
){
    if ! screen_mgr.is_changed() { return; }
    println!("Screen Status Change: {:?}", screen_mgr.current_screen);

    if screen_mgr.current_screen == CurScreen::LoadScreen {
        background_mgr.cur_map = BackgroundBitmaps::MapLoad1;
    } else if screen_mgr.current_screen == CurScreen::ActionScreen {
        background_mgr.cur_map = BackgroundBitmaps::MapAction1;
    }
}
