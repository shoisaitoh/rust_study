use eframe::egui;  // GUIフレームワークのインポート
use std::sync::{Arc, Mutex};  // スレッド間でデータを共有するための型
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};  // 音声処理のトレイト
use ringbuf::HeapRb;  // リングバッファの実装

// アプリケーションの状態を管理する構造体
struct AudioRecorder {
    recording: bool,  // 録音中かどうかのフラグ
    paused: bool,    // 一時停止中かどうかのフラグ
    stream: Option<cpal::Stream>,  // 録音ストリーム
    samples: Arc<Mutex<Vec<f32>>>,  // 録音されたサンプルデータ
}

impl Default for AudioRecorder {
    // 構造体のデフォルト値を設定するトレイトの実装
    fn default() -> Self {
        Self {
            recording: false,
            paused: false,
            stream: None,
            samples: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl AudioRecorder {
    // 録音を開始するメソッド
    fn start_recording(&mut self) -> Result<(), anyhow::Error> {
        let host = cpal::default_host();  // デフォルトのオーディオホストを取得
        let device = host.default_input_device()  // デフォルトの入力デバイスを取得
            .ok_or_else(|| anyhow::Error::msg("入力デバイスが見つかりません"))?;

        // 入力デバイスの設定
        let config = device.default_input_config()?;
        let samples = Arc::clone(&self.samples);

        // 録音ストリームを作成
        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut samples = samples.lock().unwrap();
                let mut rb = HeapRb::<f32>::new(44100 * 5);
                samples.extend_from_slice(data);
            },
            |err| eprintln!("録音エラー: {}", err),
            None,
        )?;

        stream.play()?;  // ストリームを開始
        self.stream = Some(stream);
        self.recording = true;
        self.paused = false;

        Ok(())
    }

    // 録音を停止するメソッド
    fn stop_recording(&mut self) {
        self.stream = None;  // ストリームをドロップして録音を停止
        self.recording = false;
        self.paused = false;

        // WAVファイルとして保存
        if let Ok(mut samples) = self.samples.lock() {
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: 44100,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };

            if let Ok(mut writer) = hound::WavWriter::create("recording.wav", spec) {
                for &sample in samples.iter() {
                    writer.write_sample(sample).unwrap_or_default();
                }
                samples.clear();
            }
        }
    }

    // 録音を一時停止するメソッド
    fn toggle_pause(&mut self) {
        if let Some(stream) = &self.stream {
            if self.paused {
                stream.play().unwrap_or_default();
            } else {
                stream.pause().unwrap_or_default();
            }
            self.paused = !self.paused;
        }
    }
}

impl eframe::App for AudioRecorder {
    // GUIの更新処理
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("音声録音アプリ");
            ui.add_space(20.0);

            if !self.recording {
                if ui.button("🎤 録音開始").clicked() {
                    if let Err(err) = self.start_recording() {
                        eprintln!("録音開始エラー: {}", err);
                    }
                }
            } else {
                let pause_text = if self.paused { "⏵ 再開" } else { "⏸ 一時停止" };
                if ui.button(pause_text).clicked() {
                    self.toggle_pause();
                }

                if ui.button("⏹ 録音終了").clicked() {
                    self.stop_recording();
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    // アプリケーションの設定
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 200.0]),
        ..Default::default()
    };

    // アプリケーションの起動
    eframe::run_native(
        "音声録音アプリ",
        options,
        Box::new(|_cc| Box::new(AudioRecorder::default())),
    )
}
