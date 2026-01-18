use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("Tacticus - AI Chess Trainer"),
        ..Default::default()
    };

    eframe::run_native(
        "Tacticus",
        options,
        Box::new(|cc| {
            // Apply webcore Windows XP theme
            apply_webcore_theme(&cc.egui_ctx);
            Ok(Box::<TacticusApp>::default())
        }),
    )
}

fn apply_webcore_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // WEBCORE XP - More saturated, bolder colors
    let xp_blue = egui::Color32::from_rgb(0, 84, 166);
    let xp_blue_light = egui::Color32::from_rgb(59, 130, 246);
    let xp_bg = egui::Color32::from_rgb(236, 233, 216);        // Classic beige
    let xp_button = egui::Color32::from_rgb(236, 233, 216);
    let xp_button_hover = egui::Color32::from_rgb(255, 240, 200);
    let xp_border_dark = egui::Color32::from_rgb(0, 60, 116);  // Dark border
    let xp_text = egui::Color32::from_rgb(0, 0, 0);

    // WEBCORE: Chunkier spacing
    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.spacing.button_padding = egui::vec2(16.0, 8.0);
    style.spacing.window_margin = egui::Margin::same(10.0);
    style.spacing.menu_margin = egui::Margin::same(8.0);

    // WEBCORE: More pronounced rounded corners
    style.visuals.window_rounding = egui::Rounding::same(8.0);
    style.visuals.menu_rounding = egui::Rounding::same(6.0);

    // Window with stronger borders
    style.visuals.window_fill = xp_bg;
    style.visuals.window_stroke = egui::Stroke::new(2.0, xp_border_dark);
    style.visuals.window_shadow = egui::epaint::Shadow {
        offset: egui::vec2(6.0, 6.0),
        blur: 12.0,
        spread: 2.0,
        color: egui::Color32::from_black_alpha(80),
    };

    style.visuals.panel_fill = xp_bg;

    // WEBCORE: Chunky 3D buttons with strong borders
    style.visuals.widgets.noninteractive.bg_fill = xp_button;
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(2.0, xp_border_dark);

    style.visuals.widgets.inactive.bg_fill = xp_button;
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(2.0, xp_border_dark);
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.5, xp_text);

    style.visuals.widgets.hovered.bg_fill = xp_button_hover;
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.5, xp_blue_light);
    style.visuals.widgets.hovered.expansion = 2.0;

    style.visuals.widgets.active.bg_fill = xp_blue_light;
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(2.5, xp_border_dark);
    style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.5, egui::Color32::WHITE);

    style.visuals.selection.bg_fill = xp_blue;
    style.visuals.selection.stroke = egui::Stroke::new(1.5, egui::Color32::WHITE);

    style.visuals.hyperlink_color = xp_blue_light;
    style.visuals.override_text_color = Some(xp_text);
    style.visuals.extreme_bg_color = egui::Color32::from_rgb(255, 255, 255);

    ctx.set_style(style);
}

struct TacticusApp {
    // Core state
    user_input: String,
    coach_response: String,

    // Application state
    current_view: AppView,
    is_first_run: bool,

    // Settings
    openrouter_key: String,
}

#[derive(Default, PartialEq, Clone, Copy)]
enum AppView {
    #[default]
    Home,
    Play,
    Train,
    Analyze,
    Profile,
    Settings,
}

