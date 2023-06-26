use egui_extras::RetainedImage;
use std::io::Cursor;

pub struct App {
    img: Option<RetainedImage>,
    img_offset: egui::Pos2,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    available_height: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            img: None,
            picked_path: None,
            available_height: 0.0,
            img_offset: egui::pos2(0.0, 0.0),
            dropped_files: Vec::default(),
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

    fn detect_files_being_dropped(&mut self, ctx: &egui::Context) {
        use egui::*;

        // Preview hovering files:
        if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
            let text = ctx.input(|i| {
                let mut text = "Dropping files:\n".to_owned();
                for file in &i.raw.hovered_files {
                    if let Some(path) = &file.path {
                        text += &format!("\n{}", path.display());

                        self.picked_path = Some(path.as_path().to_string_lossy().to_string());
                    } else if !file.mime.is_empty() {
                        text += &format!("\n{}", file.mime);
                    } else {
                        text += "\n???";
                    }
                }
                text
            });

            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

            let screen_rect = ctx.screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                egui::FontId::monospace(14.0),
                Color32::WHITE,
            );

            // Collect dropped files:
            if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
                self.img = None;
                self.dropped_files = ctx.input(|i| i.raw.dropped_files.clone());
            }
        }
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

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.label("Drop image here");

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };
                        if let Some(bytes) = &file.bytes {
                            info += &format!(" ({} bytes)", bytes.len());
                            if self.img.is_none() {
                                let mut buffer: Vec<u8> = Vec::new();
                                let mut writer = Cursor::new(&mut buffer);

                                let mut i = image::load_from_memory_with_format(
                                    &bytes,
                                    image::ImageFormat::Png,
                                )
                                .unwrap();

                                let h = (self.available_height - 10.0) as u32;

                                i = i.resize(h, h, image::imageops::FilterType::Triangle);

                                i.write_to(&mut writer, image::ImageFormat::Png).unwrap();

                                self.img = None;

                                self.img =
                                    Some(RetainedImage::from_image_bytes("img", &buffer).unwrap());
                            }
                        }
                        ui.label(info);
                    }
                });
            }

            // assure to clean the dropped files list as soon as we have an image. Needed to reload a new, future image.
            if self.img.is_some() {
                self.dropped_files.clear();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            // Get available space for the image
            self.img_offset = ui.cursor().left_top();
            self.available_height = ui.available_height();

            match &self.img {
                Some(i) => ui.image(i.texture_id(ctx), i.size_vec2()),
                _ => ui.label(""),
            };
        });

        self.detect_files_being_dropped(ctx);
    }
}
