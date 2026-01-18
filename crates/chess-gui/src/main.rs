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
            // Apply Windows XP theme
            apply_xp_theme(&cc.egui_ctx);
            Ok(Box::<TacticusApp>::default())
        }),
    )
}

fn apply_xp_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Windows XP Luna Blue color scheme
    let xp_blue = egui::Color32::from_rgb(0, 84, 166);        // Title bar blue
    let xp_blue_light = egui::Color32::from_rgb(49, 106, 197); // Lighter blue
    let xp_bg = egui::Color32::from_rgb(236, 233, 216);        // Classic beige background
    let xp_button = egui::Color32::from_rgb(236, 233, 216);    // Button face
    let xp_button_hover = egui::Color32::from_rgb(255, 240, 207); // Button hover
    let xp_border = egui::Color32::from_rgb(0, 60, 116);       // Dark blue border
    let xp_text = egui::Color32::from_rgb(0, 0, 0);            // Black text

    // Set spacing - Windows XP had generous spacing
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(8.0);
    style.spacing.menu_margin = egui::Margin::same(6.0);

    // Rounded corners - Windows XP style
    style.visuals.window_rounding = egui::Rounding::same(6.0);
    style.visuals.menu_rounding = egui::Rounding::same(4.0);

    // Window styling
    style.visuals.window_fill = xp_bg;
    style.visuals.window_stroke = egui::Stroke::new(1.0, xp_border);
    style.visuals.window_shadow = egui::epaint::Shadow {
        offset: egui::vec2(4.0, 4.0),
        blur: 8.0,
        spread: 0.0,
        color: egui::Color32::from_black_alpha(60),
    };

    // Panel background
    style.visuals.panel_fill = xp_bg;

    // Button styling - classic XP 3D look
    style.visuals.widgets.noninteractive.bg_fill = xp_button;
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, xp_border);

    style.visuals.widgets.inactive.bg_fill = xp_button;
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, xp_border);
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, xp_text);

    style.visuals.widgets.hovered.bg_fill = xp_button_hover;
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.5, xp_blue);
    style.visuals.widgets.hovered.expansion = 1.0;

    style.visuals.widgets.active.bg_fill = xp_blue_light;
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.5, xp_border);
    style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

    // Selection colors - Windows XP blue highlight
    style.visuals.selection.bg_fill = xp_blue;
    style.visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

    // Hyperlink color
    style.visuals.hyperlink_color = xp_blue;

    // Text color
    style.visuals.override_text_color = Some(xp_text);

    // Extreme background (for contrast)
    style.visuals.extreme_bg_color = egui::Color32::from_rgb(255, 255, 255);

    ctx.set_style(style);
}

struct TacticusApp {
    // Core state
    user_input: String,
    coach_response: String,

    // Application state
    current_view: AppView,
}

#[derive(Default, PartialEq)]
enum AppView {
    #[default]
    Home,
    Play,
    Train,
    Analyze,
    Profile,
}

impl Default for TacticusApp {
    fn default() -> Self {
        Self {
            user_input: String::new(),
            coach_response: "Welcome to Tacticus! Your AI Chess Coach is ready to help you improve.\n\nI'll analyze your games, provide personalized training exercises, and help you understand your unique playing style.\n\nLet's start by playing a game or reviewing one of your recent matches!".to_string(),
            current_view: AppView::Home,
        }
    }
}

impl eframe::App for TacticusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Windows XP-style title bar
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame {
                fill: egui::Color32::from_rgb(0, 84, 166), // XP blue
                inner_margin: egui::Margin::symmetric(12.0, 8.0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

                    ui.heading("Tacticus - AI Chess Trainer");
                    ui.add_space(20.0);

                    // Windows XP style tabs
                    let button_style = ui.style_mut();
                    button_style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(49, 106, 197);
                    button_style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(100, 140, 210);
                    button_style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(236, 233, 216);

                    if ui.selectable_label(self.current_view == AppView::Home, "Home").clicked() {
                        self.current_view = AppView::Home;
                    }
                    if ui.selectable_label(self.current_view == AppView::Play, "Play").clicked() {
                        self.current_view = AppView::Play;
                    }
                    if ui.selectable_label(self.current_view == AppView::Train, "Train").clicked() {
                        self.current_view = AppView::Train;
                    }
                    if ui.selectable_label(self.current_view == AppView::Analyze, "Analyze").clicked() {
                        self.current_view = AppView::Analyze;
                    }
                    if ui.selectable_label(self.current_view == AppView::Profile, "Profile").clicked() {
                        self.current_view = AppView::Profile;
                    }
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
            }
        });
    }
}

impl TacticusApp {
    fn show_home(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);

        ui.vertical_centered(|ui| {
            // Windows XP-style welcome header
            let header_frame = egui::Frame {
                fill: egui::Color32::from_rgb(255, 255, 255),
                stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 60, 116)),
                rounding: egui::Rounding::same(5.0),
                inner_margin: egui::Margin::same(16.0),
                ..Default::default()
            };

