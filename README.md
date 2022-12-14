# rs-lua
使用 Rust 实现 Lua 解释器


## 使用
使用
```bash
git clone https://github.com/web3js-cn/rs-lua.git
cd rs-lua
cargo run
```
文档查看
```bash
cargo doc --open
```

## 进度纵观
+ 2022-12-12 12.16 完成了从 chunk 中读取头部并进行验证(header.rs)
+ 2022-12-13 20.06 读取 chunk 到 Chunk 结构体, 包含了头部、主函数 upvalues、主函数原型(chunk.rs)
+ 2022-12-14 18.17 完成指令集的读取与解释, 从中提取出操作码等信息(opcodes.rs)
+ 2022-12-14 23.04 实现了一个简易的 LuaState 和 LuaStack(lua_state.rs lua_stack.rs)
+ 2022-12-15 12.42 实现了 25 个运算符 (math/)
+ 2022-12-15 19.07 实现简易 VM, 但是有不少 bug, 需要进一步修改 (vm/)
+ 2022-12-16 18.08 代码纠正, 成功计算 1+2+3+...+100 (*.rs)
+ 2022-12-18 15.49 实现了 Lua 表; 用表改写 1+2+3+...+100, 正确执行(table/)

## 进度
+ 2022-12-12 12.16 完成了从 chunk 中读取头部并进行验证(header.rs)
```rs
/// 头部结构体
#[derive(Debug)]
pub struct Header {
    /// 签名 魔数
    signature: Vec<u8>,
    /// 版本号
    version: u8,
    /// 格式号
    format: u8,
    /// 6 字节, 0x1993发布年份+回车+换行+替换+换行, 起进一步校验, 若与预期不一样说明文件损坏
    luac_data: Vec<u8>,
    /// cint 在 chunk 中占有字节数
    cint: u8,
    /// size_t 在 chunk 中占有字节数
    size_t: u8,
    /// 虚拟机指令在 chunk 中占有字节数
    vm: u8,
    /// 整数在 chunk 中占有字节数
    integer_size: u8,
    /// 浮点数在 chunk 中占有字节数
    number_size: u8,
    /// 存放0x5678用来检查chunk的大小端
    int: Vec<u8>,
    /// 存放浮点数370.5, 检查chunk浮点数格式
    num: Vec<u8>
}
```
+ 2022-12-13 20.06 读取 chunk 到 Chunk 结构体, 包含了头部、主函数 upvalues、主函数原型(chunk.rs)
```rs
/// chunk 结构
#[derive(Debug)]
pub struct Chunk {
    /// 头部
    header: header::Header,
    /// 主函数 upvalues 数量
    sizeUpvalues: u8,
    /// 主函数原型
    mainFunc: ProtoType
}

/// 函数原型
#[derive(Debug)]
struct ProtoType {
    /// 由哪个文件编译而来
    source: String,
    /// 函数开始行号
    line_start: u32,
    /// 函数结束行号
    line_end: u32,
    /// 固定参数个数
    nums_params: u8,
    /// 是否 vararg
    is_vararg: u8,
    /// 寄存器数量
    max_stack_size: u8,
    /// 指令表
    code: Vec<u32>,
    /// 常量表
    constants: Vec<Constant>,
    /// Upvalue 表
    upvalues: Vec<Upvalue>,
    /// 子函数原型表
    protos: Vec<ProtoType>,
    /// 行号表
    line_info: Vec<u32>,
    /// 局部变量表
    loc_vars: Vec<LocVar>,
    /// upvalues 名列表
    upvalue_names: Vec<String>
}
```
+ 2022-12-14 18.17 完成指令集的读取与解释, 从中提取出操作码等信息(opcodes.rs)
```rs
/// 每条指令基本信息
#[derive(Debug)]
struct OpCode {
    /// 编码模式
    test_flag: u8,
    /// 是否设置寄存器A
    set_a_flag: u8,
    /// 操作数B的使用类型
    arg_b_mode: u8,
    /// 操作数C的使用类型
    arg_c_mode: u8,
    /// 使用类型
    op_mode: u8,
    /// 操作码名称
    name: String
}
```
+ 2022-12-14 23.04 实现了一个简易的 LuaState 和 LuaStack(lua_state.rs lua_stack.rs)
```rs
/// 解释器状态
#[derive(Debug)]
pub struct luaState {
    pub statck: luaStack
}

/// luaStack 虚拟栈
#[derive(Debug)]
pub struct luaStack {
    /// 存放值
    slots: Vec<luaValue>,
    /// 栈顶索引
    pub(crate) top: i64
}
```
+ 2022-12-15 12.42 实现了 25 个运算符 (math/)
```rs
/// 分为基础运算和自动类型转换两种方法
pub struct Math;

pub fn Arith(&mut self, op: ArithOp);
pub(crate) fn Compare(&mut self, idx1: i64, idx2: i64, op: CompareOp) -> bool;
pub fn Concat(&mut self, n: i64);
pub fn Len(&mut self, idx: i64);
```
+ 2022-12-15 19.07 实现简易 VM, 但是有不少 bug, 需要进一步修改 (vm/)
```rs
pub mod inst_operators;
pub mod inst_load;
pub mod r#for;
pub mod inst_misc;
pub mod execute;
```
+ 2022-12-16 18.08 代码纠正, 成功计算 1+2+3+...+100 (*.rs)

+ 2022-12-18 15.49 实现了 Lua 表; 用表改写 1+2+3+...+100, 正确执行(table/)
```rs
/// LuaTable 数据结构 包含数组和HashMao
#[derive(Debug, Clone)]
pub struct LuaTable {
    /// 存放数组部分
    pub arr: Vec<luaValue>,
    /// 存放哈希表部分
    pub map: HashMap<luaValue, luaValue>,
    rdm: usize, // hash code
}
```
