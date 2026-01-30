//! Editor Panels
//! 
//! UI panels for the editor interface.

use egui::{Color32, RichText, Ui};
use crate::engine::ecs::{EntityId, World, Name, Active, Layer};
use crate::engine::{Transform, Renderer};
use super::app::{ConsoleMessage, LogLevel};

/// Hierarchy panel - shows all entities in the scene
pub struct HierarchyPanel {
    search_query: String,
}

impl HierarchyPanel {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
        }
    }
    
    pub fn show(&mut self, ui: &mut Ui, world: &World, selected: &mut Option<EntityId>) {
        ui.heading("🏗️ Hierarchy");
        ui.separator();
        
        // Search bar
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_query);
        });
        ui.separator();
        
        // Entity list
        egui::ScrollArea::vertical().show(ui, |ui| {
            let entity_count = world.entity_count();
            
            if entity_count == 0 {
                ui.label(RichText::new("No entities in scene").italics().color(Color32::GRAY));
                ui.label("Right-click to create objects");
            } else {
                for id in world.iter_ids() {
                    let name = world.get::<Name>(id)
                        .map(|n| n.0.clone())
                        .unwrap_or_else(|| format!("Entity {:?}", id));
                    
                    // Filter by search
                    if !self.search_query.is_empty() && 
                       !name.to_lowercase().contains(&self.search_query.to_lowercase()) {
                        continue;
                    }
                    
                    let is_selected = *selected == Some(id);
                    let active = world.get::<Active>(id)
                        .map(|a| a.0)
                        .unwrap_or(true);
                    
                    let text = if active {
                        RichText::new(&name)
                    } else {
                        RichText::new(&name).color(Color32::GRAY)
                    };
                    
                    if ui.selectable_label(is_selected, text).clicked() {
                        *selected = Some(id);
                    }
                }
            }
        });
        
        // Context menu
        ui.separator();
        ui.horizontal(|ui| {
            if ui.small_button("➕ Create").clicked() {
                // Will be handled by menu
            }
            if ui.small_button("🗑️ Delete").clicked() {
                if selected.is_some() {
                    // Delete selected entity
                }
            }
        });
    }
}

/// Inspector panel - shows details of selected entity
pub struct InspectorPanel;

impl InspectorPanel {
    pub fn new() -> Self {
        Self
    }
    
    pub fn show(&mut self, ui: &mut Ui, world: &mut World, selected: Option<EntityId>) {
        ui.heading("🔍 Inspector");
        ui.separator();
        
        if let Some(id) = selected {
            // Name component
            if let Some(mut name) = world.get_mut::<Name>(id) {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut name.0);
                });
            }
            
            // Active toggle
            if let Some(mut active) = world.get_mut::<Active>(id) {
                ui.checkbox(&mut active.0, "Active");
            }
            
            // Layer
            if let Some(mut layer) = world.get_mut::<Layer>(id) {
                ui.horizontal(|ui| {
                    ui.label("Layer:");
                    ui.add(egui::DragValue::new(&mut layer.0));
                });
            }
            
            ui.separator();
            
            // Transform component
            egui::CollapsingHeader::new("📐 Transform")
                .default_open(true)
                .show(ui, |ui| {
                    let has_transform = world.get::<Transform>(id).is_some();
                    
                    if has_transform {
                        if let Some(mut transform) = world.get_mut::<Transform>(id) {
                            ui.horizontal(|ui| {
                                ui.label("Position:");
                                ui.add(egui::DragValue::new(&mut transform.position.x).prefix("X: ").speed(1.0));
                                ui.add(egui::DragValue::new(&mut transform.position.y).prefix("Y: ").speed(1.0));
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("Rotation:");
                                let mut degrees = transform.rotation.to_degrees();
                                if ui.add(egui::DragValue::new(&mut degrees).suffix("°").speed(1.0)).changed() {
                                    transform.rotation = degrees.to_radians();
                                }
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("Scale:");
                                ui.add(egui::DragValue::new(&mut transform.scale.x).prefix("X: ").speed(0.1));
                                ui.add(egui::DragValue::new(&mut transform.scale.y).prefix("Y: ").speed(0.1));
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("Z-Order:");
                                ui.add(egui::DragValue::new(&mut transform.z_order).speed(0.1));
                            });
                        }
                    } else {
                        if ui.button("Add Transform").clicked() {
                            world.add_component(id, Transform::default());
                        }
                    }
                });
            
            ui.separator();
            
            // Add component button
            ui.menu_button("➕ Add Component", |ui| {
                if ui.button("Sprite").clicked() {
                    ui.close_menu();
                }
                if ui.button("Rigidbody").clicked() {
                    ui.close_menu();
                }
                if ui.button("Collider").clicked() {
                    ui.close_menu();
                }
                if ui.button("Audio Source").clicked() {
                    ui.close_menu();
                }
            });
        } else {
            ui.label(RichText::new("No entity selected").italics().color(Color32::GRAY));
        }
    }
}

