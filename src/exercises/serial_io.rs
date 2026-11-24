use super::protocol_recover::Measurement;
// use super::protocol_recover::ProtocolError;
use super::protocol_recover::protocol_recover_pack;
use super::protocol_recover::protocol_recover_process;
use crossterm::style::Stylize;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::io::Write;
use std::time::{Duration, Instant};
struct ReceiveStats {
    bytes_count: usize,
    measurements_count: usize,
}
enum PortMenuAction {
    Selected(usize),
    Refresh,
    Cancel,
}
impl ReceiveStats {
    fn new(bytes_count: usize, measurements_count: usize) -> Self {
        Self {
            bytes_count,
            measurements_count,
        }
    }
    fn recoder(&mut self, byte_tick: usize, mem_size: usize) {
        self.bytes_count += byte_tick;
        self.measurements_count += mem_size;
    }
}

fn open_recoder(path: &str) -> Result<std::fs::File, std::io::Error> {
    let mut files = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    let metdata = files.metadata()?;
    if 0 == metdata.len() {
        writeln!(&mut files, "voltage current")?;
    }
    Ok(files)
}

pub fn serial_run(port_name: &str, buard: u32) {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = match serialport::new(port_name, buard)
        .timeout(Duration::from_millis(100))
        .open()
    {
        Ok(valid) => valid,
        Err(error) => {
            eprintln!("open serial port :{} error reason :{}", port_name, error);
            return;
        }
    };

    let test_var: Measurement = Measurement::new(2500, -100);
    let content: [u8; 4] = test_var.to_payload();
    let output: Vec<u8> = match protocol_recover_pack(&content) {
        Ok(frame) => {
            println!("{:02x?}", frame);
            frame
        }
        Err(_) => {
            return;
        }
    };

    match port.write_all(&output) {
        Ok(()) => {
            println!("send ok");
        }
        Err(error) => {
            println!("send error {}", error);
        }
    }

    let mut serial_buf: Vec<u8> = vec![0; 32];
    let mut recevice_buffer: Vec<u8> = Vec::new();
    // let mut latest: Option<Measurement> = None;
    let mut recoder: ReceiveStats = ReceiveStats {
        bytes_count: 0,
        measurements_count: 0,
    };
    let mut last_report = Instant::now();
    let mut file = match open_recoder("measurements.csv") {
        Err(error) => {
            eprintln!("open file error reasion:{}", error);
            return;
        }
        Ok(valid) => {
            match valid.metadata() {
                Ok(info) => println!("size={} 字节", info.len()),
                Err(error) => eprintln!("读取文件信息失败：{}", error),
            }
            valid
        }
    };
    loop {
        if last_report.elapsed() >= Duration::from_secs(5) {
            println!("recevice_byte size :{}", recoder.bytes_count);
            println!("recevice counter :{}", recoder.measurements_count);
            last_report = Instant::now();
        }
        match port.read(serial_buf.as_mut_slice()) {
            Ok(count) => {
                println!("收到：{:02X?}", &serial_buf[..count]);
                recevice_buffer.extend_from_slice(&serial_buf[..count]);
                let measurements: Vec<Measurement> = protocol_recover_process(&mut recevice_buffer);
                for measurement in &measurements {
                    match writeln!(
                        &mut file,
                        "{},{}",
                        measurement.get_vol_mv(),
                        measurement.get_cur_ma()
                    ) {
                        Ok(()) => {
                            println!("记录成功");
                        }
                        Err(error) => {
                            eprintln!("写入记录失败：{}", error);
                            return;
                        }
                    }
                }
                recoder.recoder(count, measurements.len());
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::TimedOut {
                    continue;
                }
                eprintln!("串口读取失败：{}", error);
                break;
            }
        }
    }
}

