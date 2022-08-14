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
        inner_margin: egui::style::Margin { left: 20.0, right: 20.0, top: 10.0, bottom: 10.0 },
        rounding: Default::default(),
        // shadow: epaint::Shadow { extrusion: 1.0, color: Color32::YELLOW },
        shadow: Default::default(),
        fill: Color32::DARK_GREEN,
        // stroke: egui::Stroke::new(2.0, Color32::BLACK),  // Border
        stroke: Default::default(),
    };

    let big_text_lbl_1 = RichText::new("Mouse-drag the starfish/hexagon, and arrow-keys move the rectangle")
        .color(Color32::WHITE).font(FontId::proportional(20.0));

    let pause_string: String;
    if move_active.is_paused {
        pause_string = "Click the magic-button to start".parse().unwrap();
    } else {
        pause_string = "Click the magic-button to pause".parse().unwrap();
    }

    let btn_text_1 = RichText::new(pause_string)
        .color(Color32::WHITE).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(25.0);

    let btn_text_3 = RichText::new("Reset Things")
        .color(Color32::WHITE).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(25.0);

    let big_text_lbl_11 = RichText::new("BROWSER? Hit <ctrl>- a few times to shrink, while we work on it.")
        .color(Color32::WHITE).font(FontId::proportional(40.0));

    TopBottomPanel::top("top panel?").min_height(80.0)
        .frame(my_style_frame)
        // .resizable(true)  // Only works if there's a resizable element inside?
        .show(egui_context.ctx_mut(), |ui| {
            ui.style_mut().spacing.item_spacing = egui::Vec2 {x: 30.0, y: 0.0};

            // Tab-panel
            let tab_text_1 = RichText::new("Loading Screen")
                .color(Color32::WHITE).background_color(Color32::BLUE).font(FontId::proportional(20.0)).size(25.0);
            let tab_text_2 = RichText::new("Action Screen")
                .color(Color32::WHITE).background_color(Color32::BLUE).font(FontId::proportional(20.0)).size(25.0);
            ui.horizontal(|ui| {
                if ui.button(tab_text_1).clicked() {
                    // move_active.game_status = move_active.game_status.cycle();
                }
                if ui.button(tab_text_2).clicked() {
                    random_data.my_value += 1;
                    // move_active.is_paused = !move_active.is_paused;
                }
                ui.label(big_text_lbl_11);
            });

            ui.horizontal(|ui| {
                // ui.heading("This is a Header (Label) that does nothing useful");
                ui.add_sized([420.0, 40.0], Label::new(big_text_lbl_1));
            });

            ui.horizontal(|ui| {
                if ui.button(btn_text_3).clicked() {
                    move_active.game_status = move_active.game_status.cycle();
                }
                if ui.button(btn_text_1).clicked() {
                    random_data.my_value += 1;
                    move_active.is_paused = !move_active.is_paused;
                }
            });
    });
}
