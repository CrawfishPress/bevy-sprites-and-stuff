/*
Experiments in learning egui.
https://docs.rs/egui/latest/egui/
*/

use bevy::prelude::*;
use bevy_egui::*;
use egui::*;
use crate::{GuiData, GameData};

pub fn do_ui_setup(mut egui_context: ResMut<EguiContext>,
                   mut random_data: ResMut<GuiData>,
                   mut move_active: ResMut<GameData>,
) {
    let my_style_frame = containers::Frame {
        outer_margin: Default::default(),
        inner_margin: egui::style::Margin { left: 10.0, right: 10.0, top: 10.0, bottom: 10.0 },
        rounding: Default::default(),
        // shadow: epaint::Shadow { extrusion: 1.0, color: Color32::YELLOW },
        shadow: Default::default(),
        fill: Color32::DARK_GREEN,
        // stroke: egui::Stroke::new(2.0, Color32::BLACK),  // Border
        stroke: Default::default(),
    };

    let big_text_lbl_1 = RichText::new("This is a Label that is both sized, and Font-color is set.")
        .color(Color32::WHITE).font(FontId::proportional(20.0));

    let pause_string: String;
    if move_active.is_paused {
        pause_string = "Click the magic-button to start".parse().unwrap();
    } else {
        pause_string = "Click the magic-button to pause".parse().unwrap();
    }

    let big_text_btn_1 = RichText::new(pause_string)
        .color(Color32::WHITE).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(25.0);

    let big_text_btn_2 = RichText::new("Reset Things")
        .color(Color32::WHITE).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(25.0);

    TopBottomPanel::top("top panel?").min_height(80.0)
        .frame(my_style_frame)
        // .resizable(true)  // Only works if there's a resizable element inside?
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("This is a Header (Label) that does nothing useful");
            ui.add_sized([420.0, 40.0], Label::new(big_text_lbl_1));

            ui.horizontal(|ui| {
                if ui.button(big_text_btn_2).clicked() {
                    move_active.game_status = move_active.game_status.cycle();
                }
                if ui.button(big_text_btn_1).clicked() {
                    random_data.my_value += 1;
                    move_active.is_paused = !move_active.is_paused;
                }
            })
    });
}
