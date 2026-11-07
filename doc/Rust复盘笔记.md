# Rust 复盘笔记：结合 C 基础

更新日期：2026-11-07

这份笔记用于回顾已讲过的知识，后续随学习补充。学习到哪里、哪里还需巩固，见 [学习进度.md](学习进度.md)。

> Rust 的便利写法和语言规则要区分：类型推断、末尾表达式等是需要熟悉的写法；所有权、借用和类型检查则是理解代码行为的关键规则。

## 1. 声明变量：名字在前，类型在后

```c
// C
uint8_t data = 100;
```

```rust
// Rust
let data: u8 = 100;
```

- `let`：声明变量。
- `data`：自己取的变量名，没有特殊含义。
- `: u8`：变量的类型。
- `= 100`：初始值。

看到冒号，按“名字：类型”来读。函数参数也遵循这个顺序。

### mut 允许改值，不允许改变类型

```rust
let mut count: u8 = 1;
count = 2;
```

没有 `mut`，就不能给这个变量重新赋值。加了 `mut`，也不能把它改成字符串等其他类型。

## 2. 整数类型与字面值后缀

| Rust | 对应 C 的定宽类型 | 范围 |
|---|---|---|
| `u8` | `uint8_t` | 0～255 |
| `i8` | `int8_t` | -128～127 |

`u` 表示无符号，`i` 表示有符号，数字表示位数。

```rust
let data: u8 = 100;
let temperature: i8 = -10;
```

直接写 `let data: u8 = 256;`，默认会因字面值超范围而编译失败。这与运行期间做加法产生的整数溢出不是同一个测试，运行期间的溢出规则留到后面讲。

### C 的 0U 与 Rust 的 0u8

```c
uint8_t i = 0U;
```

Rust 不使用 `0U` 这个后缀，下面两种写法都可以：

```rust
let mut i: u8 = 0; // 从左侧类型确定 0 的类型
let mut j = 0u8;   // 从字面值后缀确定变量类型
```

- C 的 `0U` 是 `unsigned int` 类型的字面值，不是 `uint8_t`。
- Rust 的 `0u8` 明确是 `u8` 类型的字面值；类似地还有 `0u16`、`100u32`。
- 不需要修改变量时，去掉 `mut`。

## 3. bool 与条件判断

`bool` 的值是 `true` 或 `false`，不能直接用整数 `1` 或 `0` 赋值。

```rust
let test: u8 = 1;
let received: bool = 1 == test;
```

比较表达式 `1 == test` 的结果是 `bool`。保留 C 中明确比较状态值的习惯即可。

```rust
if 1 == test
{
    println!("收到数据");
}
else
{
    println!("未收到数据");
}
```

- 条件必须为 `bool`；整数不能直接写成 `if test`。
- 条件外不要求圆括号，分支花括号必须写。
- 花括号另起一行合法；默认格式化通常会把左花括号放到同一行。
- `else if` 与 C 的逻辑类似：从上往下判断，只执行首个条件成立的分支。

## 4. String 赋值：移动、复制与借用

```rust
let a = String::from("AT32");
```

这里创建了一个拥有堆上文本数据的 `String`。下面三种写法作用不同：

| 写法 | 做了什么 | 后面能否继续使用 a |
|---|---|---|
| `let b = a;` | 把 String 的所有权移给 b | 不能 |
| `let b = a.clone();` | 复制出独立的字符串内容 | 能 |
| `let b = &a;` | 共享借用原字符串 | 能，只读访问可共存 |

`u8` 等类型实现了 `Copy`，赋值时复制值，因此 `let b = a;` 后原整数变量仍可使用。`String` 不实现 `Copy`，同样的写法会移动所有权。

与 C 对照：`char *b = a;` 只是复制地址，两个指针仍指向同一份内容，并没有备份字符串。不要把“复制指针”当成“复制文本”。

## 5. mut 的位置：修改引用与修改数据

```rust
let mut b = &a;
```

`b` 是可重新赋值的共享引用变量，但不能通过它修改 `a` 的内容。

