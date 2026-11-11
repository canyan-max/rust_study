# Rust 复盘笔记：结合 C 基础

更新日期：2026-11-11

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

## 12. 串口基础操作与依赖排错

目标：先在 Windows 上通过 USB 串口与设备通信，现有 MCU 程序仍使用 C。

- 串口操作可能失败，需要学习 Result：Ok(成功结果)、Err(错误)，用 match 分别处理。与 Option 的“有或无”区别开。
- 读取数据时传入可变缓冲区，成功后只处理实际读取长度 n 对应的 &buf[..n]。
- 一次 read 不保证恰好读到一整帧，协议组帧留到基础收发打通后再做。
- 实机收发前需确认端口号、波特率、数据位、校验位、停止位，以及对端是 MCU、另一只串口还是回环连接。

### 已遇到的依赖下载问题：证书吊销检查服务器不可达

执行 cargo add serialport 时，遇到 schannel 的 CRYPT_E_REVOCATION_OFFLINE 错误。这是 Windows 证书吊销检查服务器不可达，不是 Rust 代码错误。

本次通过单次命令覆盖解决：

```text
cargo --config 'http.check-revoke=false' add serialport
cargo --config 'http.check-revoke=false' check
```

这只对对应命令关闭证书吊销检查，仍保留 HTTPS 证书校验；未写入项目或全局 Cargo 配置。遇到同类错误时可临时使用，不作为日常默认设置。

项目已加入 serialport 依赖；下面是枚举和打开串口的练习示例。

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

### 公开请求结构体：从散参数到借用（已完成）

```rust
pub struct ReadRequest {
    pub slave_id: u8,
    pub reg_adr: u16,
    pub reg_length: u16,
}
```

- `pub struct` 只公开类型本身，字段要各自写 `pub`；漏写字段的 pub，模块外能用类型名，却构造不出值。
- 定义类型不等于创建值。创建写 `类型 { 字段: 值, ... }`，字段顺序随意，末尾可有逗号，但字段名必须和定义一致。
- 字段简写 `{ slave_id }` 等价于 `{ slave_id: slave_id }`，前提是外层已有同名同类型的变量；直接写字面值时必须写全。

```rust
let request = frame_tools::ReadRequest {
    slave_id: 0x15,
    reg_adr: 0x0008,
    reg_length: 14,
};
```

与 C 的 `typedef struct { ... } ReadRequest;` 对应：C 同样是"先定义类型，声明变量时才分配内存"。

### 与 C 对照：结构体字面量不套外层花括号

曾写成下面这样，编译不过：

```rust
let request = { .frame_tools::ReadRequest { slave_id: (), reg_adr: () } };
```

- 结构体字面量自带花括号，外面再套一层 `{}` 会被当成代码块；代码块里不能以 `.` 开头。
- 路径前不加点：`frame_tools::ReadRequest`；点号只用于访问字段或调用方法。
- `()` 是单元类型的值，表示"什么都没有"，不是占位符。字段必须给真实值（或已有变量），留空编译不过。
- 想"先留空"没有对应语法；需要默认值时才用 `..Default::default()`，且类型必须实现 Default。

### 借用参数：`&ReadRequest`

```rust
pub fn build_frame(slave_info: &ReadRequest) -> Result<[u8; 8], BuildError> {
    frame[0] = slave_info.slave_id;
    frame[2..4].copy_from_slice(&slave_info.reg_adr.to_be_bytes());
```

调用方和被调函数各有一条规则，容易混：

- 调用方手里是值，所以写 `build_frame(&request)`；调用之后 request 仍可用。
- 被调函数里的参数本身已经是 `&ReadRequest`，往下传时不能再加 `&`，否则是 `&&ReadRequest`，报 E0308。

| 参数写法 | 参数做了什么 | 调用后原变量 |
| --- | --- | --- |
| `slave_info: ReadRequest` | 移动（ReadRequest 未实现 Copy） | 不能再用，用了报 E0382 |
| `slave_info: &ReadRequest` | 借用 | 仍可用 |
| `#[derive(Clone, Copy)]` + 传值 | 复制 | 仍可用，但要求所有字段都是 Copy |

