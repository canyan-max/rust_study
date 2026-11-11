// Release 双击启动时只显示图形窗口；调试版保留终端，便于学习和排错。
// #![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

// mod app;
// mod protocol;
// mod serial_worker;

// fn main() -> eframe::Result<()> {
//     let args: Vec<String> = std::env::args().collect();
//     if args.iter().any(|arg| arg == "--probe") {
//         // 只读一次实机，方便在终端排查连线和固件问题。
//         if let Err(error) = serial_worker::probe("COM14", 9600) {
//             eprintln!("读取失败：{error}");
//             std::process::exit(1);
//         }
//         return Ok(());
//     }
//     let demo = args.iter().any(|arg| arg == "--demo");
//     let auto_connect = args.iter().any(|arg| arg == "--connect") && !demo;
//     let options = eframe::NativeOptions {
//         viewport: eframe::egui::ViewportBuilder::default()
//             .with_inner_size([1080.0, 860.0])
//             .with_min_inner_size([850.0, 680.0]),
//         ..Default::default()
//     };
//     return eframe::run_native(
//         "MPPT 串口监视器",
//         options,
//         Box::new(move |context| Ok(Box::new(app::MonitorApp::new(context, demo, auto_connect)))),
//     );
// }

mod minigrep;
mod tuple_exercises;
#[allow(dead_code)]
fn main() {
    // minigrep::minigrep_start();
    tuple_exercises::tuple_code_start();
}