fn serial_slect_port_memu(port_buffer: &[serialport::SerialPortInfo]) -> PortMenuAction {
    let mut select: usize = 0;

    if port_buffer.is_empty() {
        println!("未找到串口，插入设备后按 R 重试。");
    }
    for (index, port) in port_buffer.iter().enumerate() {
        if select == index {
            println!(
                "{}",
                format!("====>>>port_name:{} port_index:{}", port.port_name, index).green()
            );
        } else {
            println!("port_name:{} port_index:{}", port.port_name, index);
        }
    }
    loop {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up if !port_buffer.is_empty() => {
                        if select == 0 {
                            select = 0;
                        } else {
                            select -= 1;
                        }
                        if let Err(error) =
                            execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0))
                        {
                            eprintln!("清屏失败：{error}");
                        }
                        for (index, port) in port_buffer.iter().enumerate() {
                            if select == index {
                                println!(
                                    "{}",
                                    format!(
                                        "====>>>port_name:{} port_index:{}",
                                        port.port_name, index
                                    )
                                    .green()
                                );
                            } else {
                                println!("port_name:{} port_index:{}", port.port_name, index);
                            }
                        }
                    }
                    KeyCode::Down if !port_buffer.is_empty() => {
                        if select + 1 < port_buffer.len() {
                            select += 1;
                        }

                        if let Err(error) =
                            execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0))
                        {
                            eprintln!("清屏失败：{error}");
                        }
                        for (index, port) in port_buffer.iter().enumerate() {
                            if select == index {
                                println!(
                                    "{}",
                                    format!(
                                        "====>>>port_name:{} port_index:{}",
                                        port.port_name, index
                                    )
                                    .green()
                                );
                            } else {
                                println!("port_name:{} port_index:{}", port.port_name, index);
                            }
                        }
                    }
                    KeyCode::Char('r' | 'R') => {
                        return PortMenuAction::Refresh;
                    }
                    KeyCode::Enter if !port_buffer.is_empty() => {
                        break;
                    }
                    KeyCode::Esc => {
                        return PortMenuAction::Cancel;
                    }
                    _ => {}
                }
            }
        }
    }
    println!("选中串口：{}", port_buffer[select].port_name);
    PortMenuAction::Selected(select)
}

fn serial_slect_buard_memu(buard_buffer: &[u32]) -> Option<usize> {
    let mut select: usize = 0;
    if buard_buffer.is_empty() {
        return None;
    }
    for (index, buard) in buard_buffer.iter().enumerate() {
        if select == index {
            println!(
                "{}",
                format!("====>>>baud_rate:{} baud_index:{}", buard, index).green()
            );
        } else {
            println!("baud_rate:{} baud_index:{}", buard, index);
        }
    }
    loop {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => {
                        if select == 0 {
                            select = 0;
                        } else {
                            select -= 1;
                        }
                        if let Err(error) =
                            execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0))
                        {
                            eprintln!("清屏失败：{error}");
                        }
                        for (index, buard) in buard_buffer.iter().enumerate() {
                            if select == index {
                                println!(
                                    "{}",
                                    format!("====>>>baud_rate:{} baud_index:{}", buard, index)
                                        .green()
                                );
                            } else {
                                println!("baud_rate:{} baud_index:{}", buard, index);
                            }
                        }
                    }
                    KeyCode::Down => {
                        if select >= buard_buffer.len() - 1 {
                            select = buard_buffer.len() - 1;
                        } else {
                            select += 1;
                        }

                        if let Err(error) =
                            execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0))
                        {
                            eprintln!("清屏失败：{error}");
                        }
                        for (index, buard) in buard_buffer.iter().enumerate() {
                            if select == index {
                                println!(
                                    "{}",
                                    format!("====>>>baud_rate:{} baud_index:{}", buard, index)
                                        .green()
                                );
                            } else {
                                println!("baud_rate:{} baud_index:{}", buard, index);
                            }
                        }
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    KeyCode::Esc => {
                        return None;
                    }
                    _ => {}
                }
            }
        }
    }
    println!("index = {}", buard_buffer[select]);
    Some(select)
}

