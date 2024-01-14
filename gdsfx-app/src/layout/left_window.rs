use eframe::egui::{Context, SidePanel, ScrollArea};

use crate::{layout, tabs, app_state::AppState, library_manager::LibraryManager};

pub fn render(ctx: &Context, app_state: &mut AppState, library_manager: &LibraryManager) {
    SidePanel::left("left_panel")
        .min_width(layout::MIN_LIBRARY_WIDTH)
        .max_width(layout::RIGHT_PANEL_WIDTH)
        .default_width(layout::DEFAULT_LIBRARY_WIDTH)
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                use tabs::*;
                
                match app_state.selected_tab {
                    Tab::Library => library::render(ui, app_state, library_manager),
                    Tab::Favorites => favorites::render(ui, app_state, library_manager),
                    Tab::Tools => tools::render(ui, ctx),
                    Tab::Settings => settings::render(ui, app_state),
                    Tab::Stats => stats::render(ui, library_manager),
                    Tab::Credits => credits::render(ui, library_manager),
                }
            });
        });
}
