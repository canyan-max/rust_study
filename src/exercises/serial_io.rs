use super::protocol_recover::Measurement;
use super::protocol_recover::ProtocolError;
use super::protocol_recover::protocol_recover_pack;
use super::protocol_recover::protocol_recover_process;
use std::time::Duration;
pub fn serial_run() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
    let mut port = serialport::new("COM14", 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    let test_var: Measurement = Measurement::new(2500, -100);
    let content: [u8; 4] = test_var.to_payload();
    let output: Vec<u8> = match protocol_recover_pack(&content) {
        Ok(frame) => {
            println!("{:02x?}", frame);
            frame
        }
        Err(ProtocolError::HeadCodeError) => {
            return;
        }
        Err(ProtocolError::InLengthError) => {
            return;
        }
        Err(ProtocolError::PackLengthError) => {
            return;
        }
        Err(ProtocolError::ValidationError) => {
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
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(count) => {
                println!("收到：{:02X?}", &serial_buf[..count]);
                recevice_buffer.extend_from_slice(&serial_buf[..count]);
                let measurements: Vec<Measurement> = protocol_recover_process(&mut recevice_buffer);
                if let Some(measurement) = measurements.last() {
                    measurement.show_self();
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

pub fn serial_thread_run() {
    let handle = std::thread::spawn(|| {
        serial_run();
    });
    println!("主线程：串口接收线程已启动");
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
