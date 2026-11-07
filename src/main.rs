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

// struct Counter {
//     value:u16,
// }
// impl  Counter {
//     fn get(&self)->u16 {
//         return self.value;
//     }

//     fn ader(&mut self) {
//          self.value+=1;
//     }

//     fn set_value(&mut self , current_v:u16) {
//         self.value = current_v;
//     }

//     fn is_above(&self,limit:u16) ->bool {
//         return self.value>limit;
//     }
// }
// fn main() {
//     let mut counter = Counter { value: 10 };
//     let mut is_above:bool = counter.is_above(20);
//     println!("当前值={}",is_above);
//     counter.ader();
//     println!("当前值={}", counter.get());
//     counter.set_value(20);
//     println!("设置后={}", counter.get());
//     is_above = counter.is_above(11);
//     println!("当前值={}",is_above);
//     return;
// }
// fn read_first_register(data: &[u8]) ->u16 {
//     let mut ret_first:u16 = 0;
//     if data.len() < 2 {
//         return  ret_first;
//     }
//     ret_first = u16::from_le_bytes([data[0], data[1]]);
//     return ret_first;
// }

// fn read_first_register(data: &[u8]) -> Result<u16, String> {
//     if data.len() < 2 {
//         return Err(String::from("length error"));
//     }

//     let ret_first: u16 = u16::from_le_bytes([data[0], data[1]]);

//     return Ok(ret_first);
// }
// fn main() {

//     let data = [0x12, 0x34, 0xAA];
//     let short_data = [0x12];

//     let result = read_first_register(&data);
//     match result {
//         Ok(value) => {
//             println!("寄存器值=0x{:04X}", value);
//         }
//         Err(error) => {
//             println!("读取失败：{}", error);
//         }
//     }

// use std::string;

// use eframe::egui::TextStyle::Name;

//     let result = read_first_register(&short_data);
//         match result {
//         Ok(value) => {
//             println!("寄存器值=0x{:04X}", value);
//         }
//         Err(error) => {
//             println!("读取失败：{}", error);
//         }
//     }
// }
// enum Portstate {
//     Disconnection,
//     Connection,
// }
// fn main() {
//     let mut state_port = Portstate::Disconnection;
//     match state_port {
//         Portstate::Disconnection => {
//             println!("not connect");
//         }

//         Portstate::Connection => {
//             println!("connect");
//         }
//     }
//     state_port = Portstate::Connection;
//     match state_port {
//         Portstate::Disconnection => {
//             println!("not connect");
//         }

//         Portstate::Connection => {
//             println!("connect");
//         }
//     }
// }

// enum Portstate {
//     Disconnection,
//     Connection(String),
//     Failed(String),
// }

// fn show_port_state (port:&Portstate) {

//     match  port {
//         Portstate::Connection(name) => {
//             println!("已连接串口：{}", name);
//         }
//         Portstate::Disconnection => {
//             println!("not connected")
//         }
//         Portstate::Failed(reason) => {
//             println!("reason {}", reason);
//         }
//     }
// }
// fn disconnected (port:&mut Portstate) {
//     *port = Portstate::Disconnection;
// }

// fn set_failed (port:&mut Portstate , reason:&String) {
//     *port = Portstate::Failed(reason.clone());
// }
// fn main() {
//     let mut state = Portstate::Connection(String::from("COM14"));
//     show_port_state(&state);
//     state = Portstate::Failed(String::from("not serial"));
//     show_port_state(&state);
//     disconnected(&mut state);
//     show_port_state(&state);
//     let mut reason:String = String::from("output");
//     set_failed(&mut state,&reason);
//     show_port_state(&state);
//     reason = String::from("last");
//     set_failed(&mut state,&reason);
//     show_port_state(&state);
// }

// enum Portstate {
//     Disconnection,
//     Connection(String),
//     Failed(String),
// }
// impl Portstate {
//     fn show_port_state(&self) {
//         match self {
//             Portstate::Connection(name) => {
//                 println!("is connected:{}", name);
//             }
//             Portstate::Disconnection => {
//                 println!("not connected")
//             }
//             Portstate::Failed(reason) => {
//                 println!("{}",reason);
//             }
//         }
//     }

//     fn disconnected(&mut self) {
//         *self = Portstate::Disconnection;
//     }

//     fn set_failed(&mut self, reason: &String) {
//         *self = Portstate::Failed(reason.clone());
//     }
// }

// fn main() {
//     let mut state = Portstate::Connection(String::from("COM14"));
//     let mut reason: String = String::from("last");
//     state.show_port_state();
//     state.disconnected();
//     state.show_port_state();
//     state.set_failed(&reason);
//     state.show_port_state();
//     reason = String::from("last two");
//     state.set_failed(&reason);
//     state.show_port_state();
// }

// enum ErrorCode{
//     Tooshort,
// }
// fn read_first_register(data: &[u8]) -> Result<u16, ErrorCode> {
//     if data.len() < 2 {
//         return Err(ErrorCode::Tooshort);
//     }
//     let ret_first: u16 = u16::from_le_bytes([data[0], data[1]]);

//     return Ok(ret_first);
// }
// fn main() {
//     let data = [0x12, 0x34, 0xAA];
//     let short_data = [0x12];
//     let result = read_first_register(&data);
//     match result {
//         Ok(value) => {
//             println!("register value = 0x{:04X}", value);
//         }
//         Err(ErrorCode::Tooshort) => {
//             println!("read error");
//         }
//     }
//     let result = read_first_register(&short_data);
//     match result {
//         Ok(value) => {
//             println!("register value = 0x{:04X}", value);
//         }
//         Err(ErrorCode::Tooshort) => {
//             println!("Tooshort");
//         }
//     }
// }

// enum ErrorCode {
//     Tooshort(usize),
// }
// fn read_first_register(data: &[u8]) -> Result<u16, ErrorCode> {
//     if data.len() < 2 {
//         return Err(ErrorCode::Tooshort(data.len()));
//     }
//     let ret_first: u16 = u16::from_le_bytes([data[0], data[1]]);

//     return Ok(ret_first);
// }
// fn main() {
//     let data = [0x12, 0x34, 0xAA];
//     let short_data = [0x12];
//     let mut result = read_first_register(&data);
//     match result {
//         Ok(value) => {
//             println!("register value = 0x{:04X}", value);
//         }
//         Err(ErrorCode::Tooshort(length)) => {
//             println!("read length is {}", length);
//         }
//     }
//     result = read_first_register(&short_data);
//     match result {
//         Ok(value) => {
//             println!("register value = 0x{:04X}", value);
//         }
//         Err(ErrorCode::Tooshort(length)) => {
//             println!("read length is {}", length);
//         }
//     }
// }
// enum ErrorCode {
//     Tooshort(usize),
// }
// fn read_first_register(data: &[u8]) -> Result<u16, ErrorCode> {
//     if data.len() < 2 {
//         return Err(ErrorCode::Tooshort(data.len()));
//     }
//     let ret_first: u16 = u16::from_le_bytes([data[0], data[1]]);

//     return Ok(ret_first);
// }

// fn read_and_show(data: &[u8]) -> Result<(), ErrorCode> {
//     let value = read_first_register(data)?;

//     println!("register 0x{:04X}", value);
//     return Ok(());
// }

// fn main() {
//     let data = [0x12, 0x34, 0xAA];
//     let short_data = [0x12];
//     let _ = read_and_show(&short_data);
//     read_and_show(&data);
// }

// use std::vec::Vec;
// enum ErrorCode {
//     Tooshort(usize),
// }
// fn read_first_register(data: &[u8]) -> Result<u16, ErrorCode> {
//     if data.len() < 2 {
//         return Err(ErrorCode::Tooshort(data.len()));
//     }
//     let ret_first: u16 = u16::from_le_bytes([data[0], data[1]]);

//     return Ok(ret_first);
// }
// fn main() {
//     let mut buffer: Vec<u8> = Vec::new();
//     buffer.push(0x12);
//     println!("current buffer length {}",buffer.len());
//     let register = read_first_register(&buffer);
//     match  register {
//         Ok(value) => {
//             println!("register value = 0x{:04X}", value);
//         }
//         Err(ErrorCode::Tooshort(length)) => {
//             println!("read length is {}", length);
//         }
//     }
//     buffer.push(0x32);
//     println!("current buffer length {}",buffer.len());
//     let register = read_first_register(&buffer);
//     match  register {
//         Ok(value) => {
//             println!("register value = 0x{:04X}", value);
//         }
//         Err(ErrorCode::Tooshort(length)) => {
//             println!("read length is {}", length);
//         }
//     }

// }

// use std::vec::Vec;
// fn main() {
//     let mut buffer: Vec<u8> = Vec::new();
//     let frist_data: [u8; 2] = [0x01, 0x02];
//     let second_data: [u8; 2] = [0x22, 0x33];
//     buffer.extend_from_slice(&frist_data);
//     println!("buffer length:{}", buffer.len());
//     buffer.extend_from_slice(&second_data);
//     println!("buffer length:{}", buffer.len());
//     for index in 0..buffer.len() {
//         println!("index:{} buffer contain:0x{:02X}", index,buffer[index]);
//     }
//     buffer.clear();
//     println!("buffer length:{}", buffer.len());
// }
// use std::vec::Vec;
// fn main() {
//     let  buffer: Vec<u8> = vec![0x12, 0x22];
//     match buffer.get(5) {
//         Some(byte) => {
//             println!("字节：0x{:02X}", byte);
//         }

//         None => {
//             println!("下标超出范围");
//         }
//     }
// }

// #[allow(dead_code)]
// fn fill_byte(p_data:&mut [u8],fill_value:u8) {
//     for  cotain in  p_data.iter_mut(){
//         *cotain = fill_value;
//     }

// use eframe::egui::frame;

// }
// fn main () {
// let mut buffer:Vec<u8>= vec![0x15, 0x03, 0x02, 0x12, 0x34];
// for byte in buffer.iter(){
//     println!("before byte value ={:02X}",*byte);
// }
// buffer.fill(1);
// let idenx : usize = 1;
// if  let Some(bytes) = buffer.get_mut(idenx){
//     *bytes = 0x03;
//     println!("the idenx value = {}",buffer[idenx]);
// } else {
//     println!("the index is valid ");
// }
// for (index,byte) in buffer.iter_mut().enumerate(){
//     *byte += 1;
//     println!("index={} byte value={}",index,*byte);
// }
// let data :&mut [u8] = &mut buffer[3..5];
// println!("data= {:02X?}",data);
// println!("data len = {}",data.len());
// for (index,byte) in buffer.iter_mut().enumerate(){
//     println!("index={} byte value={:02X}",index,*byte);
// }
// data[0] = 0xaa;
// data[1] = 0xa2;
// println!("data= {:02X?}",data);
// println!("data len = {}",data.len());
// for (index,byte) in buffer.iter_mut().enumerate(){
//     println!("index={} byte value={:02X}",index,*byte);
// }
// fill_byte(& mut buffer[0..3], 0x55);
// for byte in buffer.iter(){
//     println!("byte value ={:02X}",*byte);
// }
// buffer[0..3].fill(0x55);
// for byte in buffer.iter(){
//     println!("byte value ={:02X}",*byte);
// }
// }
// use std::vec::Vec;
// fn build_frame (reg_adr:u16,count:u16) -> [u8;8]  {
//     let mut frame :[u8;8] = [0x01,0x03,0x00,0x04,0x00,0x02,0x00,0x00];
//     frame[2..4].copy_from_slice(&reg_adr.to_be_bytes());
//     frame[4..6].copy_from_slice(&count.to_be_bytes());
//     return frame;
// }
// fn main () {
//     let mut frame :[u8;8] = [0x01,0x03,0x00,0x04,0x00,0x02,0x00,0x00];
//     let regster_adr :u16  = 0x1234;
//     let regster_count:u16 = 0x2233;
//     let mut convert_buffer :[u8;2] =regster_adr.to_be_bytes();
//     // convert_buffer = regster_adr.to_be_bytes();
//     frame[2..4].copy_from_slice(&convert_buffer);
//     for byte in frame.iter(){
//         println!("byte value ={:02X}",*byte);
//     }
//     let clac_crc :u16 = 0x7890;
//     convert_buffer = clac_crc.to_le_bytes();
//     frame[6..8].copy_from_slice(&convert_buffer);
//     for byte in frame.iter(){
//         println!("clac_crc value ={:02X}",*byte);
//     }
// }

