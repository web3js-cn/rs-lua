//! 指令集

/// 指令编码模式
/// 3操作数 8 9 9
const IABC: u8 = 0;
/// 2操作数 8 18
const IABx: u8 = 1;
/// 2操作数 8 18
const IAsBx: u8 = 2;
/// 1操作数 26
const IAx: u8 = 3;

/// 操作码 用于识别指令 操作码常量
pub const OP_MOVE: u8 = 0;
pub const OP_LOADK: u8 = 1;
pub const OP_LOADKX: u8 = 2;
pub const OP_LOADBOOL: u8 = 3;
pub const OP_LOADNIL: u8 = 4;
pub const OP_GETUPVAL: u8 = 5;
pub const OP_GETTABUP: u8 = 6;
pub const OP_GETTABLE: u8 = 7;
pub const OP_SETTABUP: u8 = 8;
pub const OP_SETUPVAL: u8 = 9;
pub const OP_SETTABLE: u8 = 10;
pub const OP_NEWTABLE: u8 = 11;
pub const OP_SELF: u8 = 12;
pub const OP_ADD: u8 = 13;
pub const OP_SUB: u8 = 14;
pub const OP_MUL: u8 = 15;
pub const OP_MOD: u8 = 16;
pub const OP_POW: u8 = 17;
pub const OP_DIV: u8 = 18;
pub const OP_IDIV: u8 = 19;
pub const OP_BAND: u8 = 20;
pub const OP_BOR: u8 = 21;
pub const OP_BXOR: u8 = 22;
pub const OP_SHL: u8 = 23;
pub const OP_SHR: u8 = 24;
pub const OP_UNM: u8 = 25;
pub const OP_BNOT: u8 = 26;
pub const OP_NOT: u8 = 27;
pub const OP_LEN: u8 = 28;
pub const OP_CONCAT: u8 = 29;
pub const OP_JMP: u8 = 30;
pub const OP_EQ: u8 = 31;
pub const OP_LT: u8 = 32;
pub const OP_LE: u8 = 33;
pub const OP_TEST: u8 = 34;
pub const OP_TESTSET: u8 = 35;
pub const OP_CALL: u8 = 36;
pub const OP_TAILCALL: u8 = 37;
pub const OP_RETURN: u8 = 38;
pub const OP_FORLOOP: u8 = 39;
pub const OP_FORPREP: u8 = 40;
pub const OP_TFORCALL: u8 = 41;
pub const OP_TFORLOOP: u8 = 42;
pub const OP_SETLIST: u8 = 43;
pub const OP_CLOSURE: u8 = 44;
pub const OP_VARARG: u8 = 45;
pub const OP_EXTRAARG: u8 = 46;

/// 操作数类型
/// 参数不被使用
const OpArgN: u8 = 0;
const OpArgU: u8 = 1;
/// iABC表示寄存器索引 iAsBx表示跳转偏移
const OpArgR: u8 = 2;
const OpArgK: u8 = 3;

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
    name: String,
}

/// 完整的指令表
#[derive(Debug)]
struct OpCodes {
    opcodes: [OpCode; 47]
}

impl OpCodes {
    fn new() -> OpCodes {
        OpCodes {
            opcodes: [
                /*           T               A               B               C           mode            name       */
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgN, op_mode: IABC, name: "MOVE    ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgN, op_mode: IABx, name: "LOADK   ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgN, arg_c_mode: OpArgN, op_mode: IABx, name: "LOADKx  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgU, arg_c_mode: OpArgU, op_mode: IABC, name: "LOADBOOL    ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgU, arg_c_mode: OpArgN, op_mode: IABC, name: "LOADNIL ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgU, arg_c_mode: OpArgN, op_mode: IABC, name: "GETUPVAL".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgU, arg_c_mode: OpArgK, op_mode: IABC, name: "GETTABUP".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgK, op_mode: IABC, name: "GETTABLF".to_string() },
                OpCode { test_flag: 0, set_a_flag: 0, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "SETTABUP".to_string() },
                OpCode { test_flag: 0, set_a_flag: 0, arg_b_mode: OpArgU, arg_c_mode: OpArgN, op_mode: IABC, name: "SETUPVAL".to_string() },
                OpCode { test_flag: 0, set_a_flag: 0, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "SETTABLE".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgU, arg_c_mode: OpArgU, op_mode: IABC, name: "NEWTABLE".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgK, op_mode: IABC, name: "SELF ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "ADD  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "SUB  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "MUL  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "MOD  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "POW  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "DIV  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "IDIV ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "BAND ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "BOR  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "BXOR ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "SHL  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "SHR  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgN, op_mode: IABC, name: "UNM  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgN, op_mode: IABC, name: "BNOT ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgN, op_mode: IABC, name: "NOT  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgN, op_mode: IABC, name: "LEN  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgR, op_mode: IABC, name: "CONCAT   ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 0, arg_b_mode: OpArgR, arg_c_mode: OpArgN, op_mode: IAsBx, name: "JMP  ".to_string() },
                OpCode { test_flag: 1, set_a_flag: 0, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "EQ   ".to_string() },
                OpCode { test_flag: 1, set_a_flag: 0, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "LT   ".to_string() },
                OpCode { test_flag: 1, set_a_flag: 0, arg_b_mode: OpArgK, arg_c_mode: OpArgK, op_mode: IABC, name: "LE   ".to_string() },
                OpCode { test_flag: 1, set_a_flag: 0, arg_b_mode: OpArgN, arg_c_mode: OpArgU, op_mode: IABC, name: "TEST ".to_string() },
                OpCode { test_flag: 1, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgU, op_mode: IABC, name: "TESTSET  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgU, arg_c_mode: OpArgU, op_mode: IABC, name: "CALL ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgU, arg_c_mode: OpArgU, op_mode: IABC, name: "TAILCALL".to_string() },
                OpCode { test_flag: 0, set_a_flag: 0, arg_b_mode: OpArgU, arg_c_mode: OpArgN, op_mode: IABC, name: "RETURN   ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgN, op_mode: IAsBx, name: "FORLOOP  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgN, op_mode: IAsBx, name: "FORPREP  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 0, arg_b_mode: OpArgN, arg_c_mode: OpArgU, op_mode: IABC, name: "TFORCALL".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgR, arg_c_mode: OpArgN, op_mode: IAsBx, name: "TFORLOOP".to_string() },
                OpCode { test_flag: 0, set_a_flag: 0, arg_b_mode: OpArgU, arg_c_mode: OpArgU, op_mode: IABC, name: "SETLIST  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgU, arg_c_mode: OpArgN, op_mode: IABx, name: "CLOSURE  ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 1, arg_b_mode: OpArgU, arg_c_mode: OpArgN, op_mode: IABC, name: "VARARG   ".to_string() },
                OpCode { test_flag: 0, set_a_flag: 0, arg_b_mode: OpArgU, arg_c_mode: OpArgU, op_mode: IAx, name: "EXTRAARG".to_string() },
            ]
        }
    }
}

/// 指令解码
pub struct Instruction {
    code: u32
}

impl Instruction {
    /// 构造解码对象
    pub fn new(code: u32) -> Instruction {
        Instruction {
            code
        }
    }

