const FRAME_HEAD_AA: u16 = 0xAA55;
enum UnpackError {
    Incomplete,
    InvalidHead,
    InLengthShort,
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
    if recevicebuff.len() < 3 + data_len {
        return Err(UnpackError::Incomplete);
    }
    Ok(&recevicebuff[3..3 + data_len])
}
fn handle_payload(data: &[u8]) {
    println!("unpack length={}", data.len());
    println!("unpack successful{:02X?}", data);
}
fn frame_55aa_buffer_process(buffer: &mut Vec<u8>) {
    loop {
        if buffer.is_empty() {
            println!("the buffer is empty");
            break;
        }
        match unpack_frame(buffer) {
            Ok(value) => {
                let frame_len: usize = value.len() + 3;
                handle_payload(value);
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
        };
    }
}
pub fn unpack_run() {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend_from_slice(&[0xAA, 0x55, 0x02]);
    buffer.extend_from_slice(&[0x11, 0x12]);
    buffer.extend_from_slice(&[0xAA, 0x55, 0x01, 0x33]);
    buffer.extend_from_slice(&[0xff, 0xfa, 0xAA, 0x55, 0x01, 0x33]);
    frame_55aa_buffer_process(&mut buffer);
}

#[cfg(test)]
mod test {
    // use super::UnpackError;
    use super::frame_55aa_buffer_process;
    use super::unpack_frame;
    #[test]
    fn test_unpack_frame_length_err() {
        let test_buffer: Vec<u8> = vec![0xAA];
        let result = unpack_frame(&test_buffer);
        assert!(result.is_err());
    }
    #[test]
    fn test_unpack_frame_ok() {
        let test_buffer: Vec<u8> = vec![0xAA, 0x55, 0x02, 0x11, 0x44];
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
        let mut test_buffer: Vec<u8> =
            vec![0xAA, 0x55, 0x03, 0x11, 0x11, 0x11, 0xAA, 0x55, 0x02, 0x01];
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
        test_buffer.extend_from_slice(&[0x22]);
        frame_55aa_buffer_process(&mut test_buffer);
        assert!(test_buffer.is_empty());
    }
}
