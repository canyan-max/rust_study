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

#[allow(dead_code)]
mod exercises;
#[allow(dead_code)]
mod minigrep;
struct Config {
    port_name: String,
    buard: u32,
}
fn parges_args(args: &[String]) -> Result<Config, String> {
    if args.len() != 3 {
        return Err("example => serial_rust.exe COM11 115200".to_string());
    }
    let port_name: &str = match args.get(1) {
        Some(name) => name.as_str(),
        None => {
            eprintln!("请填写串口名，例如 COM14");
            return Err("comm port error".to_string());
        }
    };
    let baud_text: &str = match args.get(2) {
        Some(text) => text.as_str(),
        None => {
            return Err("buard error".to_string());
        }
    };
    let baud_rate: u32 = match baud_text.parse::<u32>() {
        Ok(value) => value,
        Err(error) => {
            return Err(format!("波特率格式错误：{}", error));
        }
    };
    let cfgs: Config = Config {
        port_name: port_name.to_string(),
        buard: baud_rate,
    };
    Ok(cfgs)
}

fn main() {
    // minigrep::minigrep_start();
    // exercises::tuple_exercises::tuple_code_start();
    // exercises::array_slice::array_slice_code_start();
    // exercises::struct_enmu_const::struct_run();
    // exercises::struct_enmu_const::enum_run();
    // exercises::struct_enmu_const::constant_run();
    // exercises::unpack::unpack_run();
    // exercises::serial_io::serial_run("COM14", 9600);
    // exercises::serial_io::thread_demo()
    let args: Vec<String> = std::env::args().collect();
    match parges_args(&args) {
        Err(error) => {
            println!("{}", error);
            return;
        }
        Ok(value) => {
            exercises::serial_io::serial_thread_run(&value.port_name, value.buard);
        }
    }
}
#[cfg(test)]
mod tests {
    use std::panic;

    // use super::Config;
    use super::parges_args;
    #[test]
    fn test_parse_args_ok() {
        let args: [String; 3] = ["--".to_string(), "COM14".to_string(), "9600".to_string()];
        match parges_args(&args) {
            Err(error) => {
                panic!("合法参数解析失败：{}", error);
            }
            Ok(value) => {
                assert_eq!(value.port_name, "COM14".to_string());
                assert_eq!(value.buard, 9600);
            }
        }
    }
    #[test]
    fn test_parse_buard_is_error() {
        let args: [String; 3] = ["--".to_string(), "COM14".to_string(), "abc".to_string()];
        let result = parges_args(&args);
        assert!(result.is_err());
    }
}
