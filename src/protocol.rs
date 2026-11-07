//! 只负责字节和寄存器，不依赖串口或界面。
//! 对照 doc/local_communication.md 阅读即可。

pub const SLAVE: u8 = 0x15;
pub const FIRST_REGISTER: u16 = 0x0008;
pub const REGISTER_COUNT: usize = 14;

#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub registers: [u16; REGISTER_COUNT],
}

/// 与 MCU 文档里的 C 实现相同：CRC 初值 FFFF，多项式 A001。
pub fn crc16(data: &[u8]) -> u16 {
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

pub fn read_request() -> [u8; 8] {
    let mut frame = [SLAVE, 0x03, 0, 0, 0, REGISTER_COUNT as u8, 0, 0];
    frame[2..4].copy_from_slice(&FIRST_REGISTER.to_be_bytes());
    let crc = crc16(&frame[..6]);
    frame[6..8].copy_from_slice(&crc.to_le_bytes());
    return frame;
}

pub fn parse_response(frame: &[u8]) -> Result<Snapshot, String> {
    if frame.len() < 5 {
        return Err("应答不足 5 字节".into());
    }
    let end = frame.len() - 2;
    let received_crc = u16::from_le_bytes([frame[end], frame[end + 1]]);
    if crc16(&frame[..end]) != received_crc {
        return Err("CRC 校验失败，检查连线、干扰及串口参数".into());
    }
    if frame[0] != SLAVE {
        return Err(format!("从站地址不匹配：收到 0x{:02X}", frame[0]));
    }
    if frame[1] == 0x83 {
        if frame.len() != 5 {
            return Err("异常应答长度应为 5 字节".into());
        }
        let message = match frame[2] {
            1 => "不支持的功能码",
            2 => "寄存器地址非法或越界",
            3 => "读取数量非法",
            4 => "测量快照不可用",
            _ => "未定义的设备异常",
        };
        return Err(format!("设备异常 0x{:02X}：{message}", frame[2]));
    }
    if frame[1] != 0x03 {
        return Err(format!("功能码不匹配：0x{:02X}", frame[1]));
    }
    if usize::from(frame[2]) != REGISTER_COUNT * 2 || frame.len() != 5 + REGISTER_COUNT * 2 {
        return Err(format!(
            "读取 14 个寄存器应返回 28 字节数据、33 字节整帧，实际整帧 {} 字节",
            frame.len()
        ));
    }
    let mut registers = [0u16; REGISTER_COUNT];
    for (index, value) in registers.iter_mut().enumerate() {
        let offset = 3 + index * 2;
        *value = u16::from_be_bytes([frame[offset], frame[offset + 1]]);
    }
    return Ok(Snapshot { registers });
}

pub fn hex(data: &[u8]) -> String {
    return data
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<Vec<_>>()
        .join(" ");
}

pub struct RegisterInfo {
    pub name: &'static str,
    pub unit: &'static str,
    pub signed: bool,
    pub divisor: f64,
    pub pending: bool,
    pub note: &'static str,
}

pub const REGISTERS: [RegisterInfo; REGISTER_COUNT] = [
    RegisterInfo {
        name: "输入电压",
        unit: "V",
        signed: false,
        divisor: 1000.0,
        pending: false,
        note: "原始单位 mV",
    },
    RegisterInfo {
        name: "输入电流",
        unit: "A",
        signed: false,
        divisor: 1000.0,
        pending: false,
        note: "当前映射 Buck 输出电流，并非 PV 输入电流",
    },
    RegisterInfo {
        name: "电池电压",
        unit: "V",
        signed: false,
        divisor: 1000.0,
        pending: false,
        note: "原始单位 mV",
    },
    RegisterInfo {
        name: "电池净电流",
        unit: "A",
        signed: true,
        divisor: 1000.0,
        pending: false,
        note: "正值充电，负值放电；16 位补码",
    },
    RegisterInfo {
        name: "负载电压",
        unit: "V",
        signed: false,
        divisor: 1000.0,
        pending: false,
        note: "当前复用电池电压，并非负载端独立测量",
    },
    RegisterInfo {
        name: "负载电流",
        unit: "A",
        signed: false,
        divisor: 1000.0,
        pending: false,
        note: "原始单位 mA",
    },
    RegisterInfo {
        name: "温度",
        unit: "℃",
        signed: true,
        divisor: 10.0,
        pending: false,
        note: "原始值 ÷ 10；无效温度也可能填 0",
    },
    RegisterInfo {
        name: "SOC",
        unit: "%",
        signed: false,
        divisor: 1.0,
        pending: true,
        note: "固件占位，当前没有数据来源",
    },
    RegisterInfo {
        name: "充电功率",
        unit: "W",
        signed: false,
        divisor: 1.0,
        pending: true,
        note: "固件占位，当前没有数据来源",
    },
    RegisterInfo {
        name: "放电功率",
        unit: "W",
        signed: false,
        divisor: 1.0,
        pending: true,
        note: "固件占位，当前没有数据来源",
    },
    RegisterInfo {
        name: "日充电量",
        unit: "Wh",
        signed: false,
        divisor: 10.0,
        pending: true,
        note: "固件占位；协议标注 ×10 编码",
    },
    RegisterInfo {
        name: "日放电量",
        unit: "Wh",
        signed: false,
        divisor: 10.0,
        pending: true,
        note: "固件占位；协议标注 ×10 编码",
    },
    RegisterInfo {
        name: "充电状态",
        unit: "",
        signed: false,
        divisor: 1.0,
        pending: true,
        note: "占位；状态编码尚未定义，不解释为正常",
    },
    RegisterInfo {
        name: "保护状态",
        unit: "",
        signed: false,
        divisor: 1.0,
        pending: true,
        note: "占位；状态编码尚未定义，不解释为正常",
    },
];

impl Snapshot {
    pub fn raw_value(&self, index: usize) -> i32 {
        if REGISTERS[index].signed {
            return i32::from(self.registers[index] as i16);
        }
        return i32::from(self.registers[index]);
    }

    pub fn display_value(&self, index: usize) -> String {
        let info = &REGISTERS[index];
        if info.pending {
            return "待接入".into();
        }
        let value = f64::from(self.raw_value(index)) / info.divisor;
        if info.unit == "℃" {
            return format!("{value:.1} {}", info.unit);
        }
        return format!("{value:.3} {}", info.unit);
    }
}

#[cfg(test)]
pub fn test_response(registers: [u16; REGISTER_COUNT]) -> Vec<u8> {
    let mut bytes = vec![SLAVE, 3, 28];
    for value in registers {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    let crc = crc16(&bytes);
    bytes.extend_from_slice(&crc.to_le_bytes());
    return bytes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_matches_firmware_document() {
        assert_eq!(read_request(), [0x15, 3, 0, 8, 0, 14, 0x46, 0xD8]);
        assert_eq!(crc16(b"123456789"), 0x4B37);
    }

    #[test]
    fn decodes_big_endian_signed_values_and_scales() {
        let mut registers = [0u16; REGISTER_COUNT];
        registers[0] = 24567;
        registers[3] = (-1234i16) as u16;
        registers[6] = (-125i16) as u16;
        let data = parse_response(&test_response(registers)).unwrap();
        assert_eq!(data.registers, registers);
        assert_eq!(data.display_value(0), "24.567 V");
        assert_eq!(data.display_value(3), "-1.234 A");
        assert_eq!(data.display_value(6), "-12.5 ℃");
        assert_eq!(data.display_value(7), "待接入");
    }

    #[test]
    fn rejects_corruption_wrong_address_and_bad_lengths() {
        let good = test_response([0; REGISTER_COUNT]);
        let mut corrupt = good.clone();
        corrupt[3] ^= 1;
        assert!(parse_response(&corrupt).unwrap_err().contains("CRC"));
        for bytes in [&good[..4], &good[..32], &[]] {
            assert!(parse_response(bytes).is_err());
        }
        for (index, value) in [(0, 0x16), (1, 0x06), (2, 26)] {
            let mut invalid = good.clone();
            invalid[index] = value;
            let end = invalid.len() - 2;
            let crc = crc16(&invalid[..end]);
            invalid[end..].copy_from_slice(&crc.to_le_bytes());
            assert!(parse_response(&invalid).is_err());
        }
    }

    #[test]
    fn reports_modbus_exception() {
        let mut frame = vec![SLAVE, 0x83, 4];
        let crc = crc16(&frame);
        frame.extend_from_slice(&crc.to_le_bytes());
        assert!(
            parse_response(&frame)
                .unwrap_err()
                .contains("测量快照不可用")
        );
    }
}
