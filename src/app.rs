/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    pub fn set_default_font(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "SourceHanSansSC".to_string(),
            egui::FontData::from_static(include_bytes!("../assets/SourceHanSansSC-Light.otf")),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "SourceHanSansSC".to_string());
        ctx.set_fonts(fonts);
    }

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Self::set_default_font(&cc.egui_ctx);
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 终端窗口
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("终端输出...");
                    ui.text_edit_multiline(&mut self.label);
                });

                // 命令编写窗口
                ui.vertical(|ui| {
                    ui.label("命令编写");
                    ui.text_edit_multiline(&mut self.label); // 用于输入命令
                    if ui.button("发送命令").clicked() {
                        // 处理命令发送逻辑
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("config_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 串口选择
                ui.label("选择串口:");
                egui::ComboBox::from_label("")
                    .selected_text("9600") // 默认选择
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.label, "COM1".to_string(), "COM1");
                        ui.selectable_value(&mut self.label, "COM2".to_string(), "COM2");
                        // 添加更多串口选项
                    });

                // 波特率设置
                ui.label("波特率:");
                ui.add(
                    egui::DragValue::new(&mut self.value)
                        .speed(1.0)
                        .clamp_range(300..=115200),
                );
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
