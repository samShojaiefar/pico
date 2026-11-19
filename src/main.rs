
use eframe::egui;

struct TestApp;

impl eframe::App for TestApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Keymo 🚀");
            ui.separator();

            ui.label("Emoji Picker is working!");

            ui.add_space(20.0);

            ui.horizontal_wrapped(|ui| {
                for emoji in [
                    "😀", "😂", "😍", "🔥",
                    "🚀", "💻", "🎮", "❤️",
                    "🥹", "😎", "✨", "🎉",
                ] {
                    if ui.button(emoji).clicked() {
                        println!("Selected: {}", emoji);
                    }
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([380.0, 440.0])
            .with_visible(true),

        ..Default::default()
    };

    eframe::run_native(
        "Keymo",
        options,
        Box::new(|_cc| Ok(Box::new(TestApp))),
    )
}
