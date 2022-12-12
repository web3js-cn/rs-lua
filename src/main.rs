pub mod header;

fn main() {
    let header = header::Header::new("src/luac.out");
    header.check();
    println!("{}", header);
}