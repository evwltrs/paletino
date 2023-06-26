use crate::widgets;
use egui_extras::RetainedImage;
use std::io::Cursor;
use widgets::file::FileDialog;

pub struct App {
    img: Option<RetainedImage>,
    img_offset: egui::Pos2,
    picked_path: Option<String>,
    available_height: f32,
    file: FileDialog,
}

impl Default for App {
    fn default() -> Self {
        Self {
            img: None,
            picked_path: None,
            available_height: 0.0,
            img_offset: egui::pos2(0.0, 0.0),
            file: Default::default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            if ui.button("Open file…").clicked() {
                self.file.open();
            }

            if let Some(file) = self.file.get() {
                // println!("Vec<u8>: {:#?}", file);
                let mut buffer: Vec<u8> = Vec::new();
                let mut writer = Cursor::new(&mut buffer);
                let mut i =
                    image::load_from_memory_with_format(&file, image::ImageFormat::Png).unwrap();


                i = i.resize(1000, 1000, image::imageops::FilterType::Triangle);

                i.write_to(&mut writer, image::ImageFormat::Png).unwrap();

                self.img = None;

                self.img = Some(RetainedImage::from_image_bytes("img", &buffer).unwrap());
            }

            // Get available space for the image
            self.img_offset = ui.cursor().left_top();
            self.available_height = ui.available_height();

            match &self.img {
                Some(i) => ui.image(i.texture_id(ctx), i.size_vec2()),
                _ => ui.label(""),
            };
        });
    }
}