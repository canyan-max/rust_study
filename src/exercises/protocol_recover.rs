/*  head  length data_h data_l ...datan_h datan_l....caculate
 *  [0]    [1]    [2]   [3]                          [length+2]
 */
const PROTOCOL_RECOVER_HEAD: u8 = 0xA8;

pub(super) enum ProtocolError {
    HeadCodeError,
    InLengthError,
    PackLengthError,
    ValidationError,
}
pub(super) struct Measurement {
    voltage_mv: u16,
    current_ma: i16,
}
impl Measurement {
    pub(super) fn new(voltage_mv: u16, current_ma: i16) -> Self {
        Self {
            voltage_mv: voltage_mv,
            current_ma: current_ma,
        }
    }
    pub(super) fn payload(buffer: &[u8]) -> Result<Self, ProtocolError> {
        if buffer.len() < 4 {
            return Err(ProtocolError::InLengthError);
        }
        // let data = Measurement {
        //     voltage_mv: u16::from_be_bytes([buffer[0], buffer[1]]),
        //     current_ma: i16::from_be_bytes([buffer[2], buffer[3]]),
        // };
        Ok(Measurement {
            voltage_mv: u16::from_be_bytes([buffer[0], buffer[1]]),
            current_ma: i16::from_be_bytes([buffer[2], buffer[3]]),
        })
    }
    pub(super) fn to_payload(&self) -> [u8; 4] {
        let mut depayload_buffer: [u8; 4] = [0; 4];
        depayload_buffer[0..2].copy_from_slice(&self.voltage_mv.to_be_bytes());
        depayload_buffer[2..4].copy_from_slice(&self.current_ma.to_be_bytes());
        depayload_buffer
    }
    pub(super) fn show_self(&self) {
        println!(
            "self->vol=>{} self->cur=>{} ",
            self.voltage_mv, self.current_ma
        );
    }
    pub(super) fn conver_mv_to_v(&self) -> f32 {
        // self.voltage_mv as f32 / 1000.0
        f32::from(self.voltage_mv) / 1000.0
    }
    pub(super) fn conver_ma_to_a(&self) -> f32 {
        f32::from(self.current_ma) / 1000.0
    }
    pub(super) fn get_vol_mv(&self) -> u16 {
        self.voltage_mv
    }
    pub(super) fn get_cur_ma(&self) -> i16 {
        self.current_ma
    }
}
fn protocol_recover_anti_sum(buffer: &[u8]) -> u8 {
    let mut anti_sum: u8 = 0;
    for index in 0..buffer.len() {
        anti_sum = anti_sum.wrapping_add(buffer[index]);
    }
    anti_sum = (!anti_sum).wrapping_add(1);
    anti_sum
}

pub(super) fn protocol_recover_pack(buffer: &[u8]) -> Result<Vec<u8>, ProtocolError> {
    let mut pack_buffer: Vec<u8> = Vec::new();
    if buffer.len() > 0xff {
        return Err(ProtocolError::PackLengthError);
    }
    pack_buffer.push(PROTOCOL_RECOVER_HEAD);
    pack_buffer.push(buffer.len() as u8);
    pack_buffer.extend_from_slice(buffer);
    let data = protocol_recover_anti_sum(&pack_buffer);
    pack_buffer.push(data);
    Ok(pack_buffer)
}

fn protocol_recover_unpack(data_of_in: &[u8]) -> Result<&[u8], ProtocolError> {
    // check the recevice length err
    if data_of_in.len() < 3 {
        return Err(ProtocolError::InLengthError);
    }
    // check head is valid
    //
    if let Some(head_code) = data_of_in.first() {
        if *head_code != PROTOCOL_RECOVER_HEAD {
            return Err(ProtocolError::HeadCodeError);
        }
    } else {
        return Err(ProtocolError::HeadCodeError);
    }
    // check the data lenth is valid
    let data_length: usize = usize::from(data_of_in[1]);
    let frame_length: usize = data_length + 3;
    if data_of_in.len() < frame_length {
        return Err(ProtocolError::PackLengthError);
    }
    // check validation is valid
    let input_anti_value: u8 = data_of_in[frame_length - 1];
    let anti_value: u8 = protocol_recover_anti_sum(&data_of_in[..frame_length - 1]);
    if anti_value != input_anti_value {
        return Err(ProtocolError::ValidationError);
    }
    return Ok(&data_of_in[2..2 + data_length]);
}

