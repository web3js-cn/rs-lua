//! 实现指令分发

use crate::lua_state::luaState;
use crate::opcodes::Instruction;

impl luaState {
    /// 指令分发
    pub fn Execute(&mut self, code: u32) {
        let name = Instruction::new(code).op_name();

        if name == "MOVE    ".to_string() { self.MOVE(code); }
        if name == "LOADK   ".to_string() { self.LoadK(code); }
        if name == "LOADKx  ".to_string() { self.LoadKX(code); }
        if name == "LOADBOOL    ".to_string() { self.LoadBool(code); }
        if name == "LOADNIL ".to_string() { self.LoadNil(code); }
        if name == "ADD  ".to_string() { self.add(code); }
        if name == "SUB  ".to_string() { self.sub(code); }
        if name == "MUL  ".to_string() { self.mul(code); }
        if name == "MOD  ".to_string() { self.MOD(code); }
        if name == "POW  ".to_string() { self.pow(code); }
        if name == "DIV  ".to_string() { self.div(code); }
        if name == "IDIV ".to_string() { self.idiv(code); }
        if name == "BAND ".to_string() { self.band(code); }
        if name == "BOR  ".to_string() { self.bor(code); }
        if name == "BXOR ".to_string() { self.bxor(code); }
        if name == "SHL  ".to_string() { self.shl(code); }
        if name == "SHR  ".to_string() { self.shr(code); }
        if name == "UNM  ".to_string() { self.unm(code); }
        if name == "BNOT ".to_string() { self.bnot(code); }
        if name == "NOT  ".to_string() { self.not(code); }
        if name == "LEN  ".to_string() { self._len(code); }
        if name == "CONCAT   ".to_string() { self.concat(code); }
        if name == "JMP  ".to_string() { self.JMP(code); }
        if name == "EQ   ".to_string() { self.eq(code); }
        if name == "LT   ".to_string() { self.lt(code); }
        if name == "LE   ".to_string() { self.le(code); }
        if name == "TEST ".to_string() { self.test(code); }
        if name == "TESTSET  ".to_string() { self.testSet(code); }
        if name == "FORLOOP  ".to_string() { self.forLoop(code); }
        if name == "FORPREP  ".to_string() { self.forPrep(code); }
        // ...待补充
    }
}