判断标准不是"哪种更高级"，而是"调用者之后还要不要用这个值"。ReadRequest 只含 u8/u16，传值也不慢；但一旦字段里加了 String 或 Vec，传值就变成移动，改签名会连累一串调用点，所以先养成用 `&` 的习惯。

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
- cargo test 运行测试；见到成功和故意制造的断言失败，能确认测试确实在检查结果。正常组帧测试曾因输入/预期不一致失败；首次 Git 提交前已验证正常帧、零数量和最大数量 255 三项测试全部通过。
- 之后补充了数量 1（test_mini_count，精确帧断言）与数量 256（test_upper_max_counter，解构 CountTooLarge(count) 后 assert_eq!(count, 256)）两个用例；当前共 7 个用例。
- 一次失误：写 test_max_count 时把 255 填进了地址字段、数量仍是 14，断言只有 is_ok() 所以照样通过。说明 is_ok() 只能证明"没出错"，不能证明"输入和预期对得上"。
- 单元测试检查函数逻辑，不能代替串口实机收发、超时、断线等系统验证。

格式化暂采用 rustfmt 默认约 100 字符行宽，无需为本次学习另外设置 80 字符限制。

## 22. CRC16：把 C 的位运算翻译成 Rust

```rust
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
```

与 `doc/local_communication.md` 里的 C 版本逐行对应，几处不同：

- `for byte in data` 每轮拿到 `&u8`，要写 `*byte` 解引用；C 里 `p_data[index]` 直接就是元素值。
- `u16::from(*byte)` 是保证无损的转换。`as u16` 也能编译，但 `as` 是"可能截断"的强制转换，`u8 → u16` 用 `from` 更能表达意图。
- Rust 没有三元运算符。C 的 `cond ? a : b` 写成 `if 条件 { a } else { b }`；`if` 在这里是表达式，可以直接给 `crc` 赋值。
- `crc & 1 != 0` 的优先级没问题（Rust 里 `&` 高于 `!=`），加括号只是为了好读。
- `for _ in 0..8` 用 `_` 表示循环变量不用；`0..8` 含 0 不含 8，正好 8 次。
- `u16` 的 `>>` 是逻辑右移（高位补 0），与 C 对无符号数的行为一致。

### 接入组帧时必须分成两句

```rust
let crc = build_crc16(&frame[0..6]);
frame[6..8].copy_from_slice(&crc.to_le_bytes());
```

写成一行 `frame[6..8].copy_from_slice(&build_crc16(&frame[0..6]).to_le_bytes())` 会报 E0502：同一个表达式里，左边要可变借用 frame，右边又要不可变借用 frame。分成两句后，第一句的不可变借用在语句结束时就释放，第二句才能修改内容。

### 字节序：同一帧里并存两种

- 寄存器地址、寄存器数量：高字节在前，用 `to_be_bytes()`。
- CRC：低字节在前，用 `to_le_bytes()`。
- 协议文档帧 `15 03 00 08 00 0E 46 D8` 里，CRC 数值是 `0xD846`，线上两个字节是 `46 D8`。"CRC 是 46 D8"和"CRC 是 0xD846"是同一件事，只是分别从字节和从数值两个角度看。不要把 `46 D8` 当成数值 `0x46D8`。

### 用文档帧做金标准

```rust
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
```

期望值取自协议文档，而不是从代码里抄出来，因此它能独立判断"组帧与 CRC 是否和固件一致"。该用例已跑通。

## 23. 测试变红的两种含义

| 红的含义 | 判别办法 |
| --- | --- |
| 代码写错了 | 没有可信的外部标准，只能逐行检查 |
| 预期值过时了 | 存在一个期望值取自外部标准（协议文档、设备实录）的锚点测试，且这个锚点是绿的 |

两次实测：

