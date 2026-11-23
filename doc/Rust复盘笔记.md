# Rust 复盘笔记

更新日期：2026-11-23

这里只保留知识速查和容易混淆的写法。当前掌握程度见 [学习进度](学习进度.md)，接下来的任务见 [学习路线与复写计划](Rust学习路线.md)。

## 1. 变量、类型与返回值

```rust
let mut count: u8 = 0;
let address = 0x1234u16;
let valid: bool = count == 0;

fn add_one(value: u16) -> u16 {
    return value + 1;
}
```

- 变量和参数都是“名字: 类型”；mut 允许修改值，不允许改变类型。
- 同名 let 可遮蔽之前的变量；这与给原变量赋值不同。
- u8/u16/u32 是无符号整数，i16 等是有符号整数；usize 用于长度和下标，类似 C 的 size_t。
- 已标注 u8 时，初值写 0 即可；0u8 也可直接标明字面值类型，没有 C 的 0U 后缀。
- if 条件必须为 bool；不能用整数代替。花括号必须写，条件不要求圆括号。
- return value; 显式返回；函数末尾也可写不带分号的 value。末尾加分号会丢弃该表达式的值，块结果变为 ()。
- u16::from(byte) 表达无损转换；as 转换可能截断，缩小整数前先检查范围。
- 字面值超范围的编译错误，不等于运行期间整数运算溢出的规则。

## 2. 所有权、借用与字符串

| 写法 | 对非 Copy 值的影响 |
| --- | --- |
| let b = a; | 移动所有权，原绑定不能继续使用 |
| let b = a.clone(); | 创建独立副本，原值保留 |
| let b = &a; | 共享借用，用于读取 |
| let b = &mut a; | 可变借用，用于修改原值 |

String 和 Vec 不实现 Copy；u8 等整数实现 Copy，赋值或按值传参时复制。传入函数与赋值遵循同一套所有权规则。

```rust
fn set_value(value: &mut u8) {
    *value = 2;
}

let mut data: u8 = 4;
set_value(&mut data);
```

- let mut data、调用处 &mut data、参数 &mut u8 分别表达可修改变量和可变借用。
- let mut reference = &data 只允许重新赋值这个引用，不允许通过它修改 data。
- 对同一份数据，共享访问可共存；可变借用有效期间要满足独占访问规则。
- 借用通常可在最后一次使用后结束，不必等整个代码块结束；借用后面仍要使用时，不能先从原变量冲突访问。
- 方法可以自动解引用；修改一个数字引用通常写 *value = ...。
- C 的复制指针只复制地址，不等于 clone 字符串；Rust 引用也不是裸指针。

| 类型 | 用途 |
| --- | --- |
| String | 拥有并管理可增长文本 |
| &String | 借用一个 String |
| str | 动态大小的字符串切片类型 |
| &str | 借用文本，携带地址和字节长度 |

存储拥有的文本常用 String，只读文本参数通常用 &str。字符串字面值是 &str，&String 在需要时可转换为 &str。str 不能直接作为普通按值参数或普通局部变量类型。

## 3. 数组、切片与 Vec

```rust
let array: [u8; 2] = [0x12, 0x34];
let zeros: [u8; 10] = [0; 10];
let mut buffer: Vec<u8> = vec![0x12, 0x34];
let data: &[u8] = &buffer[0..2];
```

- [u8; N] 是固定长度数组，长度属于类型；[u8] 是动态大小类型，参数通常借为 &[u8]。
- &[u8] 用途类似 C 的“数据指针 + 长度”，不复制数据；&mut [u8] 可以修改原数据。
- a..b 包含 a、不包含 b；a..=b 包含 b；切片自己的下标从 0 开始。
- 长度为 5 时，3..5 合法，3..=5 越界；2..2 是空切片。
- len() 是元素个数；capacity() 是容器当前可容纳的元素数；size_of_val() 是值的字节大小。
- 直接索引、切片可能越界 panic；访问前检查长度，或使用 get()。