```rust
let mut a = String::from("AT32");
let b = &mut a;
b.push_str("_UART");
println!("a={}", a); // AT32_UART
```

这里 `b` 是可变引用，允许修改它引用的内容。`b` 自身没有被重新赋值，所以不需要写 `let mut b`。

`push_str` 表示追加文本。

### 借用的访问规则

- 共享引用 `&T`：可以同时有多个，用于读取。
- 可变引用 `&mut T`：在它有效期间独占所借用数据的访问，不能同时通过其他途径访问同一份数据。
- 这些是编译阶段的检查，不是运行时给变量上锁。

下面顺序可以通过编译：

```rust
let mut a = String::from("AT32");
let b = &mut a;
b.push_str("_UART"); // b 的最后一次使用
println!("a={}", a); // 本例中借用已结束
```

如果把 `println!("a={}", a)` 移到 `b.push_str(...)` 前面，就会与后面仍需使用的可变借用冲突。

引用的借用可以在最后一次使用后结束，不一定持续到所在代码块末尾；编译器会结合后续使用判断。

## 6. String、&String、str、&str

| 类型 | 当前阶段的理解 |
|---|---|
| `String` | 拥有文本数据、可增长的字符串类型；修改内容需要可变访问 |
| `&String` | 对 String 的共享引用 |
| `str` | 字符串切片类型，大小不由类型本身固定，不能直接作为普通按值参数 |
| `&str` | 字符串切片的共享引用，携带文本地址和字节长度 |

字符串字面值 `"UART"` 的类型是 `&str`。

自己遇到的错误：

```rust
let a = String::from("AT32");
let mut b = &a; // b 被推断为 &String
// b = "aas"; // 错误：需要 &String，实际给了 &str
```

如果想让 b 改为引用另一段文本，可以明确指定 `&str`：

```rust
let a = String::from("AT32");
let mut b: &str = &a;
println!("修改前：{}", b);
b = "aas";
println!("a={}, b={}", a, b); // a=AT32, b=aas
```

这里 `&String` 可以转换为需要的 `&str`。改变的是 b 引用哪段文本，a 的内容没有改变。

## 7. 函数参数：只读借用与可变借用

### 只读字符串参数通常用 &str

```rust
fn show_text(text: &str)
{
    println!("收到：{}", text);
}
```

参数名是 `text`，类型是 `&str`。下面两种调用都可以：

```rust
let name = String::from("AT32");
show_text(&name); // &String 转换为 &str
show_text("UART");
println!("{}", name); // 所有权没有移走
```

### 通过可变引用修改调用者的数据

下面是一个完整小例子：

```rust
fn show_num(text: &mut u8)
{
    *text = 2;
    println!("收到：{}", text);
}

fn main()
{
    let mut data1: u8 = 4;
    show_num(&mut data1);
    println!("修改后：{}", data1); // 2
}
```

三处配套看：

1. `let mut data1`：原变量允许修改。
2. `&mut data1`：可变借给函数。
3. `*text = 2`：解引用，修改原数字。

`text = 2` 是给引用变量本身赋整数，类型不匹配。调用 `text.push_str(...)` 等方法时，Rust 可以自动解引用，所以之前的方法调用没有显式写 `*`。

这与 C 传指针修改外部数据的用途类似，但这里准确叫“可变引用”，区别于 Rust 的裸指针。

## 8. 函数返回值

返回类型写在参数列表后，用 `->` 标注：

```rust
fn add_one(value: u8) -> u8
{
    return value + 1;
}
```

也可以用函数体末尾的表达式返回：

```rust
fn add_one(value: u8) -> u8
{
    value + 1
}
```

这两种写法都合法。如果第二种写成 `value + 1;`，它就不再作为末尾返回值，函数体结果为 `()`，与要求的 `u8` 不匹配。

## 9. 数组：类型包含元素类型和长度

```c
// C
uint8_t frame[4] = {0xAA, 0x01, 0x02, 0x55};
```

```rust
// Rust
let mut frame: [u8; 4] = [0xAA, 0x01, 0x02, 0x55];
frame[1] = 0x03;
println!("首字节={:02X}, 长度={}", frame[0], frame.len());
```