- `is_ok()` 造成的假绿：写 `test_max_count` 时把 255 填进了地址字段、数量仍是 14。断言只有 `assert!(result.is_ok())`，所以照样通过——问题出在测试本身，不是被测代码。
- 预期过时：帧尾接入真 CRC 后，`test_build_frame`、`test_mini_count` 变红，`left` 是新帧、`right` 是旧的 `00 00`。此时锚点 `test_doc_frame` 是绿的，所以可以放心按 `left` 更新 `right`。

`assert_eq!` 失败时打印 `left`（实际）与 `right`（预期），并且是**十进制**：`[21, 3, 0, 8, 0, 1, 6, 220]` 就是 `15 03 00 08 00 01 06 DC`。代码里仍写十六进制，便于和协议文档对照。

改测试预期前先问一句：**这个新值是从哪来的？** 如果只能从失败信息里抄，那只是让测试闭嘴；如果能从协议文档或外部标准推出来，才算真的验证。

## 24. 泛型、所有权和 trait 约束

```rust
fn show_value<T: std::fmt::Display>(value: &T) {
    println!("{}", value);
}
```

- T 是类型参数，不是运行时随意改变类型的变量；同一泛型名称在同一次实例化中指相同类型。
- 泛型结构体可以有多个类型参数，例如 Pair<T, U>；Pair<T> 的两个 T 字段必须是同类型。
- 泛型不会取消所有权规则：参数为 T 时，传 String 会移动；传 u8 则复制，因为 u8 实现 Copy。参数为 &T 时借用；data.clone() 则创建独立副本再移入函数。
- String 移入函数后，本例中会随参数离开作用域而销毁，不会自动把所有权还给调用方。与 let b = a 的移动是同一条规则。
- T: Display 声明类型必须支持 {} 打印；T: PartialOrd 声明类型支持相应大小比较操作。约束在编译时检查，不是传入或临时添加一种功能。
- PartialOrd 允许存在无法比较的情况，例如浮点 NaN；当前只用普通数值练习，不把它当作任意类型的全序保证。

## 25. 自定义 trait、默认实现和 dyn

```rust
trait Describe {
    fn name(&self) -> &str;

    fn describe(&self) {
        println!("设备名称：{}", self.name());
    }
}

struct Sensor {
    value: u16,
}

impl Describe for Sensor {
    fn name(&self) -> &str {
        return "Sensor";
    }

    fn describe(&self) {
        println!("sensor data：{}", self.value);
    }
}
```

- trait 规定公共能力接口，impl Describe for Sensor 为具体类型实现接口；这里的 for 不是循环。
- 只有声明的方法需要由实现方提供；有默认函数体的方法可以沿用或重写。
- 类型重写 describe 后，不会先自动执行 trait 的默认 describe。希望打印名字时，需在自己的方法中明确调用 self.name()。
- 默认方法可以调用 trait 中要求实现的其他方法；公共流程与各类型差异可以这样组合。
- 已完成 Sensor、PortConfig、Dev 通过统一入口调用、默认行为和自定义行为的练习。

```rust
fn show<T: Describe>(data: &T) {
    data.describe();
}

fn show_dynamic(data: &dyn Describe) {
    data.describe();
}
```

前者根据具体类型静态分发；后者通过 trait 对象动态分发。当前 Describe 可用于 dyn；不是任意 trait 都能用于 dyn，兼容性规则留待以后。

```rust
let sensor = Sensor { value: 123 };
let device: &dyn Describe = &sensor;
show_dynamic(device);
```

- &dyn Describe 仍是固定类型的引用；不同具体类型必须实现 Describe，才能转换为相应 trait 对象引用。
- 同一个 &dyn Describe 变量可以改为引用另一个兼容对象，但不会改变原对象的类型。
- 可以创建 [&dyn Describe; 3] 保存不同类型对象的引用，统一遍历 describe；已练习。
- 与 C 结构体函数指针用途相似：统一接口、隔离实现；泛型方式不等于在对象里保存函数指针。
- 不必为每个设备定义一个 trait 或一个上层函数，按共同能力组织即可。

