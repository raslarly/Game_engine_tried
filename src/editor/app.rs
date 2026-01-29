//! Editor Application
//! 
//! Main editor window and state management.

use eframe::egui;
use super::panels::{HierarchyPanel, InspectorPanel, ScenePanel, AssetPanel, ConsolePanel};
use super::project::Project;
use super::scene::Scene;
use crate::engine::{World, Time, InputManager, Renderer};
use crate::scripting::PythonRuntime;

/// Editor mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Edit,
    Play,
    Pause,
}

/// Main editor application
pub struct EditorApp {
    // Core systems
    world: World,
    time: Time,
    input: InputManager,
    renderer: Renderer,
    python: Option<PythonRuntime>,
    
    // Editor state
    mode: EditorMode,
    project: Option<Project>,
    current_scene: Option<Scene>,
    selected_entity: Option<crate::engine::ecs::EntityId>,
    
    // Panels
    hierarchy_panel: HierarchyPanel,
    inspector_panel: InspectorPanel,
    scene_panel: ScenePanel,
    asset_panel: AssetPanel,
    console_panel: ConsolePanel,
    
    // UI state
    show_hierarchy: bool,
    show_inspector: bool,
    show_assets: bool,
    show_console: bool,
    show_settings: bool,
    show_about: bool,
    
    // Console messages
    console_messages: Vec<ConsoleMessage>,
}

