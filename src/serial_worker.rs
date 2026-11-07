//! 串口只由后台线程持有。界面用消息发送命令，不阻塞等待串口。

use crate::protocol::{self, Snapshot};
use serialport::{ClearBuffer, SerialPort};
use std::io::{self, Read};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};

pub enum Command {
    RefreshPorts,
    Connect { name: String, baud: u32 },
    Disconnect,
    ReadOnce,
    SetAuto(bool),
    Stop,
}

pub enum Event {
    Ports(Vec<String>),
    Connected(String),
    Disconnected,
    Data(Snapshot),
    Error(String),
    Log(&'static str, String),
}

pub struct Worker {
    pub commands: Sender<Command>,
    pub events: Receiver<Event>,
}

impl Worker {
    pub fn start(context: eframe::egui::Context) -> Self {
        let (command_tx, command_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        std::thread::spawn(move || run(command_rx, event_tx, context));
        return Self {
            commands: command_tx,
            events: event_rx,
        };
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        let _ = self.commands.send(Command::Stop);
    }
}

fn open_port(name: &str, baud: u32) -> Result<Box<dyn SerialPort>, String> {
    return serialport::new(name, baud)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .flow_control(serialport::FlowControl::None)
        .timeout(Duration::from_millis(50))
        .open()
        .map_err(|error| format!("打开 {name} 失败：{error}"));
}

/// 一次 read 可能只返回一部分，因此先收头，再按长度收齐。
/// 失败时也保留收到的字节，供日志排查。
fn receive_frame<R: Read + ?Sized>(
    reader: &mut R,
    timeout: Duration,
) -> Result<Vec<u8>, (Vec<u8>, String)> {
    let started = Instant::now();
    let mut frame = Vec::new();
    let mut expected = 3;
    while frame.len() < expected {
        if started.elapsed() >= timeout {
            return Err((
                frame,
                "等待应答超时，检查接线、串口参数及 MCU 是否烧录了本协议".into(),
            ));
        }
        let mut buffer = [0u8; 256];
        let remaining = expected - frame.len();
        match reader.read(&mut buffer[..remaining]) {
            Ok(0) => return Err((frame, "串口读取返回 0 字节，连接可能已断开".into())),
            Ok(count) => frame.extend_from_slice(&buffer[..count]),
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::TimedOut
                        | io::ErrorKind::Interrupted
                        | io::ErrorKind::WouldBlock
                ) =>
            {
                continue;
            }
            Err(error) => return Err((frame, format!("读取失败：{error}"))),
        }
        if frame.len() >= 3 && expected == 3 {
            if frame[0] != protocol::SLAVE || !matches!(frame[1], 0x03 | 0x83) {
                return Err((
                    frame,
                    "应答地址或功能码不匹配，可能有旧数据、调试文本或接线干扰".into(),
                ));
            }
            if frame[1] == 0x83 {
                expected = 5;
            } else if frame[2] != 28 {
                return Err((frame, "应答字节数不是预期的 28".into()));
            } else {
                expected = 33;
            }
        }
    }
    return Ok(frame);
}

fn transaction(port: &mut dyn SerialPort, emit: &impl Fn(Event)) -> Result<Snapshot, String> {
    // 丢弃过期输入，保持帧间静默；同一时间只有一笔请求。
    port.clear(ClearBuffer::Input)
        .map_err(|error| format!("清理旧数据失败：{error}"))?;
    std::thread::sleep(Duration::from_millis(10));
    let request = protocol::read_request();
    emit(Event::Log("TX", protocol::hex(&request)));
    port.write_all(&request)
        .map_err(|error| format!("发送失败：{error}"))?;
    port.flush()
        .map_err(|error| format!("等待发送完成失败：{error}"))?;

    let received = match receive_frame(port, Duration::from_millis(1000)) {
        Ok(frame) => frame,
        Err((partial, message)) => {
            if !partial.is_empty() {
                emit(Event::Log(
                    "RX",
                    format!("{}（未完成）", protocol::hex(&partial)),
                ));
            }
            return Err(message);
        }
    };
    emit(Event::Log("RX", protocol::hex(&received)));
    return protocol::parse_response(&received);
}

fn run(commands: Receiver<Command>, events: Sender<Event>, context: eframe::egui::Context) {
    let emit = |event| {
        let _ = events.send(event);
        context.request_repaint();
    };
    let mut port: Option<Box<dyn SerialPort>> = None;
    let mut auto = true;
    let mut next_read = Instant::now();

    loop {
        let mut read_now = false;
        match commands.recv_timeout(Duration::from_millis(40)) {
            Ok(Command::RefreshPorts) => match serialport::available_ports() {
                Ok(ports) => emit(Event::Ports(
                    ports.into_iter().map(|port| port.port_name).collect(),
                )),
                Err(error) => emit(Event::Error(format!("枚举失败：{error}"))),
            },
            Ok(Command::Connect { name, baud }) => {
                port = None;
                match open_port(&name, baud) {
                    Ok(opened) => {
                        port = Some(opened);
                        next_read = Instant::now();
                        emit(Event::Connected(format!("{name} · {baud} · 8N1")));
                    }
                    Err(error) => {
                        emit(Event::Disconnected);
                        emit(Event::Error(error));
                    }
                }
            }
            Ok(Command::Disconnect) => {
                port = None;
                emit(Event::Disconnected);
            }
            Ok(Command::ReadOnce) => read_now = true,
            Ok(Command::SetAuto(enabled)) => {
                auto = enabled;
                next_read = Instant::now();
            }
            Ok(Command::Stop) | Err(RecvTimeoutError::Disconnected) => break,
            Err(RecvTimeoutError::Timeout) => {}
        }
        if let Some(opened) = port.as_mut()
            && (read_now || (auto && Instant::now() >= next_read))
        {
            match transaction(opened.as_mut(), &emit) {
                Ok(snapshot) => emit(Event::Data(snapshot)),
                Err(error) => emit(Event::Error(error)),
            }
            // 应答或超时结束后间隔 1 秒再发，避免请求重叠。
            next_read = Instant::now() + Duration::from_secs(1);
        }
    }
}

pub fn probe(name: &str, baud: u32) -> Result<(), String> {
    let mut port = open_port(name, baud)?;
    println!("已打开 {name}，{baud}/8N1，从站 0x15");
    let snapshot = transaction(port.as_mut(), &|event| {
        if let Event::Log(direction, message) = event {
            println!("{direction} {message}");
        }
    })?;
    for (index, info) in protocol::REGISTERS.iter().enumerate() {
        println!(
            "0x{:04X} {}：{}（原始值 {}）",
            protocol::FIRST_REGISTER + index as u16,
            info.name,
            snapshot.display_value(index),
            snapshot.raw_value(index)
        );
    }
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fragmented {
        bytes: io::Cursor<Vec<u8>>,
        pause: bool,
    }

    impl Read for Fragmented {
        fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
            self.pause = !self.pause;
            if self.pause {
                return Err(io::ErrorKind::TimedOut.into());
            }
            let count = output.len().min(2);
            return self.bytes.read(&mut output[..count]);
        }
    }

