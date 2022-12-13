# rs-lua
使用 Rust 实现 Lua 解释器


## 使用
文档查看
```bash
cargo doc --open
```


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
```
