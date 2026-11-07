# rust_study

Rust 学习代码和复盘文档，结合 C、AT32 MCU 和串口协议实践。

## 当前学习入口

当前 `src/main.rs` 运行命令行组帧练习，通过 `src/frame_tools.rs` 学习模块、结构体、借用、Result、错误枚举、CRC16 和单元测试。

```powershell
cargo run
cargo test
```

- [学习路线](doc/Rust学习路线.md)
- [学习进度](doc/学习进度.md)
- [复盘笔记](doc/Rust复盘笔记.md)

`src/main.rs` 中还保留了注释形式的早期练习和 GUI 入口。下方是此前已验证的图形监视器说明；当前入口不会启动 GUI，相关命令参数需恢复 GUI 入口后使用。当前练习已接入 CRC16，帧尾不再是占位字节。

## MPPT 串口监视器（此前实践）

一个用于学习 Rust 的 Windows 小界面，通过 USB 转 TTL 读取 MCU 的 Modbus RTU 数据。

## 启动

在项目目录执行：

```powershell
cargo run
```

选择串口（默认 COM14）、波特率 9600，点击“连接设备”。默认自动读取，也可以关闭“自动刷新”后点击“读取一次”。数据格式为 8N1、无流控，从站地址固定为 21（0x15）。

直接启动并连接默认端口：

```powershell
cargo run -- --connect
```

关闭窗口会退出程序并释放串口；串口调试助手等其他程序不能同时占用该端口。

首次编译图形库需要一些时间。仅当出现 CRYPT_E_REVOCATION_OFFLINE 证书吊销服务器不可达错误时，临时使用：

```powershell
cargo --config 'http.check-revoke=false' run
```

这个设置不会写入全局配置，未关闭 HTTPS 证书校验。

## 双击运行与发给别人

自己开发时用 `cargo run`。给别人用时，使用 Release 版本：

```powershell
cargo build --release
```

生成的程序是 `target/release/serial_rust.exe`，双击即可运行。Release 版不显示额外终端窗口。

本轮发布包为 `dist/MPPT-Monitor-win64-v0.1.0.zip`。解压后双击里面的 `serial_rust.exe`，选择对方电脑实际的 COM 端口，再点击“连接设备”。端口号不一定是 COM14。

接收者不需要安装 Rust、Cargo 或 VS Code。目标是 Windows x64；需要 USB 串口适配器对应驱动。连接设备应使用本文档的 9600/8N1、从站 0x15 协议。发布包不需要源代码或整个 target 目录。

已检查本次 exe 的导入表，它依赖 VCRUNTIME140.dll。若对方提示缺少此 DLL，应从 Microsoft 官方安装 Visual C++ v14 Redistributable（x64）；无需安装完整 Visual Studio。官方下载页：https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist 。

## 能显示什么

- 14 项保持寄存器，地址 0x0008～0x0015。
- 输入/电池/负载电压、电流、温度的换算值及原始整数、HEX。
- 最近有效数据的时间、读取成功/失败次数、最多 200 条收发日志。
- 超时、CRC 错误、地址/功能码/长度不匹配及 Modbus 异常应答。
- 读取失败时保留最近有效值，并提示它不代表当前实时数据。

当前协议全部只读，程序只发送功能码 03，不发送写寄存器命令。每次读取 14 个寄存器，请求为：

```text
15 03 00 08 00 0E 46 D8
```

一笔请求收到应答或超时后才会发下一笔；自动读取在上一笔结束后再间隔 1 秒。串口工作在后台线程，等待应答不会阻塞界面。

协议快照放在 [doc/local_communication.md](doc/local_communication.md)，复制自用户 MCU 项目提供的文档；该副本不会自动同步 MCU 文档变化。

## 特别注意字段含义

- “输入电流”当前实际映射 Buck 输出电流，并非 PV 输入电流。
- “负载电压”当前复用电池电压，并非负载端独立测量。
- 电池净电流是有符号数，正充电、负放电。
- 温度按有符号原始值除以 10 显示；固件也可能用 0 表示无效温度，目前无法区分真实 0℃ 和无效值。
- SOC、功率、日电量、充电状态、保护状态尚无来源/编码，界面显示“待接入”，同时保留原始值；不能把原始 0 解读成状态正常。
- 日电量的 ×10 编码尚未通过非零实机值验证，当前仍视为待接入字段。

## 从哪里看代码

建议按这个顺序阅读：

1. `src/protocol.rs`：CRC、请求组帧、应答检查、寄存器解码、单位换算。
2. `src/serial_worker.rs`：打开串口、write_all、分段接收、超时，以及后台命令/事件。
3. `src/app.rs`：按钮、数据显示、日志；通过消息与串口线程通信。
4. `src/main.rs`：程序入口，启动图形界面或终端探测模式。

CRC、数组、借用、Result、match 等知识可对照 [doc/Rust复盘笔记.md](doc/Rust复盘笔记.md)。线程、channel、结构体、trait、闭包是后面结合这份程序逐步学习的内容。

## 验证与排查

```powershell
# 协议和分段接收测试，不需要硬件
cargo test

# 终端只读取 COM14 一次，输出 TX/RX 与全部字段
cargo run -- --probe

# 离线示例界面，不连接串口；界面明确标为演示数据
cargo run -- --demo
```

8 项测试覆盖文档测试帧、标准 CRC 向量、有符号字段和单位、错误 CRC/地址/功能码/长度、异常应答、分段接收、部分数据及超时边界。

2026-11-03 已完成 COM14、9600/8N1 实机读取：收到通过 CRC 的 33 字节应答。终端读到的某次快照包括电池电压 13.197 V、电池净电流 -0.013 A、温度 27.9℃。图形界面自动读取也已成功，中文显示经过截图检查。数值会随设备测量变化。

开发时可用可选 screenshot feature 检查实际渲染；它不影响普通运行：

```powershell
$env:MPPT_SCREENSHOT_TO = "$PWD\target\ui-live.png"
cargo run --features screenshot -- --connect
Remove-Item Env:MPPT_SCREENSHOT_TO
```

开启此模式后，界面约 2 秒后截图并自动退出。截图功能由程序处理框架的 Screenshot 事件完成。
