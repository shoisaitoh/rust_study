use eframe::egui;
use std::time::{Duration, Instant};

struct StopWatch {
    start_time: Option<Instant>,
    elapsed: Duration,
    running: bool,
}

impl StopWatch {
    fn new() -> Self {
        Self {
            start_time: None,
            elapsed: Duration::from_secs(0),
            running: false,
        }
    }

    fn start(&mut self) {
        if !self.running {
            self.start_time = Some(Instant::now() - self.elapsed);
            self.running = true;
        }
    }

    fn stop(&mut self) {
        if self.running {
            self.elapsed = self.elapsed();
            self.running = false;
        }
    }

    fn reset(&mut self) {
        self.elapsed = Duration::from_secs(0);
        self.start_time = None;
        self.running = false;
    }

    fn elapsed(&self) -> Duration {
        if self.running {
            if let Some(start) = self.start_time {
                Instant::now() - start
            } else {
                Duration::from_secs(0)
            }
        } else {
            self.elapsed
        }
    }
}

impl eframe::App for StopWatch {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 時間表示
            let elapsed = self.elapsed();
            let hours = elapsed.as_secs() / 3600;
            let minutes = (elapsed.as_secs() % 3600) / 60;
            let seconds = elapsed.as_secs() % 60;
            let milliseconds = elapsed.subsec_millis();

            ui.vertical_centered(|ui| {
                ui.heading(format!(
                    "{:03}:{:02}:{:02}.{:03}",
                    hours, minutes, seconds, milliseconds
                ));

                ui.add_space(20.0);

                // ボタンの配置
                ui.horizontal(|ui| {
                    if ui.add_sized([160.0, 80.0], egui::Button::new("Start")).clicked() {
                        self.start();
                    }
                    ui.add_space(10.0);
                    if ui.add_sized([160.0, 80.0], egui::Button::new("Stop")).clicked() {
                        self.stop();
                    }
                    ui.add_space(10.0);
                    if ui.add_sized([160.0, 80.0], egui::Button::new("Reset")).clicked() {
                        self.reset();
                    }
                });
            });
            
            // 継続的な更新のために再描画をリクエスト
            ctx.request_repaint();
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ストップウォッチ",
        options,
        Box::new(|_cc| Box::new(StopWatch::new())),
    )
} 