//! 界面只保存显示状态，串口收发交给 serial_worker。

use crate::protocol::{self, REGISTERS, Snapshot};
use crate::serial_worker::{Command, Event, Worker};
use eframe::egui::{self, Color32, RichText};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const GREEN: Color32 = Color32::from_rgb(21, 128, 105);
const MUTED: Color32 = Color32::from_rgb(107, 119, 135);
const RED: Color32 = Color32::from_rgb(186, 51, 59);

pub struct MonitorApp {
    worker: Worker,
    ports: Vec<String>,
    selected_port: String,
    baud: u32,
    connected: bool,
    connecting: bool,
    auto: bool,
    busy: bool,
    status: String,
    last_error: Option<String>,
    snapshot: Option<Snapshot>,
    updated: Option<Instant>,
    success: u64,
    failures: u64,
    logs: VecDeque<(String, &'static str, String)>,
    started: Instant,
    demo: bool,
    #[cfg(feature = "screenshot")]
    screenshot_requested: bool,
}

impl MonitorApp {
    pub fn new(context: &eframe::CreationContext<'_>, demo: bool, auto_connect: bool) -> Self {
        setup_style(&context.egui_ctx);
        let worker = Worker::start(context.egui_ctx.clone());
        let mut app = Self {
            worker,
            ports: Vec::new(),
            selected_port: "COM14".into(),
            baud: 9600,
            connected: false,
            connecting: false,
            auto: true,
            busy: false,
            status: "未连接".into(),
            last_error: None,
            snapshot: None,
            updated: None,
            success: 0,
            failures: 0,
            logs: VecDeque::new(),
            started: Instant::now(),
            demo,
            #[cfg(feature = "screenshot")]
            screenshot_requested: false,
        };
        app.send(Command::RefreshPorts);
        if demo {
            app.snapshot = Some(Snapshot {
                registers: [
                    24567,
                    1234,
                    12880,
                    (-320i16) as u16,
                    12880,
                    1554,
                    256,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            });
            app.status = "演示模式 · 示例数据，未连接实机".into();
            app.updated = Some(Instant::now());
            app.log("DEMO", "示例数据只用于查看界面，不代表实机测量值".into());
        }
        if auto_connect {
            app.connecting = true;
            app.send(Command::Connect {
                name: "COM14".into(),
                baud: 9600,
            });
        }
        return app;
    }

    fn send(&self, command: Command) {
        let _ = self.worker.commands.send(command);
    }

    fn log(&mut self, direction: &'static str, message: String) {
        let elapsed = self.started.elapsed();
        let time = format!(
            "{:02}:{:02}.{:03}",
            elapsed.as_secs() / 60,
            elapsed.as_secs() % 60,
            elapsed.subsec_millis()
        );
        self.logs.push_back((time, direction, message));
        while self.logs.len() > 200 {
            self.logs.pop_front();
        }
    }

    fn receive_events(&mut self) {
        while let Ok(event) = self.worker.events.try_recv() {
            match event {
                Event::Ports(mut ports) => {
                    ports.sort();
                    self.ports = ports;
                }
                Event::Connected(message) => {
                    self.connected = true;
                    self.connecting = false;
                    self.status = format!("已连接 {message}");
                    self.last_error = None;
                    self.log("INFO", self.status.clone());
                }
                Event::Disconnected => {
                    self.connected = false;
                    self.connecting = false;
                    self.busy = false;
                    self.status = "未连接 · 历史数据不再刷新".into();
                    self.log("INFO", "串口已断开".into());
                }
                Event::Data(snapshot) => {
                    self.snapshot = Some(snapshot);
                    self.updated = Some(Instant::now());
                    self.last_error = None;
                    self.busy = false;
                    self.success += 1;
                }
                Event::Error(error) => {
                    self.busy = false;
                    self.failures += 1;
                    self.last_error = Some(error.clone());
                    self.log("ERR", error);
                }
                Event::Log(direction, message) => {
                    if direction == "TX" {
                        self.busy = true;
                    }
                    self.log(direction, message);
                }
            }
        }
    }

    fn connection_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.add_enabled_ui(!self.connected && !self.connecting, |ui| {
                ui.label("串口");
                egui::ComboBox::from_id_salt("port_list")
                    .selected_text(&self.selected_port)
                    .width(115.0)
                    .show_ui(ui, |ui| {
                        for port in &self.ports {
                            ui.selectable_value(&mut self.selected_port, port.clone(), port);
                        }
                    });
                if ui.button("刷新端口").clicked() {
                    self.send(Command::RefreshPorts);
                }
                ui.label("波特率");
                ui.add(
                    egui::DragValue::new(&mut self.baud)
                        .range(1200..=115200)
                        .speed(100),
                );
                ui.label(RichText::new("8N1 · 无流控").color(MUTED));
            });
            if self.connected {
                if ui.button("断开连接").clicked() {
                    self.send(Command::Disconnect);
                    self.connecting = true;
                }
            } else if ui
                .add_enabled(
                    !self.connecting && !self.demo,
                    egui::Button::new("连接设备")
                        .fill(GREEN)
                        .stroke(egui::Stroke::NONE),
                )
                .on_hover_text("打开串口；默认开启自动读取")
                .clicked()
            {
                self.connecting = true;
                self.snapshot = None;
                self.updated = None;
                self.last_error = None;
                self.success = 0;
                self.failures = 0;
                self.send(Command::SetAuto(self.auto));
                self.send(Command::Connect {
                    name: self.selected_port.clone(),
                    baud: self.baud,
                });
            }
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            let color = if self.connected { GREEN } else { MUTED };
            ui.colored_label(color, &self.status);
            if self.connecting || self.busy {
                ui.spinner();
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_enabled(
                        self.connected && !self.busy && !self.connecting,
                        egui::Button::new("读取一次"),
                    )
                    .clicked()
                {
                    self.busy = true;
                    self.send(Command::ReadOnce);
                }
                if ui
                    .checkbox(&mut self.auto, "自动刷新（间隔 1 秒）")
                    .changed()
                {
                    self.send(Command::SetAuto(self.auto));
                }
            });
        });
    }

    fn cards(&self, ui: &mut egui::Ui) {
        ui.columns(4, |columns| {
            for (column, index) in columns.iter_mut().zip([0, 2, 3, 6]) {
                egui::Frame::group(column.style())
                    .fill(Color32::WHITE)
                    .inner_margin(14.0)
                    .show(column, |ui| {
                        ui.set_min_width((ui.available_width() - 2.0).max(0.0));
                        ui.label(RichText::new(REGISTERS[index].name).color(MUTED));
                        let value = self
                            .snapshot
                            .as_ref()
                            .map(|snapshot| snapshot.display_value(index))
                            .unwrap_or_else(|| "—".into());
                        ui.label(RichText::new(value).size(25.0).color(GREEN));
                    });
            }
        });
    }

    fn register_table(&self, ui: &mut egui::Ui) {
        ui.heading("寄存器数据");
        ui.label(
            RichText::new("电流单位已换算为 A，电压为 V；悬停数值查看字段说明。").color(MUTED),
        );
        ui.add_space(6.0);
        egui::Grid::new("registers")
            .num_columns(5)
            .striped(true)
            .min_col_width(90.0)
            .spacing([16.0, 7.0])
            .show(ui, |ui| {
                for title in ["地址", "名称", "显示值", "原始整数", "原始 HEX"] {
                    ui.strong(title);
                }
                ui.end_row();
                for (index, info) in REGISTERS.iter().enumerate() {
                    ui.monospace(format!("0x{:04X}", protocol::FIRST_REGISTER + index as u16));
                    ui.label(info.name);
                    let display = self
                        .snapshot
                        .as_ref()
                        .map(|snapshot| snapshot.display_value(index))
                        .unwrap_or_else(|| "—".into());
                    ui.colored_label(if info.pending { MUTED } else { GREEN }, display)
                        .on_hover_text(info.note);
                    if let Some(snapshot) = &self.snapshot {
                        ui.monospace(snapshot.raw_value(index).to_string());
                        ui.monospace(format!("{:04X}", snapshot.registers[index]));
                    } else {
                        ui.label("—");
                        ui.label("—");
                    }
                    ui.end_row();
                }
            });
    }
}

impl eframe::App for MonitorApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(feature = "screenshot")]
        self.capture_for_check(context);
        self.receive_events();
        context.request_repaint_after(Duration::from_millis(150));
        egui::CentralPanel::default().show(context, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("MPPT 串口监视器").size(27.0));
                    ui.label(RichText::new("MODBUS RTU · 只读").color(MUTED));
                });
                ui.label(
                    RichText::new(
                        "从站 21 / 0x15    ·    功能码 03    ·    0x0008—0x0015 共 14 项",
                    )
                    .color(MUTED),
                );
                ui.add_space(12.0);
                egui::Frame::group(ui.style())
                    .fill(Color32::WHITE)
                    .inner_margin(12.0)
                    .show(ui, |ui| self.connection_controls(ui));
                ui.add_space(10.0);
                if self.demo {
                    ui.colored_label(
                        Color32::from_rgb(163, 105, 21),
                        "演示模式：下方是示例数据，不连接串口。",
                    );
                }
                if let Some(error) = &self.last_error {
                    ui.colored_label(RED, error);
                    ui.colored_label(RED, "下方保留最近一次有效值，不代表当前实时数据。");
                }
                ui.horizontal(|ui| {
                    let age = self
                        .updated
                        .map(|time| {
                            format!("最近有效数据：{:.1} 秒前", time.elapsed().as_secs_f32())
                        })
                        .unwrap_or_else(|| "尚未收到有效数据".into());
                    ui.label(RichText::new(age).color(MUTED));
                    ui.label(
                        RichText::new(format!("成功 {}    失败 {}", self.success, self.failures))
                            .color(MUTED),
                    );
                });
                ui.add_space(8.0);
                self.cards(ui);
                ui.add_space(12.0);
                self.register_table(ui);
                ui.add_space(8.0);
                ui.label(
                    RichText::new(
                        "注意：SOC / 功率 / 日电量 / 状态是固件占位字段；原始 0 不表示状态正常。",
                    )
                    .color(MUTED),
                );
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.heading("收发日志");
                    ui.label(RichText::new("HEX · 保留最近 200 条").color(MUTED));
                    if ui.small_button("清空").clicked() {
                        self.logs.clear();
                    }
                });
                egui::Frame::group(ui.style())
                    .fill(Color32::from_rgb(242, 245, 248))
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("logs")
                            .max_height(135.0)
                            .min_scrolled_height(65.0)
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                if self.logs.is_empty() {
                                    ui.label(
                                        RichText::new("连接设备后，显示请求与应答原始字节。")
                                            .color(MUTED),
                                    );
                                }
                                for (time, direction, message) in &self.logs {
                                    let color = match *direction {
                                        "ERR" => RED,
                                        "TX" => GREEN,
                                        _ => MUTED,
                                    };
                                    ui.label(
                                        RichText::new(format!("{time}  {direction:4}  {message}"))
                                            .monospace()
                                            .size(12.0)
                                            .color(color),
                                    );
                                }
                            });
                    });
            });
        });
    }
}

#[cfg(feature = "screenshot")]
impl MonitorApp {
    // 使用框架截图事件做渲染检查，不依赖桌面点击或截屏脚本。
    fn capture_for_check(&mut self, context: &egui::Context) {
        let Ok(path) = std::env::var("MPPT_SCREENSHOT_TO") else {
            return;
        };
        for event in context.input(|input| input.events.clone()) {
            if let egui::Event::Screenshot { image, .. } = event {
                let bytes: Vec<u8> = image
                    .pixels
                    .iter()
                    .flat_map(|pixel| pixel.to_array())
                    .collect();
                image::save_buffer(
                    &path,
                    &bytes,
                    image.size[0] as u32,
                    image.size[1] as u32,
                    image::ColorType::Rgba8,
                )
                .expect("保存界面截图失败");
                context.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        if !self.screenshot_requested && self.started.elapsed() > Duration::from_secs(2) {
            self.screenshot_requested = true;
            context.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
        }
    }
}

fn setup_style(context: &egui::Context) {
    context.set_visuals(egui::Visuals::light());
    let mut style = (*context.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 6.0);
    style.visuals.panel_fill = Color32::from_rgb(247, 249, 252);
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(15.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(15.0));
    context.set_style(style);

    // 从 Windows 字体目录加载中文字体，避免界面显示成方框。
    let windows = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".into());
    let mut fonts = egui::FontDefinitions::default();
    for filename in ["msyh.ttc", "simhei.ttf"] {
        let path = std::path::Path::new(&windows).join("Fonts").join(filename);
        if let Ok(bytes) = std::fs::read(path) {
            fonts
                .font_data
                .insert("chinese".into(), egui::FontData::from_owned(bytes).into());
            for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
                fonts
                    .families
                    .entry(family)
                    .or_default()
                    .push("chinese".into());
            }
            break;
        }
    }
    context.set_fonts(fonts);
}
