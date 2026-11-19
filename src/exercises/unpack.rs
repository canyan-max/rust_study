const FRAME_HEAD_AA: u16 = 0xAA55;
enum UnpackError {
    Incomplete,
    InvalidHead,
    InLengthShort,
    CrcMismatch,
}
pub struct FrameBuffer {
    buffer: Vec<u8>,
}
impl FrameBuffer {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }
    pub fn receive(&mut self, datas: &[u8]) {
        self.buffer.extend_from_slice(datas);
    }

    pub fn process_pack(&mut self) {
        frame_55aa_buffer_process(&mut self.buffer);
    }
}

struct MeasureMent {
    voltage_mv: u16,
    current_ma: i16,
}

impl MeasureMent {
    fn measure_parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 4 {
            return Err("not data".to_string());
        }

        return Ok(Self {
            voltage_mv: u16::from_be_bytes([data[0], data[1]]),
            current_ma: i16::from_be_bytes([data[2], data[3]]),
        });
    }
}
fn clac_crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for byte in data {
        crc ^= u16::from(*byte);
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    return crc;
}

fn unpack_frame(recevicebuff: &[u8]) -> Result<&[u8], UnpackError> {
    if recevicebuff.len() < 3 {
        return Err(UnpackError::InLengthShort);
    }
    let head_frame: u16 = u16::from_be_bytes([recevicebuff[0], recevicebuff[1]]);
    if FRAME_HEAD_AA != head_frame {
        return Err(UnpackError::InvalidHead);
    }
    let data_len: usize = usize::from(recevicebuff[2]);
    let fram_len: usize = data_len + 5;
    if recevicebuff.len() < fram_len {
        return Err(UnpackError::Incomplete);
    }
    let crc_offset: usize = 3 + data_len;
    let received_crc: u16 =
        u16::from_le_bytes([recevicebuff[crc_offset], recevicebuff[crc_offset + 1]]);
    let calculated_crc: u16 = clac_crc16(&recevicebuff[..crc_offset]);
    if received_crc != calculated_crc {
        return Err(UnpackError::CrcMismatch);
    }
    Ok(&recevicebuff[3..3 + data_len])
}

fn pack_frame(datas: &[u8]) -> Result<Vec<u8>, String> {
    if datas.len() > 0xff {
        return Err("the buffer so large".to_string());
    }
    let mut frame: Vec<u8> = Vec::new();
    frame.extend_from_slice(&(FRAME_HEAD_AA.to_be_bytes()));
    frame.push(datas.len() as u8);
    frame.extend_from_slice(datas);
    let crc16: u16 = clac_crc16(&frame);
    frame.extend_from_slice(&(crc16.to_le_bytes()));
    Ok(frame)
}

fn handle_payload(data: &[u8]) {
    match MeasureMent::measure_parse(data) {
        Ok(measurement) => {
            println!("电压：{} mV", measurement.voltage_mv);
            println!("电流：{} mA", measurement.current_ma);
        }
        Err(error) => {
            println!("解析失败：{}", error);
        }
    }
}

fn frame_55aa_buffer_process(buffer: &mut Vec<u8>) {
    loop {
        if buffer.is_empty() {
            println!("the buffer is empty");
            break;
        }
        match unpack_frame(buffer) {
            Ok(value) => {
                let frame_len: usize = value.len() + 5;
                handle_payload(&value);
                buffer.drain(0..frame_len);
            }
            Err(UnpackError::InvalidHead) => {
                println!("lost current buffer {:02X?}", buffer[0]);
                buffer.drain(0..1);
            }
            Err(UnpackError::Incomplete) => {
                break;
            }
            Err(UnpackError::InLengthShort) => {
                break;
            }
            Err(UnpackError::CrcMismatch) => {
                println!("crc error {:02X?}", buffer[0]);
                buffer.drain(0..1);
            }
        };
    }
}
pub fn unpack_run() {
    // let mut buffer: Vec<u8> = Vec::new();
    // buffer.extend_from_slice(&[0xAA, 0x55, 0x02, 0x11, 0x22]);
    // buffer.extend_from_slice(&[0x11, 0x12]);
    // buffer.extend_from_slice(&[0xAA, 0x55, 0x01, 0x33]);
    // buffer.extend_from_slice(&[0xff, 0xfa, 0xAA, 0x55, 0x01, 0x33]);
    // frame_55aa_buffer_process(&mut buffer);
    // let datas: [u8; 3] = [0x01, 0x02, 0x03];
    // match pack_frame(&datas) {
    //     Ok(packdata) => {
    //         println!("={:02X?}", packdata);
    //     }
    //     Err(str) => {
    //         println!("{}", str);
    //     }
    // };
    //
    //

    // let datas: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
    // let mut buffer: Vec<u8> = pack_frame(&datas);
    let datas: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
    let mut packdata: Vec<u8> = match pack_frame(&datas) {
        Ok(frame) => frame,
        Err(error) => {
            panic!("pack err reason：{}", error);
        }
    };
    frame_55aa_buffer_process(&mut packdata);
}

