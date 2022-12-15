//! 其他类型的指令 如MOVE、JMP

use crate::lua_state::luaState;
use crate::opcodes::Instruction;

impl luaState {
    /// MOVE 指令常用于局部变量赋值和参数传递
    pub fn MOVE(&mut self, code: u32) {
        let (mut a, mut b, _) = Instruction::new(code).iABC();
        a += 1;
        b += 1;
        self.Copy(b, a);
    }

    /// JMP(iAsBx) 执行无条件跳转
    pub fn JMP(&mut self, code: u32) {
        // 操作数 a 与 Upvalue 相关
        let (a, sBx) = Instruction::new(code).iAsBx();
        self.AddPC(sBx);
        if a != 0 {
            panic!("todo!");
        }
    }
}
