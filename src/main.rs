pub mod header;
pub mod chunk;

fn main() {
    // 从 chunk 中读取头部 Header 并检查 check()
    // let header = header::Header::new("src/luac.out");
    // header.check();
    // println!("{}", header);


    // 读取 chunk 到 Chunk 结构体, 包含了头部、主函数 upvalues、主函数原型
    let mut chunk = chunk::Chunk::new("src/luac.out");
    println!("{:#?}", chunk);
}