## 26. 状态枚举与错误枚举的类型设计

```rust
enum SensorState { Ready, Busy, HardErr }
enum SensorError { Busy, HardwareFault }

trait SensorRead {
    fn read(&mut self) -> Result<u16, SensorError>;
}
```

- 状态用于保存设备当前情况，错误类型用于表达某次操作失败的原因，两者职责不同。
- 若使用 Result<u16, SensorState>，Err(Ready) 也是类型允许的值，即使函数没有返回它，match 也要覆盖。
- Rust 没有强制必须拆两个枚举；拆开后可以在类型层面排除不合理的 Err(Ready)。
- 自定义枚举使用 == 可添加 #[derive(PartialEq)]；用 match 判断这些无数据变体时，无需该约束。
- 构造结构体写 state: SensorState::Ready，冒号前是字段名，不是类型名；随后可用 sensor.state = SensorState::Busy 修改。
- 本段仅为练 trait、Result 与枚举的配合，用户要求暂停向设备更新流程、后台采集架构等话题展开。

## 27. 多个约束、where 和符号对照

```rust
fn is_greater<T>(a: T, b: T) -> bool
where
    T: PartialOrd + std::fmt::Display,
{
    println!("正在比较：{} 和 {}", a, b);
    return a > b;
}
```

- 等价于 fn is_greater<T: PartialOrd + std::fmt::Display>(...)。
- + 表示两种能力都要满足。函数使用 >，需要比较能力；使用 {} 打印参数，需要 Display。只打印固定文字不需要 Display。
- 用户已运行并确认简单比较加打印的例子；长约束偏好 where。
- 先前最大值切片示例把 Copy、Option 等一起引入，跨度偏大；后续每次只加一个概念。

```rust
fn show_pair<T, U>(a: &T, b: &U)
where
    T: std::fmt::Display,
    U: std::fmt::Debug,
{
    println!("a={}", a);
    println!("b={:?}", b);
}
```

| 符号 | 当前位置的含义 | 示例 |
| --- | --- | --- |
| <T, U> | 声明类型参数 | fn show_pair<T, U> |
| (a: &T) | 声明数据参数及类型 | a 借用一个 T |
| : | 变量的类型标注或泛型约束 | a: &T、T: Display |
| :: | 路径分隔 | std::fmt::Debug |
| + | 同时满足多个 trait 约束 | PartialOrd + Display |
| , | 分隔类型参数、数据参数或 where 条件 | T: Display, U: Debug, |

已遇到两次拼写问题：where 后漏写 T:；std:fmt::Debug 中少一个冒号。换行与否不影响这些语法，但每行一条 where 约束更清楚。

泛型结构体方法的后续练习与讨论见第 28–30 节。保持只讲 Rust 概念，不扩展设备架构。

## 28. 泛型结构体的方法和方法约束

```rust
struct Holder<T> {
    value: T,
}

impl<T> Holder<T> {
    fn get(&self) -> &T {
        return &self.value;
    }

    fn set(&mut self, value: T) {
        self.value = value;
    }

    fn show(&self)
    where
        T: std::fmt::Display,
    {
        println!("当前值：{}", self.value);
    }
}
```

- impl<T> 声明本实现中的类型参数；Holder<T> 指被实现方法的类型。它不是 impl Trait for Type，不涉及为类型实现某个 trait。
- Holder<u16> 和 Holder<String> 复用方法逻辑，但各个实例的类型一旦确定，set 就不能把字段换成另一种类型。
- get 只返回借用，不需要 Display；set 接收并替换 T，不需要 Copy；show 在方法自身声明 Display 约束即可。
- 已见到不同类型值的输出，字符串外的引号通常来自 Debug 的 {:?}，Display 的 {} 不额外加引号。

### 泛型字段与固定类型字段可以共存

```rust
struct Holder<T> {
    value: T,
    value2: u16,
}
```