pub(super) fn protocol_recover_process(buffer: &mut Vec<u8>) -> Vec<Measurement> {
    let mut measurements: Vec<Measurement> = Vec::new();
    loop {
        match protocol_recover_unpack(buffer) {
            Ok(frame) => {
                let frame_length = frame.len() + 3;
                println!("successful：{:02X?}", frame);
                match Measurement::payload(frame) {
                    Ok(content) => measurements.push(content),
                    Err(_) => {
                        println!("err payload");
                    }
                }
                buffer.drain(0..frame_length);
            }
            Err(ProtocolError::HeadCodeError) => {
                buffer.drain(0..1);
            }
            Err(ProtocolError::InLengthError) => {
                break;
            }
            Err(ProtocolError::PackLengthError) => {
                break;
            }
            Err(ProtocolError::ValidationError) => {
                buffer.drain(0..1);
                // break;
            }
        }
    }
    measurements
}

#[cfg(test)]
mod test {
    // #[warn(unused)]
    use super::Measurement;
    use super::ProtocolError;
    use super::protocol_recover_pack;
    use super::protocol_recover_process;
    use super::protocol_recover_unpack;
    #[test]
    fn test_unpack_ok() {
        let buffer: [u8; 4] = [0xA8, 0x01, 0x02, 0x55];
        match protocol_recover_unpack(&buffer) {
            Ok(frame) => {
                assert_eq!(frame, &[0x02]);
                println!("fram contain ={:02X?}", frame);
                println!("frame length = {}", frame.len());
            }
            Err(_) => {
                panic!(" the frame is err ");
            }
        }
    }
    #[test]
    fn test_unpack_validationerror() {
        let buffer: [u8; 4] = [0xA8, 0x01, 0x02, 0x53];
        match protocol_recover_unpack(&buffer) {
            Err(ProtocolError::ValidationError) => {}
            _ => {
                panic!(" the frame is err => ValidationError ");
            }
        }
    }
    #[test]
    fn test_unpack_packlengtherror() {
        let buffer: [u8; 4] = [0xA8, 0x03, 0x01, 0x02];
        match protocol_recover_unpack(&buffer) {
            Err(ProtocolError::PackLengthError) => {}
            _ => {
                panic!(" ");
            }
        }
    }
    #[test]
    fn test_unpack_headcodeerror() {
        let buffer: [u8; 4] = [0xA9, 0x01, 0x02, 0x55];
        match protocol_recover_unpack(&buffer) {
            Err(ProtocolError::HeadCodeError) => {}
            _ => {
                panic!(" ");
            }
        }
    }
    #[test]
    fn test_unpack_inlengtherror() {
        let buffer: [u8; 2] = [0xA8, 0x01];
        match protocol_recover_unpack(&buffer) {
            Err(ProtocolError::InLengthError) => {}
            _ => {
                panic!(" ");
            }
        }
    }
    #[test]
    fn test_pack_ok() {
        let buffer: [u8; 1] = [0x02];
        let pakcks: Vec<u8> = match protocol_recover_pack(&buffer) {
            Ok(frame) => frame,
            Err(_) => {
                panic!("pack data is error ");
            }
        };
        assert_eq!(pakcks, &[0xA8, 0x01, 0x02, 0x55]);
    }
    #[test]
    fn test_pack_length_err() {
        let buffer: [u8; 256] = [0; 256];
        match protocol_recover_pack(&buffer) {
            Err(ProtocolError::PackLengthError) => {}
            _ => {
                panic!(" ");
            }
        }
    }
    #[test]
    fn test_pack_then_unpack() {
        let buffer: [u8; 3] = [0x01, 0x02, 0x03];
        let pakcks: Vec<u8> = match protocol_recover_pack(&buffer) {
            Ok(frame) => frame,
            Err(_) => {
                panic!("pack data is error ");
            }
        };
        assert_eq!(pakcks, &[0xA8, 0x03, 0x01, 0x02, 0x03, 0x4f]);
        match protocol_recover_unpack(&pakcks) {
            Ok(frame) => {
                assert_eq!(frame, &buffer);
                println!("fram contain ={:02X?}", frame);
                println!("frame length = {}", frame.len());
            }
            Err(_) => {
                panic!("unpack data is error ");
            }
        }
    }
    #[test]
    fn test_receive_in_parts() {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&[0xA8, 0x03, 0x01]);
        match protocol_recover_unpack(&buffer) {
            Err(ProtocolError::PackLengthError) => {}
            _ => {
                panic!("数据未收齐，应该返回 PackLengthError");
            }
        }
        buffer.extend_from_slice(&[0x02, 0x03, 0x4F]);
        match protocol_recover_unpack(&buffer) {
            Ok(frame) => {
                assert_eq!(frame, &[0x01, 0x02, 0x03]);
                println!("fram contain ={:02X?}", frame);
                println!("frame length = {}", frame.len());
            }
            Err(_) => {
                panic!("unpack data is error ");
            }
        }
    }
    #[test]
    fn test_process() {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&[0xA8, 0x03, 0x01]);
        protocol_recover_process(&mut buffer);
        assert_eq!(buffer, &[0xA8, 0x03, 0x01]);
        buffer.extend_from_slice(&[0x02, 0x03, 0x4F]);
        protocol_recover_process(&mut buffer);
        assert!(buffer.is_empty());

        buffer.extend_from_slice(&[0xFF, 0xA8, 0x03, 0x01]);
        protocol_recover_process(&mut buffer);
        assert_eq!(buffer, &[0xA8, 0x03, 0x01]);
        buffer.extend_from_slice(&[0x02, 0x03, 0x4F]);
        protocol_recover_process(&mut buffer);
        assert!(buffer.is_empty());

        buffer.extend_from_slice(&[0xA8, 0x03, 0x01]);
        protocol_recover_process(&mut buffer);
        assert_eq!(buffer, &[0xA8, 0x03, 0x01]);
        buffer.extend_from_slice(&[0x02, 0x03]);
        protocol_recover_process(&mut buffer);
        assert_eq!(buffer, &[0xA8, 0x03, 0x01, 0x02, 0x03]);
        buffer.extend_from_slice(&[0x4F]);
        protocol_recover_process(&mut buffer);
        assert!(buffer.is_empty());

        buffer.extend_from_slice(&[0xA8, 0x03, 0x01, 0x02, 0x03, 0x4E]);
        protocol_recover_process(&mut buffer);
        buffer.extend_from_slice(&[0xA8, 0x03, 0x01, 0x02, 0x03, 0x4F]);
        protocol_recover_process(&mut buffer);
        assert!(buffer.is_empty());
    }
    #[test]
    fn test_measurement_round_trip() {
        let datas: Measurement = Measurement::new(2500, -100);
        let buffer: [u8; 4] = datas.to_payload();
        assert_eq!(buffer, [0x09, 0xC4, 0xFF, 0x9C]);
        match Measurement::payload(&buffer) {
            Ok(content) => {
                // content.show_self();
                assert_eq!(content.voltage_mv, 2500);
                assert_eq!(content.current_ma, -100);
                println!("电压：{:.3} V", content.conver_mv_to_v());
                println!("电流：{:.3} A", content.conver_ma_to_a());
            }
            Err(_) => {
                panic!("err payload");
            }
        }
    }
}