- `[u8; 4]` 是类型：4 个 u8 元素的数组；分号分隔元素类型与长度。
- `[0xAA, 0x01, 0x02, 0x55]` 是初始值列表。
- 下标从 0 开始；修改元素需要数组变量声明为 mut。
- `frame.len()` 获取元素个数。
- `{:02X}` 表示大写十六进制输出，至少两位，不足补零。
- 长度是数组类型的一部分，`[u8; 4]` 和 `[u8; 5]` 是不同类型。
- 不能越界访问；Rust 的安全数组索引会检查边界，越界可能在编译时被发现，或在运行时触发 panic。不要沿用 C 中越界访问内存的理解。

## 10. 切片参数：用同一个函数读取不同长度的数据

```rust
fn show_frame(data: &[u8])
{
    println!("数据长度={}", data.len());
    for byte in data
    {
        println!("{:02X}", byte);
    }
}

fn main()
{
    let frame1: [u8; 4] = [0xAA, 0x01, 0x02, 0x55];
    let frame2: [u8; 2] = [0xAA, 0x55];
    show_frame(&frame1);
    show_frame(&frame2);
    show_frame(&frame1[1..3]); // 只借用下标 1、2 的两个字节
}
```

- `data` 是参数名，`&[u8]` 是类型：u8 切片的共享引用，携带起始地址和元素个数。
- 与 `&[u8; 4]` 相比，`&[u8]` 不把固定长度 4 写进参数类型，能接收不同长度的数据。
- 用途类似 C 的 `const uint8_t *data, size_t len` 参数组合；切片把访问数据所需的地址和长度一起传递。
- `&frame1` 可以转换为函数需要的切片引用，不复制数组内容，也不转移数组所有权。
- `for byte in data` 依次借用每个元素，此处 byte 的类型是 `&u8`，可以直接用于上述格式化输出。
- `&frame1[1..3]` 借用一段数据：包含起点 1，不包含终点 3，内容是 01、02。范围必须有效。

### 数组元素与单元素切片不是同一种类型

| 表达式 | 类型 | 含义 |
|---|---|---|
| `frame1[0]` | `u8` | 第一个字节的值 |
| `&frame1[0]` | `&u8` | 第一个字节的共享引用 |
| `&frame1[0..1]` | `&[u8]` | 长度为 1 的切片引用 |

参数为 `u8` 时可以传 `frame1[0]`；参数为 `&[u8]` 时可以传 `&frame1[0..1]`。一个元素和长度为 1 的切片不能混用。

`[u8]` 与 `str` 一样是大小不由类型固定的不定长类型，不能直接作为普通按值参数。`&[u8]` 引用本身的大小固定，实际元素个数可以在运行时确定。

## 11. 安全取元素：get 与 Option

数据可能为空时，不应直接假定 `data[0]` 存在。`data.get(0)` 返回 `Option<&u8>`，明确表达“可能有一个字节，也可能没有”。

```rust
fn show_first(data: &[u8])
{
    match data.get(0)
    {
        Some(byte) => println!("首字节={:02X}", byte),
        None => println!("没有数据"),
    }
}

fn main()
{
    let frame: [u8; 2] = [0xAA, 0x55];
    let empty: [u8; 0] = [];

    show_first(&frame);
    show_first(&empty);
}
```

- `get(0)` 在元素存在时得到 `Some(元素引用)`，不存在时得到 `None`，不会因该索引越界而 panic。
- `Option` 是表示“有或无”的枚举类型；`None` 不是数字 0，字节值 0 也会得到 `Some(&0)`。
- `match` 匹配返回结果；`Some(byte)` 中的 byte 是匹配成功时绑定的变量名，本例类型为 `&u8`。
- `=>` 右边是该分支要执行的表达式，各分支用逗号分隔；这里的两个分支覆盖了所有可能。
- `data[0]` 直接索引在空切片上会触发 panic，不能当作 C 中返回错误码或读取到默认值来理解。

## 12. 串口实践前的路线

目标：先在 Windows 上通过 USB 串口与设备通信，现有 MCU 程序仍使用 C。

