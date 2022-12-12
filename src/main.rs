pub mod header;

fn main() {
    // 从 chunk 中读取头部 Header 并检查 check()
    let header = header::Header::new("src/luac.out");
    header.check();
    println!("{}", header);
}
