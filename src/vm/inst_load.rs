//! 加载类指令

use crate::lua_state::luaState;
use crate::opcodes::Instruction;

impl luaState {
    /// 给连续 n 个寄存器放置 nil
    pub fn LoadNil(&mut self, code: u32) {
        let (mut a, b, _) = Instruction::new(code).iABC();
        a += 1;
        self.PushNil();
        for i in a..=a+b {
            self.Copy(-1, i);
        }
        self.Pop(1);
    }

    /// LOADBOOL给单个寄存器设置布尔值
    pub fn LoadBool(&mut self, code: u32) {
        let (mut a, b, c) = Instruction::new(code).iABC();
        a += 1;
        self.PushBoolean(b != 0);
        self.Replace(a);
        if c != 0 {
            self.AddPC(1);
        }
    }

    /// LOADK(iABx) 将常量表里的某个常量加载到指定寄存器
    pub(crate) fn LoadK(&mut self, code: u32) {
        let (mut a, bx) = Instruction::new(code).iABx();
        a += 1;
        self.GetConst(bx);
        self.Replace(a);
    }

    /// LOADKX 用来弥补 Bx 18bits 太小的问题, 与 EXTRAARG(iAx) 搭配使用
    pub fn LoadKX(&mut self, code: u32) {
        let (mut a, _) = Instruction::new(code).iABx();
        a += 1;
        let ax = Instruction::new(self.Fetch()).iAx();

        self.GetConst(ax);
        self.Replace(a);
    }
}
