//! 头部读取每个字段 + 格式化输出头部

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

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

/// 头部相关的方法是读取、验证
impl Header {
    /// 从 chunk 中读取头部, 传入 chunk 文件名
    pub fn new(file: &str) -> Header {
        let mut f = File::open(file).unwrap();
        let mut buf = vec![0; 33];
        let n = f.read(&mut buf[..]).unwrap();
        Header {
            signature: buf[..4].to_owned(),
            version: buf[4],
            format: buf[5],
            luac_data: buf[6..12].to_owned(),
            cint: buf[12],
            size_t: buf[13],
            vm: buf[14],
            integer_size: buf[15],
            number_size: buf[16],
            int: buf[17..25].to_owned(),
            num: buf[25..33].to_owned()
        }
    }

    /// 验证头部, 通过检查 Header 字段是否和预期一致
    pub fn check(&self) {
        if self.signature != [0x1B, 0x4C, 0x75, 0x61] {
            panic!("魔术识别失败");
        } else if self.version != 0x53 {
            panic!("版本号与本机不匹配");
        } else if self.format != 0x00 {
            panic!("格式号不匹配");
        } else if self.luac_data != [0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A] {
            panic!("LUAC_DATA匹配失败，文件可能已损坏");
        } else if self.cint !=0x04 {
            panic!("cint不匹配");
        } else if self.size_t != 0x08 {
            panic!("size_t不匹配");
        } else if self.vm != 0x04 {
            panic!("Lua虚拟机指令不匹配");
        } else if self.integer_size != 0x08 {
            panic!("Lua整数不是八位");
        } else if self.number_size != 0x08 {
            panic!("Lua浮点数不是八位");
        } else if self.int != [0x78, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] {
            panic!("大小端不匹配");
        } else if self.num != [0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x77, 0x40] {
            panic!("浮点数格式不匹配");
        }
    }
}

/// 将 Header 打印
impl Display for Header {
    /// 实现 fmt
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "=============================================================================================================================================\n🚀 impl Display for Header\n+ 魔数: {:?} (参考: 0x1B4C7561; 用途: 快速识别文件格式 <=> [27, 76, 117, 97])\n\
            + 版本: {}\n\
            + 格式号: {} (参考: 官方实现使用的格式号是 0x00)\n\
            + LUAC_DATA: {:?} (参考: 0x19 0x93 0x0D回车 0x0A换行 0x1A替换 0x0A换行 <=> [25, 147, 13, 10, 26, 10])\n\
            + cint: {} 字节\n\
            + size_t: {} 字节\n\
            + Lua虚拟机指令: {} 字节\n\
            + Lua整数: {} 字节\n\
            + Lua浮点数: {} 字节\n\
            + 大小端识别: {} (参考: 22136, 小端)\n\
            + 浮点数格式检测: {} (参考: 370.5, IEE754 标准)\n=============================================================================================================================================\n\n",
            self.signature,
            self.version,
            self.format,
            self.luac_data,
            self.cint,
            self.size_t,
            self.vm,
            self.integer_size,
            self.number_size,
            u64::from_le_bytes(<[u8; 8]>::try_from(self.int.clone()).unwrap()),
            f64::from_le_bytes(<[u8; 8]>::try_from(self.num.clone()).unwrap())
        )
    }
}
