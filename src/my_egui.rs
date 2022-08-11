/*

So it turns out,
    pub fn do_ui_setup(mut egui_context: ResMut<EguiContext>,
                       mut random_data: ResMut<GuiData>,) {
        let age: i32 = 0;

doesn't work in Immediate-Mode. Wups. IM means every frame,
the function refreshes the screen, and constructs the UI anew.
Any data has to be stored somewhere else, like a Resource.
*/

use bevy::prelude::*;
#[allow(unused_imports)]
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::Color32;
use crate::SpritesMovable;

// GUI Resource
#[derive(Debug, Default)]
pub struct GuiData {
    pub some_name: String,
    pub my_value: i32,
    pub my_other_value: f64,
}

pub fn do_ui_setup(mut egui_context: ResMut<EguiContext>,
                   mut random_data: ResMut<GuiData>,
                   mut move_active: ResMut<SpritesMovable>,
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

    egui::TopBottomPanel::top("top panel?").default_height(300.0)
        .frame(my_frame)
        // .resizable(true)  // Only works if there's a resizable element inside?
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("This is a Header that does nothing useful");
            ui.label("This is a Label");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut random_data.some_name);
            });
            if ui.button("Click the magic-button").clicked() {
                random_data.my_value += 1;
                move_active.is_active = !move_active.is_active;
            }
            ui.add(egui::Slider::new(&mut random_data.my_other_value, 0.0..=100.0).text("My value"));
    });
}

pub fn do_ui_read(_random_data: ResMut<GuiData>,
) {
    // println!("random data: {:?}", random_data);
}