这个定义仍然适用。泛型只作用于写了 T 的字段，value2 始终是 u16。get 返回 &self.value，所以只得到 &T，不会自动返回整个 Holder。要读取 value2，可以写独立方法返回 u16；增加字段后，所有构造代码也必须初始化它。

### 泛型结构体什么时候有意义

Holder 本身主要是语法与所有权的教学例子，不是要求普通业务数据都加一层包装。实际价值在于管理逻辑一致、元素类型不同的容器，例如 Vec<T>、FIFO<T>、消息队列。明确固定的业务字段直接写具体类型即可，不为泛型而泛型。

## 29. Self、self、new 与字段简写

```rust
impl<T> Holder<T> {
    fn new(value: T) -> Self {
        return Self { value };
    }
}

let data: Holder<String> = Holder::new(String::from("AACV"));
```

- 在这个 impl 中，Self 是当前类型 Holder<T>；self 是接收当前具体对象的参数。大小写固定，不能互换。
- -> Self 表示返回当前类型；Self { value } 构造一个当前类型的对象。
- new 没有 self 参数，是关联函数，通过 Holder::new(...) 调用；它是惯例名称，不是必须使用的特殊构造函数关键字。
- Self { value: value } 中左边是字段名，右边是变量名；同名时可简写为 Self { value }。若参数叫 input，则写 Self { value: input }。
- 写类型时用 Self，读取当前对象的字段用 self.value。

## 30. 三种接收者、返回类型与所有权

```rust
impl<T> Holder<T> {
    fn get(&self) -> &T {
        return &self.value;
    }

    fn get_mut(&mut self) -> &mut T {
        return &mut self.value;
    }

    fn into_inner(self) -> T {
        return self.value;
    }
}
```

| 方法 | 接收方式 | 返回的是什么 | 对 Holder<String> 的影响 |
| --- | --- | --- | --- |
| get | &self | 字段的共享引用 &T | 借来读取，原对象保留 |
| get_mut | &mut self | 字段的可变引用 &mut T | 借来修改原值，原对象保留 |
| into_inner | self | 拥有的字段值 T | 移入方法，字段交给调用方，原变量不能再用 |

### 调用方变量类型也要匹配

```rust
let data: Holder<String> = Holder::new(String::from("AACV"));
let text: &String = data.get();
println!("{}", text);
```

T 为 String 时，&T 就是 &String，不是 String。因此不能写 let text: String = data.get()；需要独立副本时才 clone。

```rust
let data: Holder<String> = Holder::new(String::from("AACV"));
let text: String = data.into_inner();
// data 此后不能再使用，text 拥有取出的字符串。
```

方法名不决定移动还是借用，签名才决定。若把 into_inner 改成 (&self) -> &T，行为就变成借用，虽然名字不再符合惯例。采用 get 表达只读借用更清楚。

### 为什么不能从被消耗的对象中返回字段引用

```rust
// 错误示例：不能这样返回。
// fn into_inner(self) -> &T {
//     return &self.value;
// }
```

这个 Holder 由方法拥有，在结束时销毁；不能把指向它内部字段的引用返回给外面。改为借用对象再返回 &T，或消耗对象并返回 T。这里针对当前拥有 T 字段的 Holder，不泛化到所有带引用字段的类型。

### get_mut 明确授予修改原值的权限

```rust
let mut data: Holder<String> = Holder::new(String::from("AACV"));
let text: &mut String = data.get_mut();
text.push_str("_Rust");
println!("{}", data.get());
```

没有复制字符串，修改的是原字段。允许修改来自可变对象和 &mut 引用，借用结束后可以重新访问 data。Rust 检查内存与借用安全，不会自动保证修改符合业务规则；不想开放直接修改，就不提供 get_mut，或采用带检查的修改方法。

所有权判断顺序：先看接收者是 &self、&mut self 还是 self，判断借用或按值接收；再看返回值是 &T、&mut T 还是 T。对于 Copy 类型按值接收可复制，不能将所有 self 参数一概当作使原变量失效。