| 方法或写法 | 作用 |
| --- | --- |
| Vec::new() | 创建空 Vec |
| vec![0, 10] / vec![0; 10] | 两个元素 / 十个零 |
| push(value) | 追加一个元素 |
| extend_from_slice(data) | 复制切片元素，追加到末尾 |
| clear() | 清空元素，保留容量 |
| drain(0..length) | 移除指定范围；本轮用于消费一帧 |
| fill(value) | 覆盖已有元素，不改变长度 |
| copy_from_slice(data) | 覆盖目标切片，来源和目标必须等长 |

对 Vec<u8>，new() 本身不申请元素的堆存储；追加时按容量需要分配或扩容。返回 Vec 会转移所有权，最终所有者销毁时释放内存。传 &mut Vec 可以复用外部缓冲区。

Vec<T> 也可保存自定义结构体，但同一容器的元素类型一致。extend_from_slice 要求元素可 Clone；push 接收并放入一个元素。

## 4. 遍历、元组与 Option

本轮重点：先判断要下标、一个元素还是一段数据，再写类型。

| 表达式（buffer 为字节数组或 Vec<u8>） | 类型或每轮元素类型 |
| --- | --- |
| buffer.len() | usize，元素数量，不是可遍历范围 |
| buffer[0] | u8，一个值 |
| &buffer[0] | &u8，一个元素的引用 |
| &buffer[0..2] | &[u8]，一段数据的引用 |
| buffer.iter() | 每轮 &u8 |
| buffer.iter_mut() | 每轮 &mut u8 |
| buffer.iter().enumerate() | 每轮 (usize, &u8) |

```rust
// 下标遍历：index 是 usize，buffer[index] 是 u8。
for index in 0..buffer.len() {
    println!("{:02X}", buffer[index]);
}

// 元素遍历：byte 是 &u8，*byte 是 u8。
for byte in buffer.iter() {
    println!("{:02X}", *byte);
}
```

```rust
// buffer 需要可变；byte 是 &mut u8。
for byte in buffer.iter_mut() {
    *byte = 0x55;
}

// 同时需要下标和元素时使用 enumerate()。
for (index, byte) in buffer.iter().enumerate() {
    println!("index={} byte={:02X}", index, byte);
}
```

- iter() 每轮提供共享引用，iter_mut() 提供可变引用；into_iter() 按值迭代容器，非 Copy 值可能被移动。
- enumerate() 加入从 0 开始的计数，产生 (下标, 元素) 元组；for 的括号负责解构。
- byte 只引用一个元素，不能写 byte[index]。
- *byte + 1 只计算并丢弃结果；修改原元素要写 *byte += 1 或 *byte = *byte + 1。
- 打印 &u8 时格式化支持引用，可直接传 byte；数值运算或赋值时按需要写 *byte。
- 只用下标时可写 for index in 0..buffer.len()，不能直接 for index in buffer.len()。
- (10..13) 的括号用于组合范围表达式；(10, 13) 才是两个元素的元组。
- push((number, line)) 添加一个元组，push(number, line) 则是传入两个参数。

```rust
if let Some(byte) = buffer.get_mut(1) {
    *byte = 0xAA;
}
```

get/get_mut 返回 Option<&T>/Option<&mut T>：有元素为 Some，无元素为 None。Some/None 是枚举变体，0 也是合法元素，不等于 None。if let 适合只关心一种分支，match 适合处理全部分支。

- first()/last() 返回 Option<&T>，安全取首项或末项；空 Vec 也是合法容器，len() 为 0。
- measurements.last() 的 Some 中是 &Measurement，不是 Vec；整个容器的条数用 measurements.len()。
- 收到半帧也要累计字节数，因此字节及测量条数统计放在 if let 外：recoder.recoder(count, measurements.len())。
- 在循环外保存 Option<Measurement> 可表示“尚无最近测量值”；只需显示本批最后一条时，直接 last() 即可。

## 5. Result、match、? 与 expect

Result<T, E> 只有一个结果值：Ok(T) 或 Err(E)，不是同时返回两项。Result<Vec<u8>, String> 与里面的 Vec<u8> 是不同类型。

```rust
let frame: Vec<u8> = match pack_frame(&data) {
    Ok(frame) => frame,
    Err(error) => {
        println!("组包失败：{}", error);
        return;
    }
};
```

- match 可以产生值，Ok 分支末尾的 frame 交给外面的变量。
- 分支内部的绑定有作用域限制；不能在分支外直接使用它。
- 继续执行的分支需要产生匹配的结果类型；return/panic! 的分支不继续执行赋值。
- Err(...) 只是构造错误值，return Err(...) 才是提前退出函数。
- Result<(), E> 成功不返回业务数据，用 Ok(())；() 是单元类型，不是数字零。
- is_ok()/is_err() 只检查状态；需要值或具体错误时用 match。
- ? 成功取出内部值，失败从当前函数传播错误，不自动打印；函数返回类型必须支持相应传播。
- expect("提示") 成功取值，失败 panic；Option 的 expect 在 None 时 panic。它不负责重试或恢复。
- 错误枚举优先只列失败原因；状态 Ready 与错误 Busy 不必塞在同一枚举里。

## 6. struct、enum、impl 与 Self

```rust
struct FrameBuffer {
    buffer: Vec<u8>,
}

impl FrameBuffer {
    fn new() -> Self {
        return Self { buffer: Vec::new() };
    }

    fn receive(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }
}

let mut receiver = FrameBuffer::new();
receiver.receive(&[0xAA, 0x55]);
```

- Self 是当前类型，self 是当前实例。
- new() 没有 self，是关联函数；new 只是惯用名称。真正创建实例的是 Self { 字段: 值 }。
- 解析函数也可直接返回 Ok(Self { ... })，不需要先调用 new。
- 类型名::new 是函数本身，类型名::new() 才调用函数。
- &self 读取，&mut self 修改，self 按值接收。是否移动、是否借用取决于签名，不取决于方法名称。
- FrameBuffer 没有自动继承 Vec 的方法：receiver.receive() 调自己的方法，self.buffer.extend_from_slice() 调内部字段的方法。
- 字段简写 Self { value } 等同 Self { value: value }；Point { x: 5.2, ..point } 从原值补齐其他字段，非 Copy 字段可能移动。
- enum 的各个变体仍属于同一枚举类型，可携带 String 等数据。match &state 借用查看；按值匹配并绑定 String 可能移走内部数据。
- #[derive(Debug)] 提供 {:?}，不提供 {}；#[derive(PartialEq)] 支持相等比较。
- 独立函数、impl 中直接处理、方法调用独立函数都是组织选择，不为使用语法强行加层。

## 7. 泛型、trait 与 where（按需回看）

```rust
fn is_greater<T>(a: T, b: T) -> bool
where
    T: PartialOrd + std::fmt::Display,
{
    println!("{} 和 {}", a, b);
    return a > b;
}
```

- T 是类型参数，同一次实例化中的同名 T 表示同一类型；多个类型用 T、U 等分别声明。
- T: Trait 是能力约束；+ 表示同时满足，where 是另一种写法，不取消所有权规则。
- 路径用 ::，约束用 T:；std::fmt::Debug 不能写成 std:fmt::Debug。
- struct Holder<T> 可以同时含 value: T 和其他固定类型字段；impl<T> Holder<T> 为该类型定义方法。

| Holder 方法签名 | 作用 |
| --- | --- |
| get(&self) -> &T | 借用字段 |
| get_mut(&mut self) -> &mut T | 借用并允许修改原字段 |
| into_inner(self) -> T | 取走字段所有权 |

T 为 String 时，get() 得到 &String，不能直接赋给 String。不能消耗一个拥有字段的局部对象，再返回指向已销毁字段的引用。

```rust
trait Describe {
    fn describe(&self) {
        println!("设备");
    }
}

struct Sensor;
impl Describe for Sensor {}
```

trait 声明公共能力，impl Trait for Type 为类型提供实现。未提供默认函数体的方法必须实现；有默认实现的可重写，重写后不会自动先执行默认版本。

fn show<T: Describe>(data: &T) 是泛型静态分发；fn show(data: &dyn Describe) 使用动态分发。用途可类比 C 中通过统一接口隔离实现，但不等于每个对象都保存函数指针。

## 8. 返回引用与生命周期

```rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
```

- 返回元素借自 contents，不借自 query；原文销毁后不能继续使用这些引用。
- 'a 描述借用关系，不分配数据，不延长原数据寿命，也不只是运行时的来源标签。
- 只有一个输入引用时，常可省略显式标注，例如 fn get_text(text: &str) -> &str。
- 返回原文的 line，而不是临时 line.to_lowercase() 产生的 String 的引用。
- 返回 Vec<&str> 不复制行内容，但 Vec 自己仍管理存储；Vec<String> 则拥有各行文本。
- 调用处一般写 Vec<&str>，函数声明中的 'a 名称不会自动进入调用方作用域。