// fn build_frame(slave_id: u8, reg_adr: u16, count: u16) -> [u8; 8] {
//     let mut frame: [u8; 8] = [0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x00];
//     frame[0] = slave_id;
//     frame[2..4].copy_from_slice(&reg_adr.to_be_bytes());
//     frame[4..6].copy_from_slice(&count.to_be_bytes());
//     return frame;
// }
// fn main() {
//     let mut frame: [u8; 8] = [0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0xff, 0xff];
//     for byte in frame.iter() {
//         println!("clac_crc value ={:02X}", *byte);
//     }
//     let regster_adr: u16 = 0x1234;
//     let regster_count: u16 = 0x2233;
//     let slave_id: u8 = 0x15;
//     frame = build_frame(slave_id, regster_adr, regster_count);
//     for byte in frame.iter() {
//         println!("clac_crc value ={:02X}", *byte);
//     }
// // }
// enum BuildError {
//     ZeroErr,
//     CountTooLarge(u16),
// }
// fn build_frame(slave_id: u8, reg_adr: u16, count: u16) -> Result<[u8; 8], BuildError> {
//     let mut frame: [u8; 8] = [0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x00];
//     if 0 == count {
//         return Err(BuildError::ZeroErr);
//     } else if count > 255 {
//         return Err(BuildError::CountTooLarge(count));
//     }
//     frame[0] = slave_id;
//     frame[2..4].copy_from_slice(&reg_adr.to_be_bytes());
//     frame[4..6].copy_from_slice(&count.to_be_bytes());
//     return Ok(frame);
// }

// fn build_and_show(slave_id: u8, reg_adr: u16, count: u16) -> Result<(), BuildError> {
//     let frame: [u8; 8] = build_frame(slave_id, reg_adr, count)?;
//     println!("请求帧：{:02X?}", frame);
//     return Ok(());
// }
// fn main() {
//     let regster_adr: u16 = 0x1234;
//     let regster_count: u16 = 0x00;
//     let slave_id: u8 = 0x15;
//     match build_and_show(slave_id, regster_adr, regster_count)
//     {
//         Ok(()) => {
//             println!("操作完成");
//         }
//         Err(BuildError::ZeroErr) => {
//             println!("数量不能为 0");
//         }
//         Err(BuildError::CountTooLarge(count)) => {
//             println!("数量过大：{}", count);
//         }
//     }
// }

mod frame_tools;
fn main() {
    let request: frame_tools::ReadRequest = frame_tools::ReadRequest {
        slave_id: 0x15,
        reg_adr: 0x1234,
        reg_length: 0x01,
    };
    match frame_tools::build_and_show(&request) {
        Ok(()) => {
            println!("操作完成");
        }
        Err(frame_tools::BuildError::ZeroErr) => {
            println!("数量不能为 0");
        }
        Err(frame_tools::BuildError::CountTooLarge(count)) => {
            println!("数量过大：{}", count);
        }
    }
}
