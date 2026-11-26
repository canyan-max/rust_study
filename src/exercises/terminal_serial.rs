use crossterm::style::Stylize;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::time::Duration;

enum PortMenuAction {
    Selected(usize),
    Refresh,
    Cancel,
}

fn draw_head_memu() {
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

fn draw_refresh_menu(port_buffer: &[serialport::SerialPortInfo], menu_top: u16) {
    let line_count = port_buffer.len().max(1);

    for row in 0..line_count {
        if let Err(error) = execute!(
            std::io::stdout(),
            MoveTo(0, menu_top.saturating_add(row as u16)),
            Clear(ClearType::CurrentLine)
        ) {
            eprintln!("清理菜单失败：{error}");
            return;
        }
    }
}

fn draw_port_menu(port_buffer: &[serialport::SerialPortInfo], select: usize, menu_top: u16) {
    let line_count = port_buffer.len().max(1);
    for row in 0..line_count {
        if let Err(error) = execute!(
            std::io::stdout(),
            MoveTo(0, menu_top.saturating_add(row as u16)),
            Clear(ClearType::CurrentLine)
        ) {
            eprintln!("清理菜单失败：{error}");
            return;
        }
    }

    if let Err(error) = execute!(std::io::stdout(), MoveTo(0, menu_top)) {
        eprintln!("移动到菜单位置失败：{error}");
        return;
    }

    if port_buffer.is_empty() {
        println!("未找到串口，插入设备后按 R 重试。");
        return;
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
}

fn draw_buard_menu(buard_buffer: &[u32], select: usize, menu_top: u16) {
    let line_count = buard_buffer.len().max(1);
    for row in 0..line_count {
        if let Err(error) = execute!(
            std::io::stdout(),
            MoveTo(0, menu_top.saturating_add(row as u16)),
            Clear(ClearType::CurrentLine)
        ) {
            eprintln!("清理菜单失败：{error}");
            return;
        }
    }

    if let Err(error) = execute!(std::io::stdout(), MoveTo(0, menu_top)) {
        eprintln!("移动到菜单位置失败：{error}");
        return;
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
}

fn serial_slect_port_memu(
    port_buffer: &[serialport::SerialPortInfo],
    menu_top: u16,
) -> PortMenuAction {
    let mut select: usize = 0;
    draw_port_menu(port_buffer, select, menu_top);
    loop {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up if !port_buffer.is_empty() => {
                        select = select.saturating_sub(1);
                        draw_port_menu(port_buffer, select, menu_top);
                    }
                    KeyCode::Down if !port_buffer.is_empty() => {
                        if select + 1 < port_buffer.len() {
                            select += 1;
                        }
                        draw_port_menu(port_buffer, select, menu_top);
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
    if buard_buffer.is_empty() {
        return None;
    }
    let mut select: usize = 0;
    let (_, menu_top) = crossterm::cursor::position().unwrap_or((0, 0));
    draw_buard_menu(buard_buffer, select, menu_top);
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
                        draw_buard_menu(buard_buffer, select, menu_top);
                    }
                    KeyCode::Down => {
                        if select >= buard_buffer.len() - 1 {
                            select = buard_buffer.len() - 1;
                        } else {
                            select += 1;
                        }
                        draw_buard_menu(buard_buffer, select, menu_top);
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
    draw_head_memu();
    let (_, menu_top) = crossterm::cursor::position().unwrap_or((0, 0));
    let buard_table: [u32; 2] = [9600, 115200];
    loop {
        let ports = match serialport::available_ports() {
            Ok(ports) => ports,
            Err(error) => {
                eprintln!("扫描串口失败：{error}");
                break;
            }
        };

        match serial_slect_port_memu(&ports, menu_top) {
            PortMenuAction::Refresh => {
                draw_refresh_menu(&ports, menu_top);
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