## 9. 模块、文件与可见性

```rust
// src/exercises.rs
pub mod serial_io;
pub mod unpack;

// src/exercises/serial_io.rs
use super::unpack::FrameBuffer;
```

- mod 声明模块；use 把名字引入当前作用域，不执行函数、不复制代码。
- crate 指当前 crate 的根，super 指父模块，不是测试专用语法。
- pub struct 公开类型，方法和字段分别控制可见性；外部通过公开方法操作时，字段可保持私有。
- 当前兄弟练习模块共享的类型、函数和方法使用 pub(super)；返回类型的可见范围也要与函数接口匹配。暂不深入其他可见性写法。
- 可以保留模块前缀调用，不强制 use 每个函数。
- 某个练习暂时不用，可对函数或模块加 #[allow(dead_code)]；放在 main 函数上不会覆盖其他模块。

## 10. 大小端、字节流与本轮协议

```text
AA 55 | payload长度 u8 | payload | CRC低字节 CRC高字节
```

- 这是教学协议，不同于 [原 MCU 通信协议](local_communication.md)。
- 数据长度 N 为 0～255；CRC 起点 3 + N，整帧 N + 5。
- CRC16 使用初值 0xFFFF、右移、多项式 0xA001；计算帧头、长度和数据，不含 CRC 自身。
- u16/i16 的 to_be_bytes()/from_be_bytes() 高字节在前；le 对应低字节在前。
- to_be_bytes() 返回 [u8; 2]，不会自动展开到 u8 数组中，追加可用 extend_from_slice()。
- 先算 CRC，再写入帧尾，可避免在一个表达式里同时冲突借用 frame。
- 测量 payload 为电压 u16 大端 + 电流 i16 大端；当前约定单位 mV/mA。

```text
AA 55 04 09 C4 FF 9C EF 37
         电压2500 电流-100
```

接收字节先追加缓冲区：半帧保留，完整帧校验后处理并移除，继续处理剩余数据；帧头或 CRC 错误时丢一个字节重新寻找帧头。

这叫重新同步，不是修复错误字节。当前还没有缓存上限和假帧头超时处理。

### 当前 A8 独立复写协议

文件为 src/exercises/protocol_recover.rs，与上面的 AA55 + CRC 练习不同。

```text
A8 | payload长度 u8 | payload N字节 | 校验 u8
0       1              2开始         N+2
```

- 整帧 N + 3；空 payload 允许，组包长度超过 255 则返回错误。
- 对帧头、长度、payload 使用 u8 的 wrapping_add 累加，最终 (!sum).wrapping_add(1)。校验不包含自身及后续帧。
- 样例：数据 [01, 02, 03]，完整帧 [A8, 03, 01, 02, 03, 4F]。
- 解包顺序：最小长度检查 → 帧头 → 整帧长度 → 校验 → 返回 payload 借用。
- 组包返回 Result<Vec<u8>, ProtocolError>；解包返回 Result<&[u8], ProtocolError>；处理函数接收 &mut Vec<u8>。
- 当前处理函数返回 Vec<Measurement>；没有完整有效测量数据时，返回的 Vec 可以为空。Measurement 的 payload 是 4 字节：电压 u16 大端、电流 i16 大端。
- 实测样例：[A8, 04, 09, C4, FF, 9C, EC]，表示 2500 mV、-100 mA。
- 数据未齐应保留并退出等待；成功时先使用 payload、计算长度，再 drain(0..frame_length)，之后不继续使用该借用。
- 坏帧头或坏校验后丢一个字节再循环；数据不完整则退出等待。前者寻找下一帧，后者保留半帧。
- 此策略没有假帧头超时机制；遇到看似合法但长度很大的头部，仍可能等待后续数据，本阶段不扩展。

## 11. 串口库速查