#[cfg(test)]
mod test {
    use super::UnpackError;
    use super::frame_55aa_buffer_process;
    use super::pack_frame;
    use super::unpack_frame;
    #[test]
    fn test_unpack_frame_length_err() {
        let test_buffer: Vec<u8> = vec![0xAA];
        let result = unpack_frame(&test_buffer);
        assert!(result.is_err());
    }
    #[test]
    fn test_unpack_frame_ok() {
        let test_buffer: Vec<u8> = vec![0xAA, 0x55, 0x02, 0x11, 0x44, 0x80, 0x77];
        match unpack_frame(&test_buffer) {
            Ok(value) => {
                assert_eq!(value, &[0x11, 0x44]);
            }
            Err(_) => {
                panic!("error log ")
            }
        }
    }
    #[test]
    fn test_unpack_frame_err1() {
        let test_buffer: Vec<u8> = vec![0xAB, 0x55, 0x02, 0x11, 0x44, 0x56];
        let result = unpack_frame(&test_buffer);
        assert!(result.is_err());
    }
    #[test]
    fn test_unpack_frame_err2() {
        let test_buffer: Vec<u8> = vec![0xAA, 0x56, 0x02, 0x11, 0x44, 0x56];
        let result = unpack_frame(&test_buffer);
        assert!(result.is_err());
    }
    #[test]
    fn test_unpack_frame_err3() {
        let test_buffer: Vec<u8> = vec![0xAA, 0x55, 0x03, 0x11, 0x44];
        let result = unpack_frame(&test_buffer);
        assert!(result.is_err());
    }
    #[test]
    fn test_frame_process_ok() {
        let mut test_buffer: Vec<u8> = vec![0xAA, 0x55, 0x03, 0x11, 0x44];
        frame_55aa_buffer_process(&mut test_buffer);
        assert_eq!(test_buffer, vec![0xAA, 0x55, 0x03, 0x11, 0x44]);
    }
    #[test]
    fn test_frame_process_err_not_data() {
        let mut test_buffer: Vec<u8> = vec![0xAA, 0x55, 0x03, 0x11];
        frame_55aa_buffer_process(&mut test_buffer);
        assert_eq!(test_buffer, vec![0xAA, 0x55, 0x03, 0x11]);
    }
    #[test]
    fn test_frame_process_integrity_and_half() {
        let mut test_buffer: Vec<u8> = vec![
            0xAA, 0x55, 0x03, 0x11, 0x11, 0x11, 0x88, 0x00, 0xAA, 0x55, 0x02, 0x01,
        ];
        frame_55aa_buffer_process(&mut test_buffer);
        assert_eq!(test_buffer, vec![0xAA, 0x55, 0x02, 0x01]);
    }
    #[test]
    fn test_frame_process_messy_and_integrity() {
        let mut test_buffer: Vec<u8> = vec![0xff, 0xfa, 0xAA, 0x55, 0x02, 0x01];
        frame_55aa_buffer_process(&mut test_buffer);
        assert_eq!(test_buffer, vec![0xAA, 0x55, 0x02, 0x01]);
    }
    #[test]
    fn test_frame_process_keep_incresing() {
        let mut test_buffer: Vec<u8> = vec![0xAA, 0x55, 0x02, 0x01];
        frame_55aa_buffer_process(&mut test_buffer);
        assert_eq!(test_buffer, vec![0xAA, 0x55, 0x02, 0x01]);
        // 补齐最后一个数据字节和两个 CRC 字节。
        test_buffer.extend_from_slice(&[0x22, 0x0D, 0x9D]);
        frame_55aa_buffer_process(&mut test_buffer);
        assert!(test_buffer.is_empty());
    }
    #[test]
    fn test_frame_build_and_process() {
        let datas: [u8; 3] = [0x01, 0x02, 0x03];
        let mut packdata: Vec<u8> = match pack_frame(&datas) {
            Ok(frame) => frame,
            Err(error) => {
                panic!("pack err reason：{}", error);
            }
        };
        assert_eq!(
            packdata,
            vec![0xAA, 0x55, 0x03, 0x01, 0x02, 0x03, 0x04, 0xF8]
        );
        frame_55aa_buffer_process(&mut packdata);
        assert!(packdata.is_empty());
    }
    #[test]
    fn test_frame_build_pack_tolong() {
        let datas: [u8; 256] = [0x11; 256];
        let result: Result<Vec<u8>, String> = pack_frame(&datas);
        assert!(result.is_err());
    }
    #[test]
    fn test_frame_build_pack_max() {
        let datas: [u8; 255] = [0x11; 255];
        let packdata: Vec<u8> = match pack_frame(&datas) {
            Ok(frame) => frame,
            Err(error) => {
                panic!("pack err reason：{}", error);
            }
        };
        assert_eq!(packdata.len(), 260);
        assert_eq!(packdata[2], 255);
        assert_eq!(&packdata[3..258], &datas[..]);
    }
    #[test]
    fn test_frame_crc_error() {
        let datas: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let mut packdata: Vec<u8> = match pack_frame(&datas) {
            Ok(frame) => frame,
            Err(error) => {
                panic!("pack err reason：{}", error);
            }
        };
        packdata[3] ^= 0x01;
        match unpack_frame(&packdata) {
            Err(UnpackError::CrcMismatch) => {}
            _ => {
                panic!("数据被修改后，应该报告CRC错误");
            }
        }
    }
    #[test]
    fn test_frame_error_and_right() {
        let datas: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let mut packdata: Vec<u8> = match pack_frame(&datas) {
            Ok(frame) => frame,
            Err(error) => {
                panic!("pack err reason：{}", error);
            }
        };
        packdata[3] ^= 0x01;
        let good_frame: Vec<u8> = match pack_frame(&datas) {
            Ok(frame) => frame,
            Err(error) => {
                panic!("pack err reason：{}", error);
            }
        };
        // 坏帧和好帧连续到达，保留同一个缓冲区中的全部字节。
        packdata.extend_from_slice(&good_frame);
        frame_55aa_buffer_process(&mut packdata);
        assert!(packdata.is_empty());
    }
}
