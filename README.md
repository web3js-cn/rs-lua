# rs-lua
使用 Rust 实现 Lua 解释器


## 使用
文档查看
```bash
cargo doc --open
```


## 进度
+ 2022-12-12 12.16 完成了从 chunk 中读取头部并进行验证
+ 2022-12-13 20.06 读取 chunk 到 Chunk 结构体, 包含了头部、主函数 upvalues、主函数原型