- 先添加 serialport 依赖并枚举串口，再逐步打开端口、发送和接收字节。
- 串口操作可能失败，需要学习 Result：Ok(成功结果)、Err(错误)，用 match 分别处理。与 Option 的“有或无”区别开。
- 读取数据时传入可变缓冲区，成功后只处理实际读取长度 n 对应的 &buf[..n]。
- 一次 read 不保证恰好读到一整帧，协议组帧留到基础收发打通后再做。
- use、std::io::Read / Write、Duration 和 Vec 等语法或类型，在实际用到时逐项解释，先不展开泛型、trait、异步等完整理论。
- 实机收发前需确认端口号、波特率、数据位、校验位、停止位，以及对端是 MCU、另一只串口还是回环连接。

### 已遇到的依赖下载问题：证书吊销检查服务器不可达

执行 cargo add serialport 时，遇到 schannel 的 CRYPT_E_REVOCATION_OFFLINE 错误。这是 Windows 证书吊销检查服务器不可达，不是 Rust 代码错误。

本次通过单次命令覆盖解决：

```text
cargo --config 'http.check-revoke=false' add serialport
cargo --config 'http.check-revoke=false' check
```

这只对对应命令关闭证书吊销检查，仍保留 HTTPS 证书校验；未写入项目或全局 Cargo 配置。遇到同类错误时可临时使用，不作为日常默认设置。

本次已加入 serialport = "4.10.1"，并通过 cargo check。后续编写串口枚举示例。

### 第一个串口实验：枚举串口

```rust
fn main()
{
    match serialport::available_ports()
    {
        Ok(ports) =>
        {
            println!("找到 {} 个串口", ports.len());
            for port in ports
            {
                println!("串口：{}", port.port_name);
            }
        }
        Err(error) =>
        {
            println!("枚举串口失败：{}", error);
        }
    }
}
```

- `serialport::available_ports()` 调用库提供的函数，返回 Result。
- `Ok(ports)`：枚举成功，ports 是串口信息列表；列表可能为空，空列表不等于失败。
- `Err(error)`：枚举失败，error 是错误信息。
- Ok、Err 是 Result 的固定变体名；ports、error 是匹配时自己取的变量名。
- `port.port_name` 读取串口信息中的端口名称字段，例如 COM3。
- 该实验只列出串口，不打开端口、不发送数据；枚举到不保证稍后一定能够打开。
- 对比插入与拔掉 USB 串口时的输出，有助于辨认目标端口。

实际运行已成功找到 6 个端口：COM14、COM9、COM12、COM11、CNCA0、CNCB0。学习者确认使用 COM14、9600，USB 转 TTL 接 MCU。

### 第二个串口实验：打开 COM14

```rust
use std::time::Duration;

fn main()
{
    let result = serialport::new("COM14", 9600)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .flow_control(serialport::FlowControl::None)
        .timeout(Duration::from_millis(100))
        .open();

    match result
    {
        Ok(_port) => println!("COM14 打开成功，波特率 9600"),
        Err(error) => println!("打开失败：{}", error),
    }
}
```

- 本实验先使用 8 数据位、无校验、1 停止位、无流控；学习者尚未确认 MCU 的数据格式，后续收发必须与 MCU 配置一致。
- `use std::time::Duration` 引入标准库的时间间隔类型。
- `serialport::new` 创建配置，各行 .方法(...) 接着设置参数，最后 .open() 才真正尝试打开串口，返回 Result。
- `Duration::from_millis(100)` 是 100 毫秒的读写超时设置，不是每 100 毫秒接收一次，也不是 open 的等待时间。
- `_port` 绑定打开的串口对象；名称以 _ 开头表示当前有意不使用它。分支结束时对象被释放、端口关闭。
- 此实验只验证打开，尚不收发；打开成功不代表已经验证 MCU 通信成功。
- 如打开失败，结合错误检查端口是否仍存在，以及是否被串口调试助手等程序占用。

### 第三个串口实验：发送字节

COM14 打开实验已成功。原代码顶部增加：

```rust
use std::io::Write;
```

将原来的 Ok(_port) 成功分支替换为：

