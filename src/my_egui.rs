/*
Experiments in learning egui.
https://docs.rs/egui/latest/egui/
*/

use bevy::prelude::*;
use bevy_egui::*;
use egui::*;
use crate::*;

pub fn do_display_ui(the_map: Res<BackgroundMapVisible>,
                     mut screen_mgr: ResMut<ScreenManager>,
                     mut egui_context: ResMut<EguiContext>,
                     mut random_data: ResMut<GuiData>,
                     mut game_status: ResMut<GameData>,
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

    // Tabbed-Panel Buttons: Load/Action Screens
    let mut tab_load_screen_txt = RichText::new("Loading Screen")
        .color(Color32::WHITE).background_color(Color32::BLUE).font(FontId::proportional(20.0)).size(25.0);
    let mut tab_action_screen_txt = RichText::new("Action Screen")
        .color(Color32::WHITE).background_color(Color32::BLUE).font(FontId::proportional(20.0)).size(25.0);

    // The Active Screen/Tab will be highlighted somehow - color-change is all I've been able to figure out, so far.
    if screen_mgr.current_screen == CurScreen::LoadScreen {
        tab_load_screen_txt = tab_load_screen_txt.color(Color32::WHITE);
        tab_action_screen_txt = tab_action_screen_txt.color(Color32::GRAY);
    } else {
        tab_load_screen_txt = tab_load_screen_txt.color(Color32::GRAY);
        tab_action_screen_txt = tab_action_screen_txt.color(Color32::WHITE);
    }

    // Game Buttons: Pause/Reset
    let pause_string: String;
    if game_status.is_paused {
        pause_string = "Click the magic-button to start".parse().unwrap();
    } else {
        pause_string = "Click the magic-button to pause".parse().unwrap();
    }
    let pause_btn_txt = RichText::new(pause_string)
        .color(Color32::WHITE).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(25.0);
    let reset_btn_txt = RichText::new("Reset Things")
        .color(Color32::WHITE).background_color(Color32::BLACK).font(FontId::proportional(20.0)).size(25.0);

    // Information-Labels
    let intro_text = RichText::new("Mouse-drag the starfish/hexagon, and arrow-keys move the rectangle")
        .color(Color32::WHITE).font(FontId::proportional(20.0));
    let wasm_browser_txt = RichText::new("BROWSER? Hit <ctrl>- a few times to shrink, while we work on it.")
        .color(Color32::WHITE).font(FontId::proportional(40.0));

    let coords_string: String;
    if the_map.cursor_over_map {
        coords_string = format!("Map Coords: [{:.0}:{:.0}]", the_map.cursor_on_map.x, the_map.cursor_on_map.y);
    } else {
        coords_string = format!("Off-Map Coords: [{:.0}:{:.0}]", the_map.cursor_on_map.x, the_map.cursor_on_map.y);
    }
    let coords_txt = RichText::new(coords_string)
        .color(Color32::BLACK).background_color(Color32::GRAY).font(FontId::proportional(20.0)).size(25.0);


    // Finally, a displayable Widget - Top Panel
    TopBottomPanel::top("top panel?").min_height(GUI_PANEL_HEIGHT)
        .frame(my_style_frame)
        // .resizable(true)  // Only works if there's a resizable element inside?
        .show(egui_context.ctx_mut(), |ui| {
            ui.style_mut().spacing.item_spacing = egui::Vec2 {x: 30.0, y: 0.0};

            // Screen-manager tabs
            ui.horizontal(|ui| {
                // ui.style_mut().visuals.window_shadow;
                // ui.visuals_mut().dark_mode = true;
                let load_some_button = egui::Button::new(tab_load_screen_txt);
                if ui.add(load_some_button).clicked() {
                    // Checking existing Screen, to avoid changing it, and firing an is_changed()
                    if screen_mgr.current_screen != CurScreen::LoadScreen {
                        screen_mgr.current_screen = CurScreen::LoadScreen;
                    }
                }
                let action_some_button = egui::Button::new(tab_action_screen_txt);
                if ui.add(action_some_button).clicked() {
                    if screen_mgr.current_screen != CurScreen::ActionScreen {
                        screen_mgr.current_screen = CurScreen::ActionScreen;
                    }
                }
                ui.label(wasm_browser_txt);
            });

            // Action-Screen UI
            // This is kinda kludgy, but then again, I've found everything about egui to be kludgy. :)
            if screen_mgr.current_screen == CurScreen::ActionScreen {
                let action_pause_button = egui::Button::new(pause_btn_txt);
                let action_reset_button = egui::Button::new(reset_btn_txt);
                ui.horizontal(|ui| {
                    if ui.add(action_reset_button).clicked() {
                        game_status.action_status = game_status.action_status.cycle();
                    }
                    if ui.add(action_pause_button).clicked() {
                        random_data.my_value += 1;
                        game_status.is_paused = !game_status.is_paused;
                    }
                });

                // Assorted Information Labels
                ui.horizontal(|ui| {
                    ui.add_sized([200.0, 40.0], Label::new(coords_txt));
                    ui.add_sized([420.0, 40.0], Label::new(intro_text));
                });
            }
    });
}
