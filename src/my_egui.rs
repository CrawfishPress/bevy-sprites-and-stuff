/*

*/

use bevy::prelude::*;
#[allow(unused_imports)]
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::Color32;
use crate::{GuiData, GameData};

pub fn do_ui_setup(mut egui_context: ResMut<EguiContext>,
                   mut random_data: ResMut<GuiData>,
                   mut move_active: ResMut<GameData>,
) {
    let my_frame = egui::containers::Frame {
        outer_margin: Default::default(),
        inner_margin: egui::style::Margin { left: 10.0, right: 10.0, top: 10.0, bottom: 10.0 },
        rounding: Default::default(),
        // shadow: epaint::Shadow { extrusion: 1.0, color: Color32::YELLOW },
        shadow: Default::default(),
        fill: Color32::DARK_GREEN,
        // stroke: egui::Stroke::new(2.0, Color32::BLACK),  // Border
        stroke: Default::default(),
    };

    egui::TopBottomPanel::top("top panel?").min_height(100.0)
        .frame(my_frame)
        // .resizable(true)  // Only works if there's a resizable element inside?
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("This is a Header that does nothing useful");
            ui.label("This is just a Label");
            if ui.button("Click the magic-button to pause").clicked() {
                random_data.my_value += 1;
                move_active.is_paused = !move_active.is_paused;
            }
            if ui.button("RESET THINGS").clicked() {
                move_active.game_status = move_active.game_status.cycle();
            }
    });
}