```rust
Ok(mut port) =>
{
    let frame: [u8; 4] = [0xAA, 0x01, 0x02, 0x55];

    match port.write_all(&frame)
    {
        Ok(()) => println!("4 个字节已写入串口"),
        Err(error) => println!("发送失败：{}", error),
    }
}
```

- `Write` 是提供写入方法的 trait，此处先理解为引入 write_all 等写入能力。
- `Ok(mut port)` 在匹配打开成功时绑定可变串口对象，写入方法需要可变访问。
- `write_all` 接收 &[u8]，尝试把整个切片写完，失败则返回错误；不像单次 write 只承诺返回实际写入的部分长度。
- 返回类型为 Result<(), std::io::Error>，成功没有额外数据，用 Ok(()) 匹配；() 是单元值，不表示数字 0。
- 发送的是 4 个原始字节，不是字符串 "AA 01 02 55"。
- 写入成功只表示接口已接受全部数据，不保证 MCU 已接收或已执行。需要等待发送队列排空时，可进一步调用并处理 flush()；收发验证仍需对端反馈。

### 已完成的协议与图形界面

根据 MCU 项目的 local_communication.md 实现了只读监视器，运行方式和代码入口见项目 README.md。COM14 实机终端读取和界面自动读取均已成功。

- `protocol.rs`：将 CRC、组帧、解析从串口代码中独立出来，便于无硬件测试。
- 请求中的寄存器地址/数量、应答寄存器值用大端字节序，CRC 用小端字节序。
- `u16::from_be_bytes([high, low])` 把高低两个字节还原成寄存器。
- 将寄存器 u16 转为 i16，可按本协议解释 16 位补码，处理负电池电流、负温度。
- `serial_worker.rs`：通过 Vec 累积 read 返回的数据，直到收齐 33 字节正常应答或 5 字节异常应答；不能假定一次 read 收到一整帧。
- 串口 read 的短超时与整笔请求的 1 秒截止时间分开处理；部分应答也写入日志。
- `Result<T, String>` 表示成功返回 T、失败返回说明文本。`?` 在此类函数中遇错会提前返回错误，成功时取出内部值；这项语法后续结合代码展开。
- 后台线程、channel、struct、impl、trait、闭包等已用于程序；后续已练习 struct/impl，线程、channel、trait、闭包尚未系统教学。
- 占位字段只显示“待接入”，保留原始值供调试，不将 0 解读成有效测量或正常状态。

### 编译与分发

- `cargo run`：编译并运行调试版，适合学习和排错。
- `cargo build --release`：生成优化后的发布程序，位置为 target/release/serial_rust.exe。
- 别人不需要 Rust 或 VS Code，解压发布包后双击 exe；需使用 Windows x64 和对应 USB 串口驱动，并选择自己电脑上的 COM 端口。
- 代码中的 cfg_attr 属性让 Windows Release 版使用图形子系统，双击时不显示额外终端；调试版仍保留终端。

## 13. 读报错时先看什么

| 已遇到的报错 | 先检查 |
|---|---|
| `E0382: borrow of moved value` | 是否已经把 String 等非 Copy 值的所有权移走，又使用了原变量 |
| `E0308: mismatched types` | `expected` 是需要的类型，`found` 是实际提供的类型 |
| 不能通过共享引用修改数据 | 是否需要 &mut；原变量是否允许修改 |
| 借用冲突 | 可变引用后面是否还要使用，此时是否又通过其他途径访问了同一份数据 |

先找出“这一行变量的类型是什么”和“它此时是拥有、共享借用，还是可变借用”，再看怎么改。

## 14. 枚举、方法与所有权

```rust
enum Portstate {
    Disconnection,
    Connection(String),
    Failed(String),
}
```

三种值都是 Portstate，区别是变体及携带的数据。枚举变体按大驼峰命名；定义后没有创建过的变体可能产生 dead_code 警告。

```rust
let state = Portstate::Connection(String::from("COM14"));
match &state {
    Portstate::Connection(name) => println!("已连接：{}", name),
    Portstate::Disconnection => println!("未连接"),
    Portstate::Failed(reason) => println!("失败：{}", reason),
}
```