## 31. 闭包入门（本轮先到这里）

已手写并观察闭包捕获外部 offset；也运行过计数闭包，输出当前计数 1、2，最终计数 2。

- `|参数| { 函数体 }` 定义闭包，无参数写 `||`；保存到变量后可以调用，也可作为函数参数传入。
- 普通函数不能直接捕获调用处的局部变量；闭包可以按使用方式借用或取得外部值的所有权。
- 只读 offset 的示例使用共享借用；修改 count 的示例使用可变借用，仍受借用规则限制。
- 已讨论 retain 接受判断闭包的用途，未收到这项手写验证反馈。
- 学习者决定暂时掌握写法、调用与捕获即可；move、Fn/FnMut/FnOnce 留待需要时展开。

## 32. minigrep：文件、字符串与返回容器

更新至 2026-11-11。当前模块入口为 minigrep_start()，项目结构、运行方法和当前测试见 [学习进度.md](学习进度.md)。

- env::args().collect() 获取 Vec<String>；下标 0 是程序自身，1、2 是关键词与路径。
- Config 拥有 query 和 file_path 两个 String；parse_args 接收 &[String]，通过 clone 创建配置，返回 Result<Config, String>。
- fs::read_to_string 返回 Result<String, std::io::Error>；lines() 逐行借用为 &str，contains 返回 bool。
- to_lowercase 创建小写副本用于比较，返回结果仍保留原始文本。
- 第一版 search 返回 Vec<String>，每行用 to_string 复制。return result 把容器所有权交给调用者，局部变量作用域结束不释放已经移出的数据。
- 当前版返回 Vec<&str>，只保存原文引用；不复制匹配行，仍会为 Vec 容器分配存储；忽略大小写比较也仍会创建小写 String。
- Vec 通常不能用 {} 直接打印；调试可用 {:?}，正常结果逐行打印。
- 无匹配返回空容器；这不等于文件读取失败。
- 当前 CASE_SENSITIVE 环境变量能读到时启用区分大小写；值是 0 也同样启用。早期第四参数 --case-sensitive 方案已取消。
- 2026-11-11 运行当前七项测试全部通过，早期无匹配测试正文和关键词重合的问题已修复。

## 33. Result、match 表达式与分支类型

Result<Config, String> 只表示一个结果值：成功为 Ok(Config)，失败为 Err(String)，不是同时返回两个参数。

```rust
let config: Config = match parse_args(&args) {
    Ok(config) => config,
    Err(error) => {
        eprintln!("{}", error);
        return;
    }
};
```

- 左边需要 Config；成功分支取出 Config，作为整个 match 的结果。
- 失败分支 return 退出当前函数，不再执行赋值；如果不退出而要继续，就必须提供一个备用 Config。
- 没有 return 且只有打印的分支结果为 ()，无法与 Config 一致。
- 可以直接 match 函数调用，不强制声明 parse_result 中间变量。
- 按值 match 绑定 Config 或 String 会移动内部数据，之后不能随意再使用原 Result。
- 只问成功或失败时用 is_ok/is_err，不需要先拆出内部值。仅检查状态的方法借用结果。
- 同理，if/else 可以产生值；分支末尾加分号会丢弃调用结果，变成 ()。
- ref config 得到 &Config，不能赋给要求 Config 的变量；不要为消除报错随意加 ref。

测试里预期成功却得到 Err 时，用 panic! 让测试明确失败。只 eprintln 再 return 会正常结束测试，可能产生假通过。tests 子模块需要通过 super 引入 Config 和被测函数。

## 34. 错误传播、标准错误与退出码

```rust
fn search_file(config: &Config) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(&config.file_path)?;
    // 搜索与打印……
    Ok(())
}
```

