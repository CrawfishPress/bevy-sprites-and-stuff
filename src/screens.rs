/*
Simulating a Screen-manager, with Buttons, that are simulating a Tab-panel...

Much changes went on in the process of making separate Screens/levels.

data::ScreenManager/CurScreen:

Originally it was separate (like it is now), then I moved it to Gamedata,
then I moved it *back* out - when I realized I was doing an is_changed()
test on CurScreen, and I didn't want the system/function, firing on
any other changes. Still not sure it should be separate, but for now, there it is.

*/

use bevy::prelude::*;
use crate::{GameData, ScreenManager};

pub fn do_change_screen(game_status: ResMut<GameData>,
                        screen_mgr: Res<ScreenManager>, )
{
    if screen_mgr.is_changed() {
        println!("Screen Status Change: {:?}", screen_mgr.current_screen)
    }
}