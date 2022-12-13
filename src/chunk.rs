//! 读取 chunk 到 Chunk 结构体, 包含了头部、主函数 upvalues、主函数原型

use std::fs;
use crate::header;

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

impl Chunk {
    /// 读取 chunk
    pub fn new(file: &str) -> Chunk {
        let header = header::Header::new(file);
        header.check();

        let v = fs::read(file).unwrap();
        let sizeUpvalues = v[33];

        let mut mainFunc = MainFunc::new(file).readProto("".to_string());

        Chunk {
            header,
            sizeUpvalues,
            mainFunc
        }
    }
}

/// upvalue 结构
#[derive(Debug)]
struct Upvalue {
    in_stack: u8,
    idx: u8
}

/// 局部变量
#[derive(Debug)]
struct LocVar {
    name: String,
    start_pc: u32,
    end_pc: u32
}

const TAG_NIL: u8 = 0x00;
const TAG_BOOLEAN: u8 = 0x01;
const TAG_NUMBER: u8 = 0x03;
const TAG_INTEGER: u8 = 0x13;
const TAG_SHORT_STR: u8 = 0x04;
const TAG_LONG_STR: u8 = 0x14;

/// 常量定义
#[derive(Debug)]
enum Constant {
    TAG_NIL,
    TAG_BOOLEAN(u8),
    TAG_NUMBER(f64),
    TAG_INTEGER(u64),
    TAG_SHORT_STR(String),
    TAG_LONG_STR(String)
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

/// 用来从 chunk 读取数据的工具包
#[derive(Debug)]
struct MainFunc {
    stream: Vec<u8>,
    position: usize
}

impl MainFunc {
    /// 跳过 header 读取主函数原型
    fn new(file: &str) -> MainFunc {
        let v = fs::read(file).unwrap();
        // println!("主函数 Upvalue 数量: {}", v[33]);
        MainFunc {
            stream: v,
            // 跳过 Upvalue 数量
            position: 34
        }
    }

    fn length(&mut self) -> i64 {
        self.stream.len() as i64
    }

    /// 读取一个字节
    fn readByte(&mut self) -> u8 {
        let res = self.stream[self.position];
        self.position += 1;
        res
    }

    /// 读取多个字节 nums: 读取的字节数
    fn readBytes(&mut self, nums: i64) -> Vec<u8> {
        let res = self.stream[self.position..(self.position+nums as usize)].to_vec();
        self.position += nums as usize;
        res
    }

    /// lua 整数读取
    fn readLuaInteger(&mut self) -> u64 {
        u64::from_le_bytes(<[u8; 8]>::try_from(self.readBytes(8)).unwrap())
    }

    /// lua 浮点数读取
    fn readLuaNumber(&mut self) -> f64 {
        f64::from_le_bytes(<[u8; 8]>::try_from(self.readBytes(8)).unwrap())
    }

    fn test(&mut self) {
        // println!("源文件名: {} (@开头表示chunk由lua文件编译而来)", self.readString());
        // println!("{}", self.readByte());
        // println!("{:?}", self.readBytes(4));
    }

    /// 读取一个 u32
    fn readU32(&mut self) -> u32 {
        u32::from_le_bytes(<[u8; 4]>::try_from(self.readBytes(4)).unwrap())
    }

    /// 读取一个 u64
    fn readU64(&mut self) -> u64 {
        u64::from_le_bytes(<[u8; 8]>::try_from(self.readBytes(4)).unwrap())
    }

    /// 读取字符串
    fn readString(&mut self) -> String {
        let mut size = self.readByte();
        // 空字符串
        if size == 0 {
            return "".to_string();
        }

        // 长字符串
        if size == 0xFF {
            size = self.readByte();
        }
        let res = self.readBytes((size - 1) as i64);
        String::from_utf8(res).unwrap()
    }

    /// 指令表读取
    fn readCode(&mut self) -> Vec<u32> {
        let mut vec = Vec::with_capacity(self.readU32() as usize);
        for i in 0..vec.capacity() {
            vec.push(self.readU32());
        }
        vec
    }

    /// 读取一个常量
    fn readConstant(&mut self) -> Constant {
        match self.readByte() {
            TAG_NIL => return Constant::TAG_NIL,
            TAG_BOOLEAN => return Constant::TAG_BOOLEAN(self.readByte()),
            TAG_INTEGER => return Constant::TAG_INTEGER(self.readLuaInteger()),
            TAG_NUMBER => return Constant::TAG_NUMBER(self.readLuaNumber()),
            TAG_SHORT_STR => return Constant::TAG_SHORT_STR(self.readString()),
            TAG_LONG_STR => return Constant::TAG_LONG_STR(self.readString()),
            _ => panic!("corrupted")
        }
    }

    /// 常量表读取
    fn readConstants(&mut self) -> Vec<Constant> {
        let mut vec = Vec::with_capacity(self.readU32() as usize);
        for i in 0..vec.capacity() {
            vec.push(self.readConstant());
        }
        vec
    }

    /// Upvalues 表读取
    fn readUpvalues(&mut self) -> Vec<Upvalue> {
        let mut vec = Vec::with_capacity(self.readU32() as usize);
        for i in 0..vec.capacity() {
            vec.push(Upvalue { in_stack: self.readByte(), idx: self.readByte() });
        }
        vec
    }

    /// 行号表读取
    fn readLineInfo(&mut self) -> Vec<u32> {
        let mut vec = Vec::with_capacity(self.readU32() as usize);
        for i in 0..vec.capacity() {
            vec.push(self.readU32());
        }
        vec
    }

    /// 局部变量表读取
    fn readLocVars(&mut self) -> Vec<LocVar> {
        let mut vec = Vec::with_capacity(self.readU32() as usize);
        for i in 0..vec.capacity() {
            vec.push(LocVar {
                name: self.readString(),
                start_pc: self.readU32(),
                end_pc: self.readU32()
            });
        }
        vec
    }

    /// Upvalue名列表
    fn readUpvalueNames(&mut self) -> Vec<String> {
        let mut vec = Vec::with_capacity(self.readU32() as usize);
        for i in 0..vec.capacity() {
            vec.push(self.readString());
        }
        vec
    }

    /// 子函数列表读取
    fn readProtos(&mut self, parentSource: String) -> Vec<ProtoType> {
        let mut vec = Vec::with_capacity(self.readU32() as usize);
        for i in 0..vec.capacity() {
            vec.push(self.readProto(parentSource.clone()));
        }
        vec
    }

    /// 函数原型读取
    fn readProto(&mut self, parentSource: String) -> ProtoType {
        let mut source = self.readString();
        if source == "".to_string() { source = parentSource; }

        ProtoType {
            source: source.clone(),
            line_start: self.readU32(),
            line_end: self.readU32(),
            nums_params: self.readByte(),
            is_vararg: self.readByte(),
            max_stack_size: self.readByte(),
            code: self.readCode(),
            constants: self.readConstants(),
            upvalues: self.readUpvalues(),
            protos: self.readProtos(source),
            line_info: self.readLineInfo(),
            loc_vars: self.readLocVars(),
            upvalue_names: self.readUpvalueNames()
        }
    }
}
