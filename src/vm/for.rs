//! for 循环相关指令, 分为数值、通用循环. 简易vm先讨论数值for循环

use crate::{ArithOp, LUA_OPADD};
use crate::lua_state::luaState;
use crate::math::consts::LUA_OPSUB;
use crate::opcodes::Instruction;

/// 数值 for 循环需要借助 FORPREP 和 FORLOOP

impl luaState {
    pub fn forPrep(&mut self, code: u32) {
        let (mut a, b) = Instruction::new(code).iAsBx();
        a += 1;

        // R(A)-=R(A+2)
        self.PushValue(a);
        self.PushValue(a+2);
        self.Arith(LUA_OPSUB as ArithOp);
        self.Replace(a);
        // pc+=sBx
        self.AddPC(b);
    }

    pub fn forLoop(&mut self, code: u32) {
        let (mut a, sBx) = Instruction::new(code).iAsBx();
        a += 1;

        // R(A)+=R(A+2)
        self.PushValue(a+2);
        self.PushValue(a);
        self.Arith(LUA_OPADD as ArithOp);
        self.Replace(a);

        // 后面的代码未完成 待填充
        // R(A)=0
        // if
    }
}
