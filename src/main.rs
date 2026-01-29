//! Phoenix Engine - A 2D Action-Adventure Game Engine
//! 
//! This engine provides a complete framework for building 2D action-adventure games
//! with Python scripting support for configuration and gameplay logic.

mod engine;
mod editor;
mod scripting;
mod game;

use eframe::egui;
use tracing_subscriber;

fn main() -> eframe::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    tracing::info!("🔥 Phoenix Engine v0.1.0 Starting...");

    // Configure the native window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 900.0])
            .with_min_inner_size([1024.0, 768.0])
            .with_title("Phoenix Engine - 2D Action Adventure")
            .with_icon(eframe::icon_data::from_png_bytes(
                include_bytes!("../assets/icon.png")
            ).unwrap_or_default()),
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Phoenix Engine",
        options,
        Box::new(|cc| {
            // Configure custom fonts and styles
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(editor::EditorApp::new(cc)))
        }),
    )
}

/// Setup custom fonts and visual styles for the editor
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // You can add custom fonts here
    // fonts.font_data.insert(
    //     "custom_font".to_owned(),
    //     egui::FontData::from_static(include_bytes!("../assets/fonts/custom.ttf")),
    // );

    ctx.set_fonts(fonts);

    // Set up dark theme with custom colors
    let mut style = (*ctx.style()).clone();
    
    // Phoenix Engine color scheme - Dark theme with orange accents
    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(egui::Color32::from_rgb(220, 220, 220));
    style.visuals.hyperlink_color = egui::Color32::from_rgb(255, 140, 0);
    style.visuals.selection.bg_fill = egui::Color32::from_rgb(255, 100, 0).gamma_multiply(0.4);
    style.visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 140, 0));
    
    // Window styling
    style.visuals.window_fill = egui::Color32::from_rgb(25, 25, 30);
    style.visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 70));
    style.visuals.window_shadow = egui::epaint::Shadow {
        offset: egui::vec2(4.0, 4.0),
        blur: 8.0,
        spread: 0.0,
        color: egui::Color32::from_black_alpha(60),
    };
    
    // Panel background
    style.visuals.panel_fill = egui::Color32::from_rgb(30, 30, 35);
    
    // Widgets
    style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(40, 40, 45);
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(50, 50, 55);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(70, 70, 80);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(255, 100, 0);
    
    // Button styling
    style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(55, 55, 65);
    style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(80, 80, 95);
    
    // Spacing
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(12.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    
    // Rounding
    style.visuals.window_rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(4.0);
    
    ctx.set_style(style);
}