#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl EditorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut input = InputManager::new();
        input.setup_default_bindings();
        
        let python = match PythonRuntime::new() {
            Ok(rt) => {
                tracing::info!("Python runtime initialized successfully");
                Some(rt)
            }
            Err(e) => {
                tracing::warn!("Failed to initialize Python: {}", e);
                None
            }
        };
        
        Self {
            world: World::new(),
            time: Time::new(),
            input,
            renderer: Renderer::new(),
            python,
            mode: EditorMode::Edit,
            project: None,
            current_scene: None,
            selected_entity: None,
            hierarchy_panel: HierarchyPanel::new(),
            inspector_panel: InspectorPanel::new(),
            scene_panel: ScenePanel::new(),
            asset_panel: AssetPanel::new(),
            console_panel: ConsolePanel::new(),
            show_hierarchy: true,
            show_inspector: true,
            show_assets: true,
            show_console: true,
            show_settings: false,
            show_about: false,
            console_messages: Vec::new(),
        }
    }
    
    fn log(&mut self, level: LogLevel, message: &str) {
        let now = chrono::Local::now();
        self.console_messages.push(ConsoleMessage {
            level,
            message: message.to_string(),
            timestamp: now.format("%H:%M:%S").to_string(),
        });
    }
    
    fn render_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // File menu
                ui.menu_button("📁 File", |ui| {
                    if ui.button("New Project...").clicked() {
                        self.log(LogLevel::Info, "Creating new project...");
                        ui.close_menu();
                    }
                    if ui.button("Open Project...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Phoenix Project", &["phoenix"])
                            .pick_file()
                        {
                            self.log(LogLevel::Info, &format!("Opening project: {:?}", path));
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("New Scene").clicked() {
                        self.current_scene = Some(Scene::new("Untitled"));
                        self.log(LogLevel::Info, "Created new scene");
                        ui.close_menu();
                    }
                    if ui.button("Save Scene").clicked() {
                        self.log(LogLevel::Info, "Scene saved");
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                // Edit menu
                ui.menu_button("✏️ Edit", |ui| {
                    if ui.button("Undo (Ctrl+Z)").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Redo (Ctrl+Y)").clicked() {
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Project Settings...").clicked() {
                        self.show_settings = true;
                        ui.close_menu();
                    }
                });
                
                // View menu
                ui.menu_button("👁️ View", |ui| {
                    ui.checkbox(&mut self.show_hierarchy, "Hierarchy");
                    ui.checkbox(&mut self.show_inspector, "Inspector");
                    ui.checkbox(&mut self.show_assets, "Assets");
                    ui.checkbox(&mut self.show_console, "Console");
                });
                
                // GameObject menu
                ui.menu_button("🎮 GameObject", |ui| {
                    if ui.button("Create Empty").clicked() {
                        let id = self.world.spawn_named("Empty", (
                            crate::engine::ecs::Name::from("Empty"),
                            crate::engine::Transform::default(),
                            crate::engine::ecs::Active::enabled(),
                        ));
                        self.selected_entity = Some(id);
                        self.log(LogLevel::Info, "Created empty GameObject");
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.menu_button("2D Objects", |ui| {
                        if ui.button("Sprite").clicked() {
                            self.log(LogLevel::Info, "Created Sprite");
                            ui.close_menu();
                        }
                        if ui.button("Animated Sprite").clicked() {
                            self.log(LogLevel::Info, "Created Animated Sprite");
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Physics", |ui| {
                        if ui.button("Rigidbody").clicked() {
                            ui.close_menu();
                        }
                        if ui.button("Box Collider").clicked() {
                            ui.close_menu();
                        }
                    });
                });
                
                // Python menu
                ui.menu_button("🐍 Python", |ui| {
                    let has_python = self.python.is_some();
                    ui.set_enabled(has_python);
                    
                    if ui.button("Run Script...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Python Script", &["py"])
                            .pick_file()
                        {
                            if let Some(ref mut py) = self.python {
                                match std::fs::read_to_string(&path) {
                                    Ok(code) => match py.execute(&code) {
                                        Ok(_) => self.log(LogLevel::Info, "Script executed successfully"),
                                        Err(e) => self.log(LogLevel::Error, &format!("Script error: {}", e)),
                                    },
                                    Err(e) => self.log(LogLevel::Error, &format!("Failed to read script: {}", e)),
                                }
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("Open Config Editor").clicked() {
                        self.log(LogLevel::Info, "Opening Python config editor...");
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Reload Scripts").clicked() {
                        self.log(LogLevel::Info, "Reloading Python scripts...");
                        ui.close_menu();
                    }
                    
                    if !has_python {
                        ui.separator();
                        ui.label("⚠️ Python not available");
                    }
                });
                
                // Help menu
                ui.menu_button("❓ Help", |ui| {
                    if ui.button("Documentation").clicked() {
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("About Phoenix Engine").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
                
                // Spacer
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Play controls
                    let play_btn = if self.mode == EditorMode::Play {
                        "⏹ Stop"
                    } else {
                        "▶ Play"
                    };
                    
                    if ui.button(play_btn).clicked() {
                        self.mode = match self.mode {
                            EditorMode::Edit => {
                                self.log(LogLevel::Info, "Entering play mode");
                                EditorMode::Play
                            }
                            EditorMode::Play | EditorMode::Pause => {
                                self.log(LogLevel::Info, "Stopped");
                                EditorMode::Edit
                            }
                        };
                    }
                    
                    if self.mode == EditorMode::Play {
                        if ui.button("⏸ Pause").clicked() {
                            self.mode = EditorMode::Pause;
                        }
                    } else if self.mode == EditorMode::Pause {
                        if ui.button("▶ Resume").clicked() {
                            self.mode = EditorMode::Play;
                        }
                    }
                    
                    ui.separator();
                    ui.label(format!("FPS: {:.0}", self.time.fps()));
                });
            });
        });
    }
    
    fn render_panels(&mut self, ctx: &egui::Context) {
        // Left panel - Hierarchy
        if self.show_hierarchy {
            egui::SidePanel::left("hierarchy_panel")
                .default_width(250.0)
                .min_width(200.0)
                .show(ctx, |ui| {
                    self.hierarchy_panel.show(ui, &self.world, &mut self.selected_entity);
                });
        }
        
        // Right panel - Inspector
        if self.show_inspector {
            egui::SidePanel::right("inspector_panel")
                .default_width(320.0)
                .min_width(250.0)
                .show(ctx, |ui| {
                    self.inspector_panel.show(ui, &mut self.world, self.selected_entity);
                });
        }
        
        // Bottom panel - Assets & Console
        if self.show_assets || self.show_console {
            egui::TopBottomPanel::bottom("bottom_panel")
                .default_height(200.0)
                .min_height(100.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.show_assets, true, "📁 Assets");
                        ui.selectable_value(&mut self.show_console, true, "📋 Console");
                    });
                    ui.separator();
                    
                    if self.show_console {
                        self.console_panel.show(ui, &self.console_messages);
                    } else {
                        self.asset_panel.show(ui);
                    }
                });
        }
        
        // Central panel - Scene view
        egui::CentralPanel::default().show(ctx, |ui| {
            self.scene_panel.show(ui, &mut self.renderer, &self.world, self.selected_entity);
        });
    }
    
    fn render_dialogs(&mut self, ctx: &egui::Context) {
        // About dialog
        if self.show_about {
            egui::Window::new("About Phoenix Engine")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("🔥 Phoenix Engine");
                        ui.label("Version 0.1.0");
                        ui.add_space(10.0);
                        ui.label("A 2D Action-Adventure Game Engine");
                        ui.label("Built with Rust, EGUI, and Python");
                        ui.add_space(10.0);
                        
                        if self.python.is_some() {
                            ui.label("✅ Python integration active");
                        } else {
                            ui.label("⚠️ Python integration unavailable");
                        }
                        
                        ui.add_space(20.0);
                        if ui.button("Close").clicked() {
                            self.show_about = false;
                        }
                    });
                });
        }
        
        // Settings dialog
        if self.show_settings {
            egui::Window::new("Project Settings")
                .collapsible(false)
                .resizable(true)
                .default_size([500.0, 400.0])
                .show(ctx, |ui| {
                    egui::CollapsingHeader::new("Graphics")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label("Resolution: 1920x1080");
                            ui.label("VSync: Enabled");
                        });
                    
                    egui::CollapsingHeader::new("Physics")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label("Gravity: (0, -980)");
                            ui.label("Fixed Timestep: 1/60");
                        });
                    
                    egui::CollapsingHeader::new("Input")
                        .default_open(false)
                        .show(ui, |_ui| {
                            // Input bindings editor
                        });
                    
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            self.log(LogLevel::Info, "Settings saved");
                            self.show_settings = false;
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_settings = false;
                        }
                    });
                });
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update time
        self.time.update();
        
        // Update input
        self.input.update(ctx);
        
        // Game logic (in play mode)
        if self.mode == EditorMode::Play {
            // Run fixed updates
            while self.time.should_fixed_update() {
                // Physics updates would go here
            }
            
            // Request continuous repaint for game loop
            ctx.request_repaint();
        }
        
        // Render UI
        self.render_menu_bar(ctx);
        self.render_panels(ctx);
        self.render_dialogs(ctx);
    }
}
