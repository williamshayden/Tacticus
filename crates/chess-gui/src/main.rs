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
        Box::new(|_cc| Ok(Box::<TacticusApp>::default())),
    )
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
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.heading("Tacticus");
                ui.separator();

                ui.selectable_value(&mut self.current_view, AppView::Home, "Home");
                ui.selectable_value(&mut self.current_view, AppView::Play, "Play");
                ui.selectable_value(&mut self.current_view, AppView::Train, "Train");
                ui.selectable_value(&mut self.current_view, AppView::Analyze, "Analyze");
                ui.selectable_value(&mut self.current_view, AppView::Profile, "Profile");
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
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading("Welcome to Tacticus");
            ui.label("AI-Powered Chess Training");
            ui.add_space(20.0);

            // Coach panel
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_width(800.0);
                ui.heading("Your AI Coach");

                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        ui.label(&self.coach_response);
                    });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Ask your coach:");
                    let response = ui.text_edit_singleline(&mut self.user_input);

                    if (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                        || ui.button("Send").clicked()
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
        ui.heading("Play a Game");
        ui.label("Chess board will be displayed here");
        ui.label("This view will show:");
        ui.label("• Interactive chess board");
        ui.label("• Move input");
        ui.label("• Real-time coach hints (if requested)");
        ui.label("• Game analysis after completion");
    }

    fn show_train(&mut self, ui: &mut egui::Ui) {
        ui.heading("Training Exercises");
        ui.label("Personalized training exercises based on your weaknesses");
        ui.label("This view will show:");
        ui.label("• 5-10 exercises tailored to your skill level");
        ui.label("• Hints from your AI coach");
        ui.label("• Detailed explanations after solving");
        ui.label("• Progress tracking");
    }

    fn show_analyze(&mut self, ui: &mut egui::Ui) {
        ui.heading("Analyze Games");
        ui.label("Deep analysis of your recent games with AI coaching");
        ui.label("This view will show:");
        ui.label("• List of your recent games");
        ui.label("• Move-by-move analysis");
        ui.label("• AI coach feedback on your play");
        ui.label("• Identified strengths and weaknesses");
    }

    fn show_profile(&mut self, ui: &mut egui::Ui) {
        ui.heading("Your Chess Profile");
        ui.label("Track your progress and playing style");
        ui.label("This view will show:");
        ui.label("• Current rating and skill level");
        ui.label("• Playing style analysis");
        ui.label("• Win/loss statistics");
        ui.label("• Improvement trends");
        ui.label("• Training history");
    }
}