- match &state 时，name/reason 是 &String，只借来读取。
- match state 并绑定内部 String 时，字符串会移动给分支变量；不能再把部分内容移走的 state 当完整值使用，但可以重新给整个 state 赋值。
- `_ => {}` 表示匹配剩余情况，什么也不做；单表达式分支可以省花括号，用逗号分隔。
- impl 中的方法仍然是函数。&self 用于读取，&mut self 用于修改，点号调用自动传入接收者。

```rust
impl Portstate {
    fn set_failed(&mut self, reason: &str) {
        *self = Portstate::Failed(String::from(reason));
    }
}
```

参数也可以用 &String，再通过 reason.clone() 创建独立 String。只需要读取文本时，&str 通常更通用；它可以接字符串字面值和由 &String 转换来的引用。克隆后修改原 reason 不会改变状态里保存的副本；替换整个状态时，旧状态携带的 String 自动清理。

## 15. Result：整个结果和内部数据的类型不同

```rust
pub enum BuildError {
    ZeroErr,
    CountTooLarge(u16),
}
```

| 写法 | 含义 |
| --- | --- |
| Result<[u8; 8], BuildError> | 成功装数组，失败装 BuildError |
| Ok(frame) | 成功结果，里面是 [u8; 8] |
| Err(BuildError::ZeroErr) | 失败，数量为零 |
| Err(BuildError::CountTooLarge(count)) | 失败，并携带实际数量 |

```rust
let result: Result<[u8; 8], BuildError> = build_frame(0x15, 0x0008, 14);
match result {
    Ok(frame) => println!("请求帧：{:02X?}", frame),
    Err(BuildError::ZeroErr) => println!("数量不能为 0"),
    Err(BuildError::CountTooLarge(count)) => println!("数量过大：{}", count),
}
```

- 原来的 frame 若是 [u8; 8]，不能直接赋给它 Result；先接住 result，再解构出数组。
- let result = ... 是编译期类型推断，不是运行时类型可随意变化；学习时可以显式标注。
- 错误枚举通常只列失败原因，成功已由 Ok 表示。定义 NoErr 后，Err(NoErr) 也是类型允许的值，match 同样需要处理。
- Some/None 属于 Option，Ok/Err 属于 Result，都是枚举变体；构造时放入数据，匹配时取出数据，不是日志函数。

### Debug 与警告

```rust
#[derive(Debug)]
enum ReadError {
    TooShort(usize),
}
```

derive(Debug) 自动实现调试格式支持，可用 println!("{:?}", ReadError::TooShort(1))，输出 TooShort(1)。它不自动提供 {} 所需的 Display。

单个暂时不用的函数可在前面加 #[allow(dead_code)]；main.rs 顶部的 #![allow(dead_code)] 范围更大。不是所有警告都必须压掉，先理解原因。

### 单元类型与问号

- Result<(), BuildError>：成功时没有额外数据，以 Ok(()) 返回；失败仍携带错误。
- () 是单元类型，也有一个写作 () 的值，不是数字 0。

```rust
fn build_and_show(slave: u8, address: u16, count: u16) -> Result<(), BuildError> {
    let frame: [u8; 8] = build_frame(slave, address, count)?;
    println!("请求帧：{:02X?}", frame);
    return Ok(());
}
```

对这里相同错误类型的 Result，? 相当于：成功取出内部值继续，失败立刻从 build_and_show 返回 Err。它不打印，也不直接结束整个程序；main 的 match 仍可接到并打印错误。返回类型必须支持相应错误传播，不能随意在返回 () 的普通 main 中添加这种 ?。

若只是原样转交结果，直接 return build_frame(...) 即可；成功后还需进一步处理时，? 才能减少重复的 match。此处按当前例子理解，错误类型转换等细节以后再补。

## 16. Vec：自动管理内存的动态数组

```rust
let mut buffer: Vec<u8> = Vec::new();
buffer.push(0x12);
buffer.extend_from_slice(&[0x34, 0xAA]);
```

