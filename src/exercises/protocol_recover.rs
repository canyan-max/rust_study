/*  head  length data_h data_l ...datan_h datan_l....caculate
 *  [0]    [1]    [2]   [3]                          [length+2]
 */
const PROTOCOL_RECOVER_HEAD: u8 = 0xA8;
enum ProtocolError {
    HeadCodeError,
    InLengthError,
    PackLengthError,
    ValidationError,
}

fn protocol_recover_anti_sum(buffer: &[u8]) -> u8 {
    let mut anti_sum: u8 = 0;
    for index in 0..buffer.len() {
        anti_sum = anti_sum.wrapping_add(buffer[index]);
    }
    anti_sum = (!anti_sum).wrapping_add(1);
    anti_sum
}

fn protocol_recover_pack(buffer: &[u8]) -> Result<Vec<u8>, ProtocolError> {
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
    if PROTOCOL_RECOVER_HEAD != data_of_in[0] {
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
fn payload() {}
pub fn protocol_recover_process(buffer: &mut Vec<u8>) {
    loop {
        match protocol_recover_unpack(buffer) {
            Ok(frame) => {
                let frame_length = frame.len() + 3;
                println!("successful：{:02X?}", frame);
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
}

#[cfg(test)]
mod test {
    // #[warn(unused)]
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
}
