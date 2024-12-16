use strum::IntoEnumIterator;
use tokio_serial::SerialStream;

use crate::serial::{list_serial_ports, open_serial_port, Baudrate};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)]
    value: f32,

    serial_port: String,
    baudrate: Baudrate,

    #[serde(skip)]
    serial: Option<SerialStream>,
    serial_output: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            serial_port: "".to_owned(),
            baudrate: Baudrate::Baud115200,
            serial: None,
            serial_output: "".to_owned(),
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
                ui.vertical(|ui| {
                    ui.label("终端窗口");
                    ui.text_edit_multiline(&mut self.serial_output);
                });

                // 命令编写窗口
                ui.vertical(|ui| {
                    ui.label("命令编写");
                    ui.text_edit_multiline(&mut self.label);
                });
            });
        });

        egui::TopBottomPanel::bottom("config_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 串口选择
                ui.label("串口:");
                if let Ok(ports) = list_serial_ports() {
                    egui::ComboBox::new("Combo_port", "")
                        .selected_text(format!("{}", self.serial_port))
                        .show_ui(ui, |ui| {
                            for port in ports {
                                ui.selectable_value(
                                    &mut self.serial_port,
                                    port.port_name.clone(),
                                    &port.port_name,
                                );
                            }
                        });
                } else {
                    ui.label("无法获取串口信息");
                }

                // 波特率设置
                ui.label("波特率:");
                egui::ComboBox::new("Combo_rate", "")
                    .selected_text(format!("{}", self.baudrate))
                    .show_ui(ui, |ui| {
                        for rate in Baudrate::iter() {
                            ui.selectable_value(
                                &mut self.baudrate,
                                rate,
                                format!("{}", rate),
                            );
                        }
                    });

                // 连接按钮
                if ui.button("连接").clicked() {
                    if let Ok(serial) = open_serial_port(&self.serial_port, self.baudrate) {
                        self.serial = Some(serial);
                    }
                }
            });
        });
    }
}