pub fn serial_log_run(port_name: &str, buard: u32) {
    let mut port = match serialport::new(port_name, buard)
        .timeout(Duration::from_millis(100))
        .open()
    {
        Ok(valid) => valid,
        Err(error) => {
            eprintln!("open serial port :{} error reason :{}", port_name, error);
            return;
        }
    };
    eprintln!("已打开 {}，波特率 {}，等待数据…", port_name, buard);
    let mut serial_buf: Vec<u8> = vec![0; 2048];
    let mut filtered = Vec::with_capacity(serial_buf.len());
    // FF 可能在本次读取的末尾，下一字节要等下次读取。
    let mut skip_next = false;
    loop {
        if let Ok(true) = event::poll(Duration::ZERO) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('c' | 'C') => {
                            if let Err(error) =
                                execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0))
                            {
                                eprintln!("清屏失败：{error}");
                            }
                        }
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }
        match port.read(serial_buf.as_mut_slice()) {
            Ok(count) => {
                filtered.clear();
                for &byte in &serial_buf[..count] {
                    if skip_next {
                        skip_next = false;
                    } else if byte == 0xFF {
                        skip_next = true;
                    } else {
                        filtered.push(byte);
                    }
                }
                if !filtered.is_empty() {
                    print!("{}", String::from_utf8_lossy(&filtered));
                }
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::TimedOut {
                    continue;
                }
                eprintln!("串口读取失败：{}", error);
                break;
            }
        }
    }
}

pub fn serial_task_run() {
    if let Err(error) = terminal::enable_raw_mode() {
        eprintln!("无法开启按键模式：{error}");
        return;
    }
    if let Err(error) = execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0)) {
        eprintln!("清屏失败：{error}");
    }
    serial_head();
    let buard_table: [u32; 2] = [9600, 115200];
    loop {
        let ports = match serialport::available_ports() {
            Ok(ports) => ports,
            Err(error) => {
                eprintln!("扫描串口失败：{error}");
                break;
            }
        };

        match serial_slect_port_memu(&ports) {
            PortMenuAction::Refresh => {
                if let Err(error) = execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0))
                {
                    eprintln!("清屏失败：{error}");
                }
                continue;
            }
            PortMenuAction::Cancel => break,
            PortMenuAction::Selected(port_index) => {
                if let Some(buard_index) = serial_slect_buard_memu(&buard_table) {
                    serial_log_run(&ports[port_index].port_name, buard_table[buard_index]);
                }
                break;
            }
        }
    }

    if let Err(error) = terminal::disable_raw_mode() {
        eprintln!("恢复终端失败：{error}");
    }
}

pub fn serial_log_thread_run() {
    let handle = std::thread::spawn(move || {
        // serial_run(&name_port, buard);
        serial_task_run();
    });
    handle.join().expect("串口线程发生了panic");
}

fn serial_head() {
    println!(
        r#"
        ██╗      ██████╗  ██████╗     ███████╗███████╗██╗     ███████╗
        ██║     ██╔═══██╗██╔════╝     ██╔════╝██╔════╝██║     ██╔════╝
        ██║     ██║   ██║██║  ███╗    ███████╗█████╗  ██║     █████╗
        ██║     ██║   ██║██║   ██║    ╚════██║██╔══╝  ██║     ██╔══╝
        ███████╗╚██████╔╝╚██████╔╝    ███████║███████╗███████╗██║
        ╚══════╝ ╚═════╝  ╚═════╝     ╚══════╝╚══════╝╚══════╝╚═╝
    "#
    );
    println!("  ↑↓ Select Port | Enter Confirm | R Rescan | Esc Cancel");
}
pub fn serial_thread_run(port_name: &str, buard: u32) {
    let name_port: String = port_name.to_string();
    let handle = std::thread::spawn(move || {
        serial_run(&name_port, buard);
    });
    handle.join().expect("串口线程发生了panic");
}

pub fn thread_demo() {
    let (tx, rx) = std::sync::mpsc::channel::<u16>();

    let handle = std::thread::spawn(move || {
        println!("后台线程：开始测量");
        std::thread::sleep(Duration::from_millis(1000));

        tx.send(2500).expect("发送失败");
        println!("后台线程：电压已发送");
    });

    println!("主线程：等待测量结果");

    match rx.recv() {
        Ok(voltage) => {
            println!("主线程：收到电压 {} mV", voltage);
        }
        Err(error) => {
            println!("主线程：接收失败：{}", error);
        }
    }

    handle.join().expect("后台线程发生了panic");
}