impl Default for TacticusApp {
    fn default() -> Self {
        // Try to load from .env
        dotenv::dotenv().ok();
        let openrouter_key = std::env::var("OPENROUTER_API_KEY").unwrap_or_default();

        // Check if this is first run (no API key configured)
        let is_first_run = openrouter_key.is_empty();

        let coach_response = if is_first_run {
            "═══ WELCOME TO TACTICUS! ═══\n\n\
            Your AI-powered chess training journey starts here!\n\n\
            FIRST TIME SETUP:\n\
            → Click 'SETTINGS' in the top right\n\
            → Enter your OpenRouter API key\n\
            → Click 'SAVE SETTINGS'\n\n\
            Don't have an API key?\n\
            → Visit https://openrouter.ai to get free credits\n\
            → It takes less than 1 minute!\n\n\
            Once configured, I'll be your personal chess coach, \
            analyzing your games and helping you improve!".to_string()
        } else {
            "═══ WELCOME BACK! ═══\n\n\
            Your AI Chess Coach is ready to help you improve!\n\n\
            WHAT WOULD YOU LIKE TO DO?\n\
            → PLAY - Start a new game and practice\n\
            → TRAIN - Get personalized exercises\n\
            → ANALYZE - Review your recent games\n\
            → PROFILE - Check your progress\n\n\
            Or just ask me anything about chess below!".to_string()
        };

        Self {
            user_input: String::new(),
            coach_response,
            current_view: AppView::Home,
            is_first_run,
            openrouter_key,
        }
    }
}

impl eframe::App for TacticusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // WEBCORE: Chunky title bar with strong gradient feel
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame {
                fill: egui::Color32::from_rgb(0, 84, 166),
                stroke: egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 50, 100)), // Chunky border
                inner_margin: egui::Margin::symmetric(16.0, 10.0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

                    // Big chunky title
                    let title = egui::RichText::new("⚔ TACTICUS ⚔")
                        .size(20.0)
                        .strong();
                    ui.label(title);

                    ui.add_space(30.0);
                    ui.separator();
                    ui.add_space(20.0);

                    // WEBCORE: Chunky tab buttons
                    let make_tab = |ui: &mut egui::Ui, text: &str, view: AppView, current: AppView| {
                        let is_active = view == current;
                        let (fill, text_color) = if is_active {
                            (egui::Color32::from_rgb(236, 233, 216), egui::Color32::from_rgb(0, 0, 0))
                        } else {
                            (egui::Color32::from_rgb(49, 106, 197), egui::Color32::WHITE)
                        };

                        let button = egui::Button::new(
                            egui::RichText::new(text)
                                .color(text_color)
                                .size(14.0)
                                .strong()
                        )
                        .fill(fill)
                        .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 50, 100)))
                        .min_size(egui::vec2(90.0, 32.0));

                        ui.add(button).clicked()
                    };

                    if make_tab(ui, "HOME", AppView::Home, self.current_view) {
                        self.current_view = AppView::Home;
                    }
                    ui.add_space(5.0);
                    if make_tab(ui, "PLAY", AppView::Play, self.current_view) {
                        self.current_view = AppView::Play;
                    }
                    ui.add_space(5.0);
                    if make_tab(ui, "TRAIN", AppView::Train, self.current_view) {
                        self.current_view = AppView::Train;
                    }
                    ui.add_space(5.0);
                    if make_tab(ui, "ANALYZE", AppView::Analyze, self.current_view) {
                        self.current_view = AppView::Analyze;
                    }
                    ui.add_space(5.0);
                    if make_tab(ui, "PROFILE", AppView::Profile, self.current_view) {
                        self.current_view = AppView::Profile;
                    }

                    // Settings on the right
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if make_tab(ui, "⚙ SETTINGS", AppView::Settings, self.current_view) {
                            self.current_view = AppView::Settings;
                        }
                    });
                });
            });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                AppView::Home => self.show_home(ui),
                AppView::Play => self.show_play(ui),
                AppView::Train => self.show_train(ui),
                AppView::Analyze => self.show_analyze(ui),
                AppView::Profile => self.show_profile(ui),
                AppView::Settings => self.show_settings(ui),
            }
        });
    }
}

impl TacticusApp {
    fn show_home(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);

        ui.vertical_centered(|ui| {
            // WEBCORE: Chunky welcome header
            let header_frame = egui::Frame {
                fill: egui::Color32::from_rgb(255, 255, 255),
                stroke: egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 60, 116)), // CHUNKY
                rounding: egui::Rounding::same(8.0),
                inner_margin: egui::Margin::same(20.0),
                shadow: egui::epaint::Shadow {
                    offset: egui::vec2(4.0, 4.0),
                    blur: 10.0,
                    spread: 2.0,
                    color: egui::Color32::from_black_alpha(100),
                },
                ..Default::default()
            };

