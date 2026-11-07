pub enum BuildError {
    ZeroErr,
    CountTooLarge(u16),
}

// pub fn is_length_valid (length :u16)-> bool {
//     return length>0 && length<255;
// }

pub fn build_frame(slave_id: u8, reg_adr: u16, count: u16) -> Result<[u8; 8], BuildError> {
    let mut frame: [u8; 8] = [0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x00];
    if 0 == count {
        return Err(BuildError::ZeroErr);
    } else if count > 255 {
        return Err(BuildError::CountTooLarge(count));
    }
    frame[0] = slave_id;
    frame[2..4].copy_from_slice(&reg_adr.to_be_bytes());
    frame[4..6].copy_from_slice(&count.to_be_bytes());
    return Ok(frame);
}

pub fn build_and_show(slave_id: u8, reg_adr: u16, count: u16) -> Result<(), BuildError> {
    let frame: [u8; 8] = build_frame(slave_id, reg_adr, count)?;
    println!("请求帧：{:02X?}", frame);
    return Ok(());
}

#[cfg(test)]
mod tests {

use super::build_frame;

    #[test]
    fn test_build_frame() {
        let result = build_frame(0x15, 0x0002, 14);

        match result {
            Ok(frame) => {
                assert_eq!(frame, [0x15, 0x03, 0x00, 0x02, 0x00, 0x0E, 0x00, 0x00]);
            }
            Err(_) => {
                panic!("合法参数不应该组帧失败");
            }
        }
    }

    #[test]
    fn test_zero_count() {
        let result = super::build_frame(0x15, 0x0008, 0);

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
        let result = super::build_frame(0x15, 0x0008, 255);

        assert!(result.is_ok());
    }
}
