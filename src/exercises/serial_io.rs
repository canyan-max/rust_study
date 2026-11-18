use super::unpack::FrameBuffer;
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
    let output: [u8; 6] = [0x12, 0x22, 0x12, 0x22, 0x12, 0x22];
    match port.write_all(&output) {
        Ok(()) => {
            println!("send ok");
        }
        Err(error) => {
            println!("send error {}", error);
        }
    }

    let mut serial_buf: Vec<u8> = vec![0; 32];
    let mut recevice_buffer: FrameBuffer = FrameBuffer::new();
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(count) => {
                // println!("收到：{:02X?}", &serial_buf[..count]);
                recevice_buffer.receive(&serial_buf[..count]);
                recevice_buffer.process_pack();
                // break;
            }
            Err(_) => {}
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