/// Scene panel - viewport for the game
pub struct ScenePanel {
    zoom: f32,
    pan_offset: egui::Vec2,
    grid_enabled: bool,
}

impl ScenePanel {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            pan_offset: egui::Vec2::ZERO,
            grid_enabled: true,
        }
    }
    
    pub fn show(&mut self, ui: &mut Ui, _renderer: &mut Renderer, _world: &World, _selected: Option<EntityId>) {
        // Toolbar
        ui.horizontal(|ui| {
            ui.label("Scene View");
            ui.separator();
            
            if ui.button("🔲").on_hover_text("Toggle Grid").clicked() {
                self.grid_enabled = !self.grid_enabled;
            }
            
            ui.separator();
            ui.label("Zoom:");
            if ui.button("-").clicked() {
                self.zoom = (self.zoom - 0.1).max(0.1);
            }
            ui.label(format!("{:.0}%", self.zoom * 100.0));
            if ui.button("+").clicked() {
                self.zoom = (self.zoom + 0.1).min(5.0);
            }
            if ui.button("Reset").clicked() {
                self.zoom = 1.0;
                self.pan_offset = egui::Vec2::ZERO;
            }
        });
        
        ui.separator();
        
        // Scene viewport
        let (response, painter) = ui.allocate_painter(
            ui.available_size(),
            egui::Sense::click_and_drag(),
        );
        
        let rect = response.rect;
        let center = rect.center();
        
        // Background
        painter.rect_filled(rect, 0.0, Color32::from_rgb(30, 30, 35));
        
        // Grid
        if self.grid_enabled {
            let grid_size = 50.0 * self.zoom;
            let grid_color = Color32::from_rgba_unmultiplied(80, 80, 100, 40);
            
            // Vertical lines
            let mut x = center.x + self.pan_offset.x % grid_size;
            while x < rect.max.x {
                painter.line_segment(
                    [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                    egui::Stroke::new(1.0, grid_color),
                );
                x += grid_size;
            }
            let mut x = center.x + self.pan_offset.x % grid_size - grid_size;
            while x > rect.min.x {
                painter.line_segment(
                    [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                    egui::Stroke::new(1.0, grid_color),
                );
                x -= grid_size;
            }
            
            // Horizontal lines
            let mut y = center.y + self.pan_offset.y % grid_size;
            while y < rect.max.y {
                painter.line_segment(
                    [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                    egui::Stroke::new(1.0, grid_color),
                );
                y += grid_size;
            }
            let mut y = center.y + self.pan_offset.y % grid_size - grid_size;
            while y > rect.min.y {
                painter.line_segment(
                    [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                    egui::Stroke::new(1.0, grid_color),
                );
                y -= grid_size;
            }
            
            // Origin axes
            let origin = center + self.pan_offset;
            if rect.contains(origin) {
                // X axis (red)
                painter.line_segment(
                    [origin, egui::pos2(rect.max.x, origin.y)],
                    egui::Stroke::new(2.0, Color32::from_rgb(255, 100, 100)),
                );
                // Y axis (green) - pointing up
                painter.line_segment(
                    [origin, egui::pos2(origin.x, rect.min.y)],
                    egui::Stroke::new(2.0, Color32::from_rgb(100, 255, 100)),
                );
            }
        }
        
        // Handle panning
        if response.dragged_by(egui::PointerButton::Middle) ||
           (response.dragged_by(egui::PointerButton::Primary) && ui.input(|i| i.modifiers.alt))
        {
            self.pan_offset += response.drag_delta();
        }
        
        // Handle zooming
        if response.hovered() {
            let scroll = ui.input(|i| i.raw_scroll_delta.y);
            if scroll != 0.0 {
                self.zoom = (self.zoom + scroll * 0.001).clamp(0.1, 5.0);
            }
        }
        
        // Draw placeholder text
        painter.text(
            center + self.pan_offset,
            egui::Align2::CENTER_CENTER,
            "Scene View",
            egui::FontId::proportional(24.0),
            Color32::from_rgba_unmultiplied(255, 255, 255, 60),
        );
    }
}

/// Asset panel - shows project assets
pub struct AssetPanel {
    current_path: String,
}

impl AssetPanel {
    pub fn new() -> Self {
        Self {
            current_path: "assets".to_string(),
        }
    }
    
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("📂");
            ui.label(&self.current_path);
            ui.separator();
            if ui.button("↑").on_hover_text("Go up").clicked() {
                // Navigate up
            }
            if ui.button("🔄").on_hover_text("Refresh").clicked() {
                // Refresh
            }
        });
        
        ui.separator();
        
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                // Placeholder assets
                for i in 0..5 {
                    ui.vertical(|ui| {
                        let rect = ui.allocate_space(egui::vec2(64.0, 64.0));
                        ui.painter().rect_filled(
                            rect.1,
                            4.0,
                            Color32::from_rgb(60, 60, 70),
                        );
                        let name = match i {
                            0 => "sprites",
                            1 => "audio",
                            2 => "scripts",
                            3 => "fonts",
                            _ => "config",
                        };
                        ui.label(name);
                    });
                }
            });
        });
    }
}

