pub enum BuildError {
    ZeroErr,
    CountTooLarge(u16),
}

// pub fn is_length_valid (length :u16)-> bool {
//     return length>0 && length<255;
// }

// pub fn build_frame(slave_id: u8, reg_adr: u16, count: u16) -> Result<[u8; 8], BuildError> {
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

// pub fn build_and_show(slave_id: u8, reg_adr: u16, count: u16) -> Result<(), BuildError> {
//     let frame: [u8; 8] = build_frame(slave_id, reg_adr, count)?;
//     println!("请求帧：{:02X?}", frame);
//     return Ok(());
// }
//

pub struct ReadRequest {
    pub slave_id: u8,
    pub reg_adr: u16,
    pub reg_length: u16,
}

pub fn build_frame(slave_info: &ReadRequest) -> Result<[u8; 8], BuildError> {
    let mut frame: [u8; 8] = [0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x00];
    if 0 == slave_info.reg_length {
        return Err(BuildError::ZeroErr);
    } else if slave_info.reg_length > 255 {
        return Err(BuildError::CountTooLarge(slave_info.reg_length));
    }
    frame[0] = slave_info.slave_id;
    frame[2..4].copy_from_slice(&slave_info.reg_adr.to_be_bytes());
    frame[4..6].copy_from_slice(&slave_info.reg_length.to_be_bytes());
    let crc: u16 = build_crc16(&frame[0..6]);
    frame[6..8].copy_from_slice(&crc.to_le_bytes());
    return Ok(frame);
}

pub fn build_and_show(slave_info: &ReadRequest) -> Result<(), BuildError> {
    let frame: [u8; 8] = build_frame(slave_info)?;
    println!("请求帧：{:02X?}", frame);
    return Ok(());
}

pub fn build_crc16(data: &[u8]) -> u16 {
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
#[cfg(test)]
mod tests {
    use super::build_frame;
    use crate::frame_tools::build_crc16;
    // use crate::frame_tools::tests;
    #[test]
    fn test_build_frame() {
        let request = super::ReadRequest {
            slave_id: 0x15,
            reg_adr: 0x0002,
            reg_length: 14,
        };
        let result = build_frame(&request);

        match result {
            Ok(frame) => {
                assert_eq!(frame, [0x15, 0x03, 0x00, 0x02, 0x00, 0x0E, 0x66, 0xDA]);
            }
            Err(_) => {
                panic!("合法参数不应该组帧失败");
            }
        }
    }
    #[test]
    fn test_zero_count() {
        let request = super::ReadRequest {
            slave_id: 0x15,
            reg_adr: 0x0000,
            reg_length: 0,
        };
        let result = build_frame(&request);

        match result {
            Err(super::BuildError::ZeroErr) => {
                // 得到了预期错误，测试通过
            }
            _ => {
                panic!("数量为 0 时，应该返回 ZeroErr");
            }
        }
    }
    #[test]
    fn test_max_count() {
        let request = super::ReadRequest {
            slave_id: 0x15,
            reg_adr: 11,
            reg_length: 255,
        };
        let result = build_frame(&request);
        assert!(result.is_ok());
    }
    #[test]
    fn test_mini_count() {
        let request = super::ReadRequest {
            slave_id: 0x15,
            reg_adr: 0x0008,
            reg_length: 1,
        };
        let result = build_frame(&request);
        match result {
            Ok(frame) => {
                assert_eq!(frame, [0x15, 0x03, 0x00, 0x08, 0x00, 0x01, 0x06, 0xDC]);
            }
            Err(_) => {
                panic!("合法参数不应该组帧失败");
            }
        }
    }
    #[test]
    fn test_upper_max_counter() {
        let request = super::ReadRequest {
            slave_id: 0x15,
            reg_adr: 11,
            reg_length: 256,
        };
        let result = build_frame(&request);
        match result {
            Err(super::BuildError::CountTooLarge(count)) => {
                assert_eq!(count, 256);
            }
            _ => panic!("256 应该返回 CountTooLarge"),
        }
    }
    #[test]
    fn test_crc() {
        let buffer: [u8; 8] = [0x15, 0x03, 0x00, 0x08, 0x00, 0x0E, 0x46, 0xD8];
        let crc16 = build_crc16(&buffer[0..6]);
        assert_eq!(crc16, 0xd846);
    }
    #[test]
    fn test_doc_frame() {
        let request = super::ReadRequest {
            slave_id: 0x15,
            reg_adr: 0x0008,
            reg_length: 0x0E,
        };
        let result = build_frame(&request);
        match result {
            Ok(frame) => assert_eq!(frame, [0x15, 0x03, 0x00, 0x08, 0x00, 0x0E, 0x46, 0xD8]),
            Err(_) => panic!("文档示例帧应该组帧成功"),
        }
    }
}
