/*  head functioncode length data_h data_l ...datan_h datan_l....caculate
 *  [0]  [1]          [2]    [3]   [4]                          [length+3]
 */
const PROTOCOL_RECOVER_HEAD: u8 = 0xA8;
enum ProtocolError {
    HeadCodeError,
    FunctionCodeError,
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
fn protocol_recover_unpack(data_of_in: &[u8]) -> Result<&[u8], ProtocolError> {
    // check the recevice length err
    if data_of_in.len() < 3 {
        return Err(ProtocolError::InLengthError);
    }
    // check the data lenth is valid
    let data_length: usize = usize::from(data_of_in[2]);
    let frame_length: usize = data_length + 4;
    if data_of_in.len() < frame_length {
        return Err(ProtocolError::PackLengthError);
    }
    // check head is valid
    if PROTOCOL_RECOVER_HEAD != data_of_in[0] {
        return Err(ProtocolError::HeadCodeError);
    }
    // check validation is valid
    let input_anti_value: u8 = data_of_in[frame_length - 1];
    let anti_value: u8 = protocol_recover_anti_sum(&data_of_in[..frame_length - 1]);
    if anti_value != input_anti_value {
        return Err(ProtocolError::ValidationError);
    }
    return Ok(&data_of_in[3..3 + data_length]);
}
