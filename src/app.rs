use crate::{median_cut, widgets, Color};
use egui_extras::RetainedImage;
use std::io::Cursor;
use widgets::file::FileDialog;

pub struct App {
    img: Option<RetainedImage>,
    file: FileDialog,
    palette: Vec<Color>,
    color_count: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            img: (None),
            file: (Default::default()),
            palette: (Default::default()),
            color_count: 6,
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("Open file…").clicked() {
                    self.file.open();
                }
                ui.label("Number of colors:");
                let response = ui.add(egui::Slider::new(&mut self.color_count, 0..=20));
                response.on_hover_text("Drag me!");
            });

            if let Some(file) = self.file.get() {
                let mut buffer: Vec<u8> = Vec::new();
                let mut writer = Cursor::new(&mut buffer);
                let mut i = image::load_from_memory(&file).unwrap();

                i = i.resize(500, 500, image::imageops::FilterType::Triangle);

                i.write_to(&mut writer, image::ImageFormat::Png).unwrap();

                self.img = None;

                self.img = Some(RetainedImage::from_image_bytes("img", &buffer).unwrap());

                self.palette = median_cut(&i, self.color_count);
            }

            match &self.img {
                Some(i) => ui.image(i.texture_id(ctx), i.size_vec2()),
                _ => ui.label(""),
            };

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                for color in &self.palette {
                    ui.color_edit_button_srgb(&mut [color.r, color.g, color.b]);
                }
            });
        });
    }
}