| 调用 | 作用或返回结果 |
| --- | --- |
| serialport::available_ports() | Result，成功得到串口列表，列表可以为空 |
| serialport::new("COM14", 9600) | 创建串口配置 |
| .timeout(Duration::from_millis(100)) | 配置超时，不是读取周期 |
| .open() | 真正打开串口，返回 Result |
| port.write(&data) | Result，成功返回本次写入数量，可能只写部分 |
| port.write_all(&data) | 尝试全部写完或报错，成功为 Ok(()) |
| port.read(&mut buffer) | Result，成功返回实际读取数量 count |

```rust
match port.read(serial_buf.as_mut_slice()) {
    Ok(count) => {
        receiver.receive(&serial_buf[..count]);
        receiver.process_pack();
    }
    Err(error) => {
        if error.kind() != std::io::ErrorKind::TimedOut {
            println!("读取失败：{}", error);
            break;
        }
    }
}
```

此片段放在接收循环内。只传 [..count]，不能把整个接收数组当作本次收到的数据。一次 read 不是一帧，有数据通知也不保证收齐一帧。

串口对象销毁时关闭句柄；_port 仍持有对象，下划线前缀不使它提前释放。终端练习目前用 Ctrl+C 退出。serialport 为同步库，tokio-serial 用于异步；现在先不换库。

- Ok 分支中的局部 Vec 在作用域结束时释放，逐次创建不会自动累积泄漏；循环外的接收 Vec 则跨 read 保留半帧。
- ReceiveStats 保存累计字节数和有效测量条数；read 次数、测量条数、字节数是不同统计口径。
- Instant::now() 记录计时起点，elapsed() 返回 Duration；报告后更新起点，不清零计数时显示的是累计统计。
- print! 不换行可能暂未显示输出，调试接收流程时用 println!，避免误认为代码未执行。

## 12. 闭包、线程与通道（刚入门）

```rust
let (tx, rx) = std::sync::mpsc::channel::<u16>();
let handle = std::thread::spawn(move || {
    tx.send(2500).expect("发送失败");
});
match rx.recv() {
    Ok(value) => println!("{}", value),
    Err(error) => println!("{}", error),
}
handle.join().expect("后台线程发生了panic");
```

- |参数| { ... } 是闭包，|| 是无参数闭包；闭包可捕获外部变量，普通函数不能直接捕获局部环境。
- spawn 创建线程；spawn(serial_run) 传函数，serial_run() 是在当前线程立即调用。
- move 按值捕获闭包实际使用的外部变量；这里 tx 移入闭包，rx 留在主线程。Copy 值会复制，move 不等于 clone，也不会延长所捕获引用指向数据的寿命。
- tx.send() 发送线程消息，rx.recv() 等待线程消息，与串口 read 不同。
- join() 等待线程结束，不负责停止线程；无限串口循环不退出，join 就一直等待。
- 将函数收到的 &str 传给 spawn 前，先 to_string()，再用 move 把拥有的 String 交给闭包；move 一个引用不会复制其指向的文本。立即 join 也不会取消 spawn 对捕获数据的 'static 要求。
- recv 也会阻塞；这个例子还不等于完成界面的非阻塞更新。