            header_frame.show(ui, |ui| {
                ui.set_min_width(900.0);
                ui.vertical_centered(|ui| {
                    let title_text = egui::RichText::new("═══ WELCOME TO TACTICUS ═══")
                        .size(28.0)
                        .color(egui::Color32::from_rgb(0, 84, 166))
                        .strong();
                    ui.label(title_text);

                    ui.add_space(6.0);
                    let subtitle = egui::RichText::new("★ AI-POWERED CHESS TRAINING ★")
                        .size(16.0)
                        .color(egui::Color32::from_rgb(100, 100, 100));
                    ui.label(subtitle);
                });
            });

            ui.add_space(20.0);

            // WEBCORE: Chunky coach panel
            let coach_frame = egui::Frame {
                fill: egui::Color32::from_rgb(245, 245, 245),
                stroke: egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 84, 166)), // CHUNKY
                rounding: egui::Rounding::same(8.0),
                inner_margin: egui::Margin::same(16.0),
                outer_margin: egui::Margin::same(10.0),
                shadow: egui::epaint::Shadow {
                    offset: egui::vec2(5.0, 5.0),
                    blur: 12.0,
                    spread: 2.0,
                    color: egui::Color32::from_black_alpha(90),
                },
                ..Default::default()
            };

            coach_frame.show(ui, |ui| {
                ui.set_min_width(900.0);

                // Group box title with banner
                egui::Frame {
                    fill: egui::Color32::from_rgb(0, 84, 166),
                    inner_margin: egui::Margin::symmetric(16.0, 8.0),
                    rounding: egui::Rounding::same(4.0),
                    ..Default::default()
                }.show(ui, |ui| {
                    let title = egui::RichText::new("═══ YOUR AI COACH ═══")
                        .size(18.0)
                        .color(egui::Color32::WHITE)
                        .strong();
                    ui.label(title);
                });

                ui.add_space(12.0);

                // Response area with chunky border
                egui::Frame {
                    fill: egui::Color32::WHITE,
                    stroke: egui::Stroke::new(2.5, egui::Color32::from_rgb(100, 100, 100)),
                    inner_margin: egui::Margin::same(12.0),
                    rounding: egui::Rounding::same(4.0),
                    ..Default::default()
                }
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            ui.label(&self.coach_response);
                        });
                });

                ui.add_space(16.0);

                // Input area with chunky button
                ui.horizontal(|ui| {
                    let label = egui::RichText::new("Ask your coach:")
                        .size(14.0)
                        .strong();
                    ui.label(label);
                    ui.add_space(10.0);

                    let text_edit = egui::TextEdit::singleline(&mut self.user_input)
                        .desired_width(550.0)
                        .hint_text("Type your question here...");
                    let response = ui.add(text_edit);

                    ui.add_space(10.0);

                    // WEBCORE: Chunky send button - disabled if no API key
                    let can_send = !self.openrouter_key.is_empty() && !self.user_input.is_empty();
                    let button_text = if self.openrouter_key.is_empty() {
                        "⚙ CONFIGURE API KEY FIRST"
                    } else if self.user_input.is_empty() {
                        "▶ SEND (or press Enter)"
                    } else {
                        "▶ SEND"
                    };

                    let send_button = egui::Button::new(
                        egui::RichText::new(button_text)
                            .size(14.0)
                            .strong()
                    )
                    .min_size(egui::vec2(100.0, 32.0));

                    let send_button = ui.add_enabled(can_send, send_button);

                    if send_button.clicked()
                        || (can_send && response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        // TODO: Call LLM agent
                        self.coach_response = format!(
                            "You asked: {}\n\n[LLM Response will appear here once integrated]",
                            self.user_input
                        );
                        self.user_input.clear();
                    }
                });
            });
        });
    }

    fn show_play(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        self.show_xp_panel(ui, "Play a Game", |ui| {
            ui.vertical_centered(|ui| {
                let icon = egui::RichText::new("♟")
                    .size(48.0)
                    .color(egui::Color32::from_rgb(0, 84, 166));
                ui.label(icon);

                ui.add_space(12.0);

                let title = egui::RichText::new("CHESS BOARD COMING SOON")
                    .size(18.0)
                    .strong()
                    .color(egui::Color32::from_rgb(0, 84, 166));
                ui.label(title);

                ui.add_space(16.0);

                ui.label("This feature will include:");
                ui.add_space(8.0);

                egui::Frame {
                    fill: egui::Color32::from_rgb(255, 255, 255),
                    stroke: egui::Stroke::new(2.0, egui::Color32::from_rgb(200, 200, 200)),
                    inner_margin: egui::Margin::same(12.0),
                    rounding: egui::Rounding::same(4.0),
                    ..Default::default()
                }.show(ui, |ui| {
                    ui.label("✓ Interactive chess board with drag & drop");
                    ui.label("✓ Play against the AI engine");
                    ui.label("✓ Optional coach hints during play");
                    ui.label("✓ Move validation and legal move highlighting");
                    ui.label("✓ Post-game analysis with AI coach feedback");
                });

                ui.add_space(16.0);

                ui.label("For now, use the HOME tab to chat with your AI coach!");
            });
        });
    }

    fn show_train(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        self.show_xp_panel(ui, "Training Exercises", |ui| {
            ui.vertical_centered(|ui| {
                let icon = egui::RichText::new("📚")
                    .size(48.0);
                ui.label(icon);

                ui.add_space(12.0);

                let title = egui::RichText::new("TRAINING SYSTEM COMING SOON")
                    .size(18.0)
                    .strong()
                    .color(egui::Color32::from_rgb(0, 84, 166));
                ui.label(title);

                ui.add_space(16.0);

                ui.label("Personalized exercises tailored to your skill level");
                ui.add_space(8.0);

                egui::Frame {
                    fill: egui::Color32::from_rgb(255, 255, 255),
                    stroke: egui::Stroke::new(2.0, egui::Color32::from_rgb(200, 200, 200)),
                    inner_margin: egui::Margin::same(12.0),
                    rounding: egui::Rounding::same(4.0),
                    ..Default::default()
                }.show(ui, |ui| {
                    ui.label("✓ 5-10 exercises based on your weaknesses");
                    ui.label("✓ Multiple difficulty levels");
                    ui.label("✓ Hints from your AI coach when stuck");
                    ui.label("✓ Detailed explanations after solving");
                    ui.label("✓ Progress tracking and improvement metrics");
                });

                ui.add_space(16.0);

                ui.label("For now, ask your AI coach for training tips on the HOME tab!");
            });
        });
    }

    fn show_analyze(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        self.show_xp_panel(ui, "Analyze Games", |ui| {
            ui.vertical_centered(|ui| {
                let icon = egui::RichText::new("🔍")
                    .size(48.0);
                ui.label(icon);

                ui.add_space(12.0);

                let title = egui::RichText::new("GAME ANALYSIS COMING SOON")
                    .size(18.0)
                    .strong()
                    .color(egui::Color32::from_rgb(0, 84, 166));
                ui.label(title);

                ui.add_space(16.0);

                ui.label("Deep AI-powered analysis of your games");
                ui.add_space(8.0);

                egui::Frame {
                    fill: egui::Color32::from_rgb(255, 255, 255),
                    stroke: egui::Stroke::new(2.0, egui::Color32::from_rgb(200, 200, 200)),
                    inner_margin: egui::Margin::same(12.0),
                    rounding: egui::Rounding::same(4.0),
                    ..Default::default()
                }.show(ui, |ui| {
                    ui.label("✓ Browse your game history");
                    ui.label("✓ Move-by-move analysis with best alternatives");
                    ui.label("✓ AI coach feedback on your decisions");
                    ui.label("✓ Identify patterns in your strengths and weaknesses");
                    ui.label("✓ Track improvement over time");
                });

                ui.add_space(16.0);

                ui.label("For now, describe your games to the AI coach on the HOME tab!");
            });
        });
    }

    fn show_profile(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        self.show_xp_panel(ui, "Your Chess Profile", |ui| {
            ui.vertical_centered(|ui| {
                let icon = egui::RichText::new("👤")
                    .size(48.0);
                ui.label(icon);

                ui.add_space(12.0);

                let title = egui::RichText::new("PLAYER PROFILE COMING SOON")
                    .size(18.0)
                    .strong()
                    .color(egui::Color32::from_rgb(0, 84, 166));
                ui.label(title);

                ui.add_space(16.0);

                ui.label("Your personalized chess journey dashboard");
                ui.add_space(8.0);

                egui::Frame {
                    fill: egui::Color32::from_rgb(255, 255, 255),
                    stroke: egui::Stroke::new(2.0, egui::Color32::from_rgb(200, 200, 200)),
                    inner_margin: egui::Margin::same(12.0),
                    rounding: egui::Rounding::same(4.0),
                    ..Default::default()
                }.show(ui, |ui| {
                    ui.label("✓ Current rating and skill level");
                    ui.label("✓ Playing style analysis (aggressive, defensive, etc.)");
                    ui.label("✓ Win/loss/draw statistics");
                    ui.label("✓ Improvement trends and graphs");
                    ui.label("✓ Training history and achievements");
                });

                ui.add_space(16.0);

                ui.label("For now, ask your AI coach about your progress on the HOME tab!");
            });
        });
    }

    fn show_settings(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);

        ui.vertical_centered(|ui| {
            // WEBCORE: Chunky settings panel
            let settings_frame = egui::Frame {
                fill: egui::Color32::from_rgb(245, 245, 245),
                stroke: egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 84, 166)),
                rounding: egui::Rounding::same(8.0),
                inner_margin: egui::Margin::same(20.0),
                shadow: egui::epaint::Shadow {
                    offset: egui::vec2(5.0, 5.0),
                    blur: 12.0,
                    spread: 2.0,
                    color: egui::Color32::from_black_alpha(90),
                },
                ..Default::default()
            };

            settings_frame.show(ui, |ui| {
                ui.set_min_width(900.0);

                // Title banner
                egui::Frame {
                    fill: egui::Color32::from_rgb(0, 84, 166),
                    inner_margin: egui::Margin::symmetric(16.0, 8.0),
                    rounding: egui::Rounding::same(4.0),
                    ..Default::default()
                }.show(ui, |ui| {
                    let title = egui::RichText::new("⚙ SETTINGS ⚙")
                        .size(20.0)
                        .color(egui::Color32::WHITE)
                        .strong();
                    ui.label(title);
                });

                ui.add_space(20.0);

                // OpenRouter API Key
                egui::Frame {
                    fill: egui::Color32::WHITE,
                    stroke: egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 100)),
                    inner_margin: egui::Margin::same(16.0),
                    rounding: egui::Rounding::same(6.0),
                    ..Default::default()
                }.show(ui, |ui| {
                    ui.vertical(|ui| {
                        let label = egui::RichText::new("OpenRouter API Key")
                            .size(16.0)
                            .strong()
                            .color(egui::Color32::from_rgb(0, 84, 166));
                        ui.label(label);

                        ui.add_space(8.0);

                        ui.label("Get your free API key at: https://openrouter.ai");

                        ui.add_space(12.0);

                        ui.horizontal(|ui| {
                            ui.label("API Key:");
                            ui.add_space(10.0);

                            let key_input = egui::TextEdit::singleline(&mut self.openrouter_key)
                                .password(true)
                                .desired_width(500.0);
                            ui.add(key_input);
                        });

                        ui.add_space(12.0);

                        let status_text = if self.openrouter_key.is_empty() {
                            egui::RichText::new("⚠ No API key set - AI features will not work!")
                                .color(egui::Color32::from_rgb(200, 0, 0))
                        } else {
                            egui::RichText::new("✓ API key configured - Ready to train!")
                                .color(egui::Color32::from_rgb(0, 150, 0))
                        };
                        ui.label(status_text);

                        ui.add_space(16.0);

                        if self.is_first_run && !self.openrouter_key.is_empty() {
                            let hint = egui::RichText::new("💡 Don't forget to click 'SAVE SETTINGS' below!")
                                .color(egui::Color32::from_rgb(200, 120, 0))
                                .size(14.0)
                                .strong();
                            ui.label(hint);
                        }
                    });
                });

                ui.add_space(20.0);

                // Database info (read-only)
                egui::Frame {
                    fill: egui::Color32::from_rgb(250, 250, 250),
                    stroke: egui::Stroke::new(2.0, egui::Color32::from_rgb(150, 150, 150)),
                    inner_margin: egui::Margin::same(16.0),
                    rounding: egui::Rounding::same(6.0),
                    ..Default::default()
                }.show(ui, |ui| {
                    ui.vertical(|ui| {
                        let label = egui::RichText::new("Database Storage")
                            .size(16.0)
                            .strong()
                            .color(egui::Color32::from_rgb(0, 84, 166));
                        ui.label(label);

                        ui.add_space(8.0);

                        ui.label("✓ Local SQLite database (automatically configured)");
                        ui.label("✓ Your data is stored locally on your computer");
                        ui.label("✓ No internet connection required for storage");

                        ui.add_space(8.0);

                        // Show database location
                        if let Some(data_dir) = dirs::data_local_dir() {
                            let db_path = data_dir.join("tacticus").join("chess_training.db");
                            ui.label(format!("Location: {}", db_path.display()));
                        }
                    });
                });

                ui.add_space(20.0);

                // Save button
                ui.vertical_centered(|ui| {
                    let save_button = egui::Button::new(
                        egui::RichText::new("💾 SAVE SETTINGS")
                            .size(16.0)
                            .strong()
                    )
                    .min_size(egui::vec2(200.0, 40.0));

                    if ui.add(save_button).clicked() {
                        // Save to .env file
                        match self.save_settings() {
                            Ok(_) => {
                                self.is_first_run = false;
                                self.coach_response = "✓ Settings saved successfully!\n\n\
                                    Your API key has been configured.\n\n\
                                    You're all set! Click 'HOME' to start your chess training journey.".to_string();
                            }
                            Err(e) => {
                                eprintln!("Failed to save settings: {}", e);
                                self.coach_response = format!("⚠ Error saving settings: {}\n\nPlease try again.", e);
                            }
                        }
                    }
                });

                ui.add_space(10.0);

                ui.vertical_centered(|ui| {
                    ui.label("Settings are saved to .env file in the current directory");
                });
            });
        });
    }

    fn save_settings(&mut self) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = std::fs::File::create(".env")?;
        writeln!(file, "OPENROUTER_API_KEY={}", self.openrouter_key)?;
        writeln!(file, "OPENROUTER_BASE_URL=https://openrouter.ai/api/v1")?;
        Ok(())
    }

    // Helper function for WEBCORE panels
    fn show_xp_panel<R>(
        &mut self,
        ui: &mut egui::Ui,
        title: &str,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        ui.vertical_centered(|ui| {
            let panel_frame = egui::Frame {
                fill: egui::Color32::from_rgb(245, 245, 245),
                stroke: egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 84, 166)), // CHUNKY
                rounding: egui::Rounding::same(8.0),
                inner_margin: egui::Margin::same(20.0),
                outer_margin: egui::Margin::same(10.0),
                shadow: egui::epaint::Shadow {
                    offset: egui::vec2(5.0, 5.0),
                    blur: 12.0,
                    spread: 2.0,
                    color: egui::Color32::from_black_alpha(90),
                },
                ..Default::default()
            };

            panel_frame.show(ui, |ui| {
                ui.set_min_width(900.0);

                // Panel title with banner
                egui::Frame {
                    fill: egui::Color32::from_rgb(0, 84, 166),
                    inner_margin: egui::Margin::symmetric(16.0, 8.0),
                    rounding: egui::Rounding::same(4.0),
                    ..Default::default()
                }.show(ui, |ui| {
                    let title_text = egui::RichText::new(format!("═══ {} ═══", title.to_uppercase()))
                        .size(18.0)
                        .color(egui::Color32::WHITE)
                        .strong();
                    ui.label(title_text);
                });

                ui.add_space(16.0);

                content(ui)
            })
            .inner
        })
        .inner
    }
}