| 操作 | 作用 |
| --- | --- |
| vec![0, 10] | 创建两个元素：0、10 |
| vec![0; 10] | 创建十个元素，初值全为 0 |
| len() | 已有元素数量 |
| capacity() | 当前空间可容纳的元素数量，不是字节数量 |
| push(value) | 末尾添加一个元素 |
| extend_from_slice(&data) | 复制切片中的元素，追加到末尾 |
| clear() | 清空元素，长度归零，保留容量 |

Vec::new() 不为元素分配堆空间；增加非零大小的元素时按需要分配、扩容，最终自动释放。它不是无限内存。Vec<T> 可保存自定义结构体，同一个 Vec 的元素类型一致；extend_from_slice 要求元素类型实现 Clone，push 则直接接收并放入元素。

普通数组也有 [0, 10] 和 [0; 10] 两种初始化语法，但数组长度固定。let data: [u8] = ... 不行；普通数组写 [u8; N]，切片引用写 &[u8]。

## 17. 安全访问与遍历：分清下标和元素引用

```rust
let mut buffer: Vec<u8> = vec![0x12, 0x34];
let index: usize = 1;
if let Some(byte) = buffer.get_mut(index) {
    *byte = 0xAA;
    println!("下标={}，值={:02X}", index, *byte);
} else {
    println!("下标无效：{}", index);
}
```

- get(index) 返回 Option<&u8>，get_mut(index) 返回 Option<&mut u8>。
- get 不会因索引无效而 panic，而是返回 None；字节值为 0 仍是 Some(&0)。
- is_some() 只判断是否有值；if let Some(byte) = ... 同时匹配并绑定内部值。
- get_mut 不返回下标，若还需打印位置，先将下标保存在变量里。

```rust
for (index, byte) in buffer.iter_mut().enumerate() {
    *byte += 1;
    println!("index={}，value={:02X}", index, *byte);
}
```

- iter_mut() 返回迭代器，每轮提供 &mut u8，不是整段数组，也不是独立复制的 u8。
- enumerate 从 0 开始计数，依次生成 (usize, &mut u8)；for 用 (index, byte) 解构。
- byte[index] 不行，byte 只引用一个数字；用 *byte 读取或修改它。
- 只读遍历用 iter()；只有下标时，可用 for index in 0..buffer.len() 再访问 buffer[index]。len() 本身是数量，不能直接当作可遍历序列。

## 18. 切片范围、原数据与借用结束

```rust
let mut buffer: Vec<u8> = vec![0x15, 0x03, 0x02, 0x12, 0x34];
{
    let data: &mut [u8] = &mut buffer[3..5];
    data[0] = 0xAA;
    data[1] = 0xA2;
    println!("切片：{:02X?}，长度={}", data, data.len());
}
println!("整体：{:02X?}", buffer);
// [15, 03, 02, AA, A2]
```

- 切片是一段连续数据；此处 &mut [u8] 是切片的可变引用，包含起始地址和元素数量，没有复制字节。
- data[0]/data[1] 对应 buffer[3]/buffer[4]，切片自己的下标从 0 开始。
- 修改原数据的权限来自 &mut，data 自身未重新赋值时不必写 let mut data。
- 3..5 包含下标 3、4；3..=5 还包含下标 5。长度为 5 时，后者越界。..2 等同 0..2。
- 范围有效时，范围长度为 end - start；buffer.get(3..5) 返回 Option<&[u8]>，无效范围返回 None，直接索引则会 panic。
- data 后面还需使用时，不能中途通过 buffer.iter()/iter_mut() 或打印 buffer 再访问同一份数据。通过 data 操作即可，或将整体访问放到其最后一次使用后。显式代码块有助于看清范围。
- 引用可用 C 的指针用途辅助理解，但 Rust 的 &T 不是裸指针；&[u8] 也不是只有地址，它还携带长度并受借用规则检查。

## 19. fill、覆盖复制与字节序组帧

```rust
fn fill_byte(data: &mut [u8], value: u8) {
    data.fill(value);
}
```

fill 将已有元素设为同一个值，不改变长度；方法调用会自动借用接收者，因此可直接写 buffer[3..5].fill(0xAA)。