- ? 遇到成功就取出 String 继续；遇到错误就提前返回当前函数的 Err，不自动打印。
- Result<(), E> 成功时只表达操作完成，不返回业务数据。
- 调用者可用 if let Err(error) = search_file(&config) 只处理失败。
- std::io::Error 经 {} 格式化产生系统错误说明；read_to_string 本身不打印。
- error_log 是普通 String，不会自动关联错误。文件不存在却手写“参数非法”，提示就会不对应。
- println 写 stdout，eprintln 写 stderr；用 > 保存结果时二者可分开，当前主流程已去掉配置调试输出。
- std::process::exit(1) 直接结束进程并返回非零退出码。return 只退出当前函数。
- PowerShell 的 $LASTEXITCODE 应在目标命令后立即查看；重新运行成功命令后为 0 是预期行为。
- 用户已理解基本用法，不继续在输出重定向上反复练习。

## 35. str、String 与返回借用的生命周期

- String 拥有可变长 UTF-8 文本并管理存储；&str 借用一段文本，引用携带地址和字节长度。
- str 自身是动态大小类型，不能像 u16 一样直接按值定义普通局部变量，通常通过引用使用。
- String 用于持有数据；只查看或搜索文本时 &str 很常用。
- 借用不会延长原数据的存活时间；外层引用不能在其借用的内层 String 释放后再使用。

```rust
fn get_text(text: &str) -> &str {
    text
}
```

只有一个输入引用时，编译器可按省略规则推断返回引用与它关联，可以省略显式标注。

```rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
```

- <'a> 声明生命周期参数，contents 与返回元素共享该标注，约束结果的使用范围。
- 标注描述借用关系，不创建数据、不延长寿命；也不能只理解成运行时来源标签。
- 多个引用输入时，需要指明返回引用的关系；当前结果借自 contents，不借自 query。
- 在函数中直接 result.push(line)，不要 line.to_string()；后者创建 String。
- 调用处写 Vec<&str> 即可；函数中声明的 'a 名字不会自动出现在调用者作用域。
- 输入已经是 &str 时直接传；拥有 String 时借用为 &String，可自动转换为 &str。
- 忽略大小写函数也返回原始 line，不返回临时 lower_line 的引用。
- 当前两个搜索函数均返回 Vec<&str>，因此 search_file 中 if/else 类型一致。

已通过 test_search_query_scope：contents 在外层，query 在内层创建，离开内层后 query 释放，外层结果仍可使用。它验证返回行依赖原文，与关键词的存活时间无关。

## 36. 行号搜索与元组容器

```rust
fn search_with_line_number<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<(usize, &'a str)>
```

- enumerate 给出 (number, line)，number 是从 0 开始的下标，line 是原文的一行 &str。
- 返回每个元素是 (usize, &str) 元组，容器声明为 Vec<(usize, &'a str)>。
- push((number + 1, line)) 只有一个参数：整个元组；push(number, line) 是两个参数，不符合 push 签名。
- for (number, line) in &ret 可拆开结果打印。
- 测试匹配第一、第三行，预期为 vec![(1, 原始第一行), (3, 原始第三行)]，已通过。
- 本函数固定忽略大小写，仅用于练习返回元组、容器、原文引用；主流程没有接入，不继续为它扩展模式管理。
- #[allow(dead_code)] 局部关闭未使用警告，#[un] 不是有效写法。
- 两个同名 let 会遮蔽：先调用一次搜索丢弃结果，再新建 ret 调用另一函数没有必要。

## 37. 下一步：迭代器 next（尚未验证）

```rust
let numbers: [u16; 2] = [10, 20];
let mut iterator = numbers.iter();
println!("{:?}", iterator.next());
println!("{:?}", iterator.next());
println!("{:?}", iterator.next());
```

预期 Some(10)、Some(20)、None。这里 next 返回 Option<&u16>，迭代器 mut 用于更新内部位置，数组本身未修改。先观察 for 反复取元素的关系，再逐步讲 map/filter/collect；示例已给，用户要求先记笔记，尚未收到运行反馈。

学习方法：库方法用到再查；关注接收类型、返回类型和对原数据的影响。用户目前觉得所有权、借用尚可判断，主要卡在多种语法表达同一目的和库方法不熟，后续不堆新语法。