/// Console panel - shows log messages
pub struct ConsolePanel {
    filter_level: Option<LogLevel>,
    auto_scroll: bool,
}

impl ConsolePanel {
    pub fn new() -> Self {
        Self {
            filter_level: None,
            auto_scroll: true,
        }
    }
    
    pub fn show(&mut self, ui: &mut Ui, messages: &[ConsoleMessage]) {
        ui.horizontal(|ui| {
            if ui.button("Clear").clicked() {
                // Clear handled by app
            }
            ui.separator();
            
            let info_btn = ui.selectable_label(self.filter_level == Some(LogLevel::Info), "ℹ️ Info");
            if info_btn.clicked() {
                self.filter_level = if self.filter_level == Some(LogLevel::Info) { None } else { Some(LogLevel::Info) };
            }
            
            let warn_btn = ui.selectable_label(self.filter_level == Some(LogLevel::Warning), "⚠️ Warn");
            if warn_btn.clicked() {
                self.filter_level = if self.filter_level == Some(LogLevel::Warning) { None } else { Some(LogLevel::Warning) };
            }
            
            let err_btn = ui.selectable_label(self.filter_level == Some(LogLevel::Error), "❌ Error");
            if err_btn.clicked() {
                self.filter_level = if self.filter_level == Some(LogLevel::Error) { None } else { Some(LogLevel::Error) };
            }
            
            ui.separator();
            ui.checkbox(&mut self.auto_scroll, "Auto-scroll");
        });
        
        ui.separator();
        
        let scroll_area = egui::ScrollArea::vertical()
            .stick_to_bottom(self.auto_scroll)
            .show(ui, |ui| {
                for msg in messages {
                    if let Some(filter) = self.filter_level {
                        if msg.level != filter {
                            continue;
                        }
                    }
                    
                    let (icon, color) = match msg.level {
                        LogLevel::Info => ("ℹ️", Color32::from_rgb(150, 200, 255)),
                        LogLevel::Warning => ("⚠️", Color32::from_rgb(255, 200, 100)),
                        LogLevel::Error => ("❌", Color32::from_rgb(255, 100, 100)),
                    };
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&msg.timestamp).color(Color32::GRAY).small());
                        ui.label(icon);
                        ui.label(RichText::new(&msg.message).color(color));
                    });
                }
            });
        
        let _ = scroll_area;
    }
}
