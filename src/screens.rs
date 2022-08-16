/*
Simulating a Screen-manager, with Buttons, that are simulating a Tab-panel...

Much changes went on in the process of making separate Screens/levels.

*/

use bevy::prelude::*;
use crate::*;


pub fn do_change_screen(mut commands: Commands,
                        screen_mgr: Res<ScreenManager>,
                        mut game_status: ResMut<GameData>,
                        mut background_mgr: ResMut<BackgroundMapVisible>,
){
    if ! screen_mgr.is_changed() { return; }
    // println!("Screen Status Change: {:?}", screen_mgr.current_screen);

    if screen_mgr.current_screen == CurScreen::LoadScreen {
        background_mgr.cur_map = BackgroundBitmaps::MapLoad1;
        game_status.action_status = ActionState::OffScreen;
        game_status.is_paused = true;
    } else
    if screen_mgr.current_screen == CurScreen::ActionScreen {
        background_mgr.cur_map = BackgroundBitmaps::MapAction1;
        game_status.action_status = ActionState::Loading;

        // TODO: need to rethink/refactor this, kinda kludgy. Need a more general
        // TODO: way of resetting all game-data.
        commands.remove_resource::<DragPoint>();
        commands.insert_resource(DragPoint::default());
    }
}
