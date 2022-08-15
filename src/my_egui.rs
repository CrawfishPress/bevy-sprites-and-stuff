/*
Experiments in learning egui.
https://docs.rs/egui/latest/egui/
*/

use bevy::prelude::*;
use bevy_egui::*;
use egui::*;
use crate::{GuiData, GameData, BackgroundMap};

pub fn do_ui_setup(the_map: Res<BackgroundMap>,
                   mut egui_context: ResMut<EguiContext>,
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

    let intro_text = RichText::new("Mouse-drag the starfish/hexagon, and arrow-keys move the rectangle")
        .color(Color32::WHITE).font(FontId::proportional(20.0));

    let pause_string: String;
    if move_active.is_paused {
        pause_string = "Click the magic-button to start".parse().unwrap();
    } else {
        pause_string = "Click the magic-button to pause".parse().unwrap();
    }
    let pause_btn_txt = RichText::new(pause_string)
        .color(Color32::WHITE).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(25.0);

    let coords_string: String;
    if the_map.cursor_over_map {
        coords_string = format!("Map Coords: [{:.0}:{:.0}]", the_map.cursor_on_map.x, the_map.cursor_on_map.y);
    } else {
        coords_string = format!("Off-Map Coords: [{:.0}:{:.0}]", the_map.cursor_on_map.x, the_map.cursor_on_map.y);
    }
    let coords_txt = RichText::new(coords_string)
        .color(Color32::BLACK).background_color(Color32::GRAY).font(FontId::proportional(20.0)).size(25.0);

    let reset_btn_txt = RichText::new("Reset Things")
        .color(Color32::WHITE).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(25.0);

    let wasm_browser_txt = RichText::new("BROWSER? Hit <ctrl>- a few times to shrink, while we work on it.")
        .color(Color32::WHITE).font(FontId::proportional(40.0));

    TopBottomPanel::top("top panel?").min_height(80.0)
        .frame(my_style_frame)
        // .resizable(true)  // Only works if there's a resizable element inside?
        .show(egui_context.ctx_mut(), |ui| {
            ui.style_mut().spacing.item_spacing = egui::Vec2 {x: 30.0, y: 0.0};

            // Tab-panel
            let tab_load_screen = RichText::new("Loading Screen")
                .color(Color32::WHITE).background_color(Color32::BLUE).font(FontId::proportional(20.0)).size(25.0);
            let tab_action_screen = RichText::new("Action Screen")
                .color(Color32::WHITE).background_color(Color32::BLUE).font(FontId::proportional(20.0)).size(25.0);
            ui.horizontal(|ui| {
                if ui.button(tab_load_screen).clicked() {
                    // move_active.game_status = move_active.game_status.cycle();
                }
                if ui.button(tab_action_screen).clicked() {
                    // move_active.is_paused = !move_active.is_paused;
                }
                ui.label(wasm_browser_txt);
            });

            ui.horizontal(|ui| {
                if ui.button(reset_btn_txt).clicked() {
                    move_active.game_status = move_active.game_status.cycle();
                }
                if ui.button(pause_btn_txt).clicked() {
                    random_data.my_value += 1;
                    move_active.is_paused = !move_active.is_paused;
                }
            });

            ui.horizontal(|ui| {
                // ui.heading("This is a Header (Label) that does nothing useful");
                ui.add_sized([200.0, 40.0], Label::new(coords_txt));
                ui.add_sized([420.0, 40.0], Label::new(intro_text));
            });

    });
}