    #[test]
    fn reassembles_fragmented_serial_reads_and_tolerates_short_timeouts() {
        let frame = protocol::test_response([1234; 14]);
        let mut stream = Fragmented {
            bytes: io::Cursor::new(frame.clone()),
            pause: false,
        };
        assert_eq!(
            receive_frame(&mut stream, Duration::from_secs(1)).unwrap(),
            frame
        );
    }

    #[test]
    fn reads_five_byte_exception_instead_of_waiting_for_normal_reply() {
        let mut frame = vec![protocol::SLAVE, 0x83, 4];
        frame.extend_from_slice(&protocol::crc16(&frame).to_le_bytes());
        assert_eq!(
            receive_frame(&mut io::Cursor::new(frame.clone()), Duration::from_secs(1)).unwrap(),
            frame
        );
    }

    #[test]
    fn retains_partial_frame_when_stream_ends() {
        let partial = vec![protocol::SLAVE, 3, 28, 0x12];
        let error = receive_frame(
            &mut io::Cursor::new(partial.clone()),
            Duration::from_secs(1),
        )
        .unwrap_err();
        assert_eq!(error.0, partial);
    }

    #[test]
    fn rejects_wrong_header_and_stops_at_deadline() {
        assert!(
            receive_frame(&mut io::Cursor::new(vec![0, 3, 28]), Duration::from_secs(1)).is_err()
        );
        let result = receive_frame(&mut io::Cursor::new(vec![0x15]), Duration::ZERO);
        assert!(result.unwrap_err().1.contains("超时"));
    }
}