            header_frame.show(ui, |ui| {
                ui.set_min_width(900.0);
                ui.vertical_centered(|ui| {
                    let title_text = egui::RichText::new("Welcome to Tacticus")
                        .size(24.0)
                        .color(egui::Color32::from_rgb(0, 84, 166));
                    ui.label(title_text);

                    ui.add_space(4.0);
                    ui.label("AI-Powered Chess Training");
                });
            });

            ui.add_space(20.0);

            // Coach panel - Windows XP GroupBox style
            let coach_frame = egui::Frame {
                fill: egui::Color32::from_rgb(245, 245, 245),
                stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(160, 160, 160)),
                rounding: egui::Rounding::same(4.0),
                inner_margin: egui::Margin::same(12.0),
                outer_margin: egui::Margin::same(8.0),
                ..Default::default()
            };

            coach_frame.show(ui, |ui| {
                ui.set_min_width(900.0);

                // Group box title
                let title = egui::RichText::new("Your AI Coach")
                    .size(16.0)
                    .color(egui::Color32::from_rgb(0, 84, 166))
                    .strong();
                ui.label(title);

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Response area with white background
                egui::Frame {
                    fill: egui::Color32::WHITE,
                    stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(128, 128, 128)),
                    inner_margin: egui::Margin::same(10.0),
                    rounding: egui::Rounding::same(2.0),
                    ..Default::default()
                }
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            ui.label(&self.coach_response);
                        });
                });

                ui.add_space(12.0);

                // Input area
                ui.horizontal(|ui| {
                    ui.label("Ask your coach:");
                    ui.add_space(8.0);

                    let text_edit = egui::TextEdit::singleline(&mut self.user_input)
                        .desired_width(600.0);
                    let response = ui.add(text_edit);

                    ui.add_space(8.0);

                    // Windows XP default button style
                    let send_button = egui::Button::new("Send")
                        .min_size(egui::vec2(80.0, 24.0));

                    if ui.add(send_button).clicked()
                        || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        if !self.user_input.is_empty() {
                            // TODO: Call LLM agent
                            self.coach_response = format!(
                                "You asked: {}\n\n[LLM Response will appear here once integrated]",
                                self.user_input
                            );
                            self.user_input.clear();
                        }
                    }
                });
            });
        });
    }

    fn show_play(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        self.show_xp_panel(ui, "Play a Game", |ui| {
            ui.label("Chess board will be displayed here");
            ui.add_space(10.0);
            ui.label("This view will show:");
            ui.label("• Interactive chess board");
            ui.label("• Move input");
            ui.label("• Real-time coach hints (if requested)");
            ui.label("• Game analysis after completion");
        });
    }

    fn show_train(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        self.show_xp_panel(ui, "Training Exercises", |ui| {
            ui.label("Personalized training exercises based on your weaknesses");
            ui.add_space(10.0);
            ui.label("This view will show:");
            ui.label("• 5-10 exercises tailored to your skill level");
            ui.label("• Hints from your AI coach");
            ui.label("• Detailed explanations after solving");
            ui.label("• Progress tracking");
        });
    }

    fn show_analyze(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        self.show_xp_panel(ui, "Analyze Games", |ui| {
            ui.label("Deep analysis of your recent games with AI coaching");
            ui.add_space(10.0);
            ui.label("This view will show:");
            ui.label("• List of your recent games");
            ui.label("• Move-by-move analysis");
            ui.label("• AI coach feedback on your play");
            ui.label("• Identified strengths and weaknesses");
        });
    }

    fn show_profile(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        self.show_xp_panel(ui, "Your Chess Profile", |ui| {
            ui.label("Track your progress and playing style");
            ui.add_space(10.0);
            ui.label("This view will show:");
            ui.label("• Current rating and skill level");
            ui.label("• Playing style analysis");
            ui.label("• Win/loss statistics");
            ui.label("• Improvement trends");
            ui.label("• Training history");
        });
    }

    // Helper function for Windows XP-style panels
    fn show_xp_panel<R>(
        &mut self,
        ui: &mut egui::Ui,
        title: &str,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        ui.vertical_centered(|ui| {
            let panel_frame = egui::Frame {
                fill: egui::Color32::from_rgb(245, 245, 245),
                stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(160, 160, 160)),
                rounding: egui::Rounding::same(4.0),
                inner_margin: egui::Margin::same(16.0),
                outer_margin: egui::Margin::same(8.0),
                ..Default::default()
            };

            panel_frame.show(ui, |ui| {
                ui.set_min_width(900.0);

                // Panel title
                let title_text = egui::RichText::new(title)
                    .size(16.0)
                    .color(egui::Color32::from_rgb(0, 84, 166))
                    .strong();
                ui.label(title_text);

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(12.0);

                content(ui)
            })
            .inner
        })
        .inner
    }
}