    /// 提取操作码
    fn opcode(&mut self) -> i64 {
        (self.code & 0x3F) as i64
    }

    /// 从 iABC 模式提取参数
    pub(crate) fn iABC(&mut self) -> (i64, i64, i64) {
        // println!("{}", self.code);
        let a = self.code >> 6 & 0xFF;
        let b = self.code >> 14 & 0x1FF;
        let c = self.code >> 23 & 0x1FF;
        (a as i64, b as i64, c as i64)
    }

    /// 从 iABx 模式提取参数
    pub(crate) fn iABx(&mut self) -> (i64, i64) {
        let a = self.code >> 6 & 0xFF;
        let b = self.code >> 14 & 0x1FF;
        (a as i64, b as i64)
    }

    /// 从 iAsBx 模式提取参数
    pub(crate) fn iAsBx(&mut self) -> (i64, i64) {
        let (a, b) = self.iABx();
        (a as i64, (b - MAXARG_sBx as i64) as i64)
    }

    /// 从 iAx 模式提取参数
    pub(crate) fn iAx(&mut self) -> i64 {
        (self.code >> 6) as i64
    }

    /// 获取指令名
    pub fn op_name(&mut self) -> String {
        OpCodes::new().opcodes[self.opcode() as usize].name.clone()
    }

    /// 获取指令模式
    fn op_mode(&mut self) -> u8 {
        OpCodes::new().opcodes[self.opcode() as usize].op_mode
    }

    /// 获取B操作数的模式
    fn b_mode(&mut self) -> u8 {
        OpCodes::new().opcodes[self.opcode() as usize].arg_b_mode
    }

    /// 获取C操作数的模式
    fn c_mode(&mut self) -> u8 {
        OpCodes::new().opcodes[self.opcode() as usize].arg_c_mode
    }

    /// 获取指令模式的字符串形式
    pub fn print_op_mode(&mut self) -> String {
        match self.op_mode() {
            IABC => "IABC".to_string(),
            IAsBx => "IAsBx".to_string(),
            IABx => "IABx".to_string(),
            IAx => "IAx".to_string(),
            _ => panic!("获取失败")
        }
    }

    /// B C操作数类型打印
    pub fn b_c_mode_print(&mut self) -> (String, String) {
        let mode = ["OpArgN".to_string(), "OpArgU".to_string(), "OpArgR".to_string(), "OpArgK".to_string()];
        let b = self.b_mode();
        let c = self.c_mode();
        (mode[b as usize].clone(), mode[c as usize].clone())
    }

    /// 打印指令操作数
    pub fn printOperands(&mut self) -> (Option<i64>, Option<i64>, Option<i64>) {
        match self.op_mode() {
            IABC => {
                let (mut a, mut b, mut c) = self.iABC();
                if self.b_mode() != OpArgN && b > 0xFF { b = -1-b & 0xFF; }
                if self.c_mode() != OpArgN && c > 0xFF { c = -1-c & 0xFF; }
                if self.b_mode() == OpArgN && self.c_mode() != OpArgN { return (Some(a), None, Some(c)); }
                if self.c_mode() == OpArgN && self.b_mode()!= OpArgN { return (Some(a), Some(b), None); }
                if self.c_mode() == OpArgN && self.b_mode() == OpArgN { return (Some(a), None, None); }
                return (Some(a), Some(b), Some(c));
            },
            IABx => {
                let (a, mut bx) = self.iABx();
                if self.b_mode() == OpArgK { bx = -1-bx; }
                if self.b_mode() == OpArgU { bx = bx }
                return (Some(a), Some(bx), None);
            },
            IAsBx => {
                let (a, sbx) = self.iAsBx();
                return (Some(a), Some(sbx), None);
            },
            IAx => {
                return (Some(-1-self.iAx()), None, None);
            },
            _ => panic!("无法获取指令模式")
        }
    }
}

const MAXARG_Bx: u32 = 1 << 18 - 1;
const MAXARG_sBx: u32 = MAXARG_Bx >> 1;