```rust
let mut frame: [u8; 8] = [0x15, 0x03, 0, 0, 0, 0, 0, 0];
let address: u16 = 0x1234;
let count: u16 = 14;
frame[2..4].copy_from_slice(&address.to_be_bytes());
frame[4..6].copy_from_slice(&count.to_be_bytes());
```

- copy_from_slice 是覆盖已有位置，不是插入或追加，数组长度不变。
- 来源与目标长度必须相同：两个字节要复制到 2..4，不能用只有一个字节的 2..3。已遇到的 panic：source slice length (2) does not match destination slice length (1)。
- to_be_bytes 把整数拆成高字节在前的数组，from_be_bytes 反向组合；le 对应低字节在前。
- u8 只有一个字节，frame[0] = slave_id 即可。
- 返回 [u8; 8] 是直接返回整个数组，不必传 C 式输出指针。给已有 frame 赋新数组会替换所有八个元素。
- 练习曾把示例数值 0x7890 用 to_le_bytes 放到 6..8，得到 90、78；这只验证放置顺序，不表示它是该帧的有效 CRC。
- 当前 build_frame 的 1～255 数量限制是教学规则，真实设备应依据协议检查；帧尾尚未接入 CRC 计算，不能据此当作完整请求发送。

## 20. 模块与路径

| 写法 | 作用 |
| --- | --- |
| mod frame_tools; | 声明模块，常规布局下关联 frame_tools.rs |
| pub fn build_frame(...) | 允许模块外访问函数 |
| pub enum BuildError | 允许模块外使用错误类型 |
| frame_tools::build_frame(...) | 带模块前缀调用 |
| use crate::frame_tools::build_frame; | 引入函数名，以后可直接调用 |
| super::build_frame(...) | 从当前模块的父模块寻找函数 |

use 不执行或复制函数，只把名字引入当前作用域；crate 指当前 crate 的根，super 指父模块，不是测试专用语法。学习者当前偏好保留模块前缀，便于定位来源。

### 最新布置：公开请求结构体（尚未反馈完成）

```rust
pub struct ReadRequest {
    pub slave_id: u8,
    pub reg_addr: u16,
    pub count: u16,
}
```

pub struct 不自动公开字段。练习是在 main 中创建 frame_tools::ReadRequest，再用 request.slave_id 等字段调用原 build_frame；下一步才将函数参数改成 &ReadRequest。

## 21. 单元测试与边界检查

放在 frame_tools.rs 中的一个独立测试示例：

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_build_frame() {
        let result = super::build_frame(0x15, 0x0008, 14);
        match result {
            Ok(frame) => {
                assert_eq!(frame, [0x15, 0x03, 0, 8, 0, 14, 0, 0]);
            }
            Err(_) => panic!("合法参数不应组帧失败"),
        }
    }
}
```

- #[cfg(test)]：只在测试配置下编译这个模块；#[test] 标记由测试工具运行的函数，不必从 main 调用。
- assert_eq!(实际值, 预期值) 检查相等，数组按元素比较；失败时 panic 并显示 left/right，既不赋值，也不修改数组。
- 输入地址改为 0x0002 后，预期字节也必须改成 00、02。函数返回 Ok 不代表测试预期一定相等。
- assert!(条件) 检查 bool 为 true；Result 的 is_ok()/is_err() 分别检查成功和失败。
- 错误分支测试匹配到预期的 Err 才算通过；只用 is_err() 不区分具体错误，需精确检查时用 match。
- 正常值 14、边界值 1/255、非法值 0/256 对应当前教学规则。优先选择有意义的测试，不必穷举所有输入。
- cargo test 运行测试；见到成功和故意制造的断言失败，能确认测试确实在检查结果。正常组帧测试曾因输入/预期不一致失败；首次 Git 提交前已验证正常帧、零数量和最大数量 255 三项测试全部通过，1、256 的独立用例尚未添加。
- 单元测试检查函数逻辑，不能代替串口实机收发、超时、断线等系统验证。

格式化暂采用 rustfmt 默认约 100 字符行宽，无需为本次学习另外设置 80 字符限制。