## 13. 测试与排错

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
```

- assert_eq! 比较实际值和预期值，失败时显示 left/right；assert! 检查 bool。
- 正常输入遇到 Err 应 panic；非法输入应得到指定错误。只打印不检查，会产生假通过。
- 可以显式写全 Ok、指定 Err、其他 Err 分支；关键是只有预期结果正常结束，其他结果都 panic。
- 处理函数消费整帧后，接收 buffer 应为空，不能再要求它等于 payload；payload 内容应在解包成功时比较。
- 验证同一次调用内恢复：坏帧和好帧一起追加到同一缓冲区，再调用一次处理函数；不能用新 Vec 覆盖残余。
- 边界输入要真正填在待测参数上；只检查 is_ok() 不证明内容正确。
- 测试预期应来自协议或独立已知样例，不要只从失败输出复制；协议增加 CRC 后，样本、长度和数据范围需要同步。
- 缓冲区为空只证明字节已消费，不证明业务结果已交付。

```powershell
cargo check
cargo test
cargo test exercises::unpack
cargo test test_frame_error_and_right -- --nocapture
```

默认隐藏通过测试的打印，--nocapture 显示日志。筛选测试不需要注释其他测试。

| 报错或现象 | 先检查 |
| --- | --- |
| E0308 mismatched types | expected/found，整个 Result 与内部值是否混用 |
| E0382 moved value | String/Vec 是否已被按值传出或匹配取走 |
| 借用冲突 | 可变引用后面是否仍会使用，同时又访问了原数据 |
| copy_from_slice panic | 来源与目标切片是否等长 |
| 调不出方法 | 点号左边实际是什么类型，是否忘了调用 new() |
| 解析失败 not data | 已拆出的 payload 是否满足业务字段长度 |

## 14. minigrep 与文件 I/O 速查

- env::args().collect() 获取 Vec<String>，下标 0 是程序自身；parse_args 返回拥有关键词和路径的 Config。
- fs::read_to_string() 返回 Result<String, std::io::Error>；? 可向调用者传播读取错误。
- contents.lines() 借用各行，contains(query) 匹配；to_lowercase() 创建小写副本，用于比较时仍返回原文行。
- 当前 search 返回 Vec<&str>；带行号练习返回 Vec<(usize, &str)>，enumerate 的下标加 1 表示人读的行号。
- 当前 CASE_SENSITIVE 用 env::var(...).is_ok() 判断变量是否可读，不解析其值；值为 0 也启用。
- eprintln! 写标准错误，println! 写标准输出；process::exit(1) 表示失败退出，$LASTEXITCODE 需紧接对应命令检查。
- Cargo 的 -- 把后续参数传给程序。运行 minigrep 前先切换 main 的入口，不能在串口或线程入口下测试其命令行参数。

依赖下载若再次出现 CRYPT_E_REVOCATION_OFFLINE，历史曾用单次命令配置 http.check-revoke=false 绕过吊销服务器不可达；这是临时降低该检查的排障办法，不作为默认项目设置。

## 15. 串口命令行参数复写

```rust
fn parse_args(args: &[String]) -> Result<Config, String>
```

- args.get(1) 返回 Option<&String>，判断有没有串口名；args.get(2) 取得波特率文本。
- text.parse::<u32>() 返回 Result<u32, ParseIntError>，判断能否转成数字；类型参数写在调用括号前。
- Config 用 String 保存串口名，通过 to_string()/clone() 获取自己拥有的文本；参数列表本身只需借用。
- &Vec<String> 可以工作；只读取列表时，&[String] 同时适用于 Vec 和数组。
- 失败提前 return Err(...)，成功构造 Ok(Config { ... })；main 匹配结果并启动串口，无需额外 run 标志或占位初值。
- format!("波特率格式错误：{}", error) 生成 String；println!/eprintln! 打印。解析函数返回原因、main 统一打印可避免重复输出。
- 参数数量检查和格式转换通过，不代表端口一定存在或波特率一定受支持；打开串口仍可能失败。
- 当前入口：cargo run -- COM14 9600。下标 0 是程序名，两个用户参数时 args.len() 为 3。

## 16. CSV 追加记录和 `?`

- `OpenOptions::new().create(true).append(true).open(path)` 打开或创建文件，写入时追加。两个配置方法返回 `&mut Self`，所以可以继续接方法。
- `file.metadata()` 返回 `io::Result<Metadata>`。先用 `match` 或 `?` 取得 `Metadata`，再用 `metadata.len()` 得到文件字节数。要显示大小，用 `println!("size={}", metadata.len())`。
- 文件大小为 0 才写表头。表头 `voltage_mv,current_ma` 和数据行 `2500,-100` 都由逗号分成两列。
- `writeln!(&mut file, ...)` 写一行并返回 `io::Result<()>`。操作 `File` 时在模块中引入 `use std::io::Write;`。
- `io::Result<File>` 是 `Result<File, io::Error>` 的简写。调用后用 `Ok(file) => file` 取出 `File`；出错分支可先打印再提前返回。
- `?` 遇到 `Ok` 取值继续，遇到 `Err` 从当前函数提前返回。成功类型可以各不相同，错误需要能转成外层函数的错误类型。打开取得 `File`，查询取得 `Metadata`，写入取得 `()`，最后用 `Ok(file)` 返回。
- `for measurement in &measurements` 借用遍历本批所有结果，每条写一行，循环后仍可使用 `measurements.len()`。一次串口读取可能含多帧。
