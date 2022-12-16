//! for 循环相关指令, 分为数值、通用循环. 简易vm先讨论数值for循环

use crate::{ArithOp, CompareOp, LUA_OPADD};
use crate::lua_stack::luaValue;
use crate::lua_state::luaState;
use crate::math::consts::{LUA_OPLE, LUA_OPSUB};
use crate::opcodes::Instruction;

/// 数值 for 循环需要借助 FORPREP 和 FORLOOP

impl luaState {
    pub fn forPrep(&mut self, code: u32) {
        let (mut a, sBx) = Instruction::new(code).iAsBx();
        a += 1;
        // println!("a={};sBx={}", a, sBx);

        // println!("type={:?}", self.statck.get(a+2)); //type=Some(I64(1))
        if let luaValue::Str(s) = self.statck.get(a).unwrap() {
            let _a = self.ToNumber(a);
            self.PushNumber(_a);
            self.Replace(a);
        }
        if let luaValue::Str(s) = self.statck.get(a+1).unwrap() {
            let _a = self.ToNumber(a+1);
            self.PushNumber(_a);
            self.Replace(a+1);
        }
        if let luaValue::Str(s) = self.statck.get(a+2).unwrap() {
            let _a = self.ToNumber(a+2);
            self.PushNumber(_a);
            self.Replace(a+2);
        }


        // R(A)-=R(A+2)
        self.PushValue(a);
        self.PushValue(a+2);
        self.Arith(LUA_OPSUB as ArithOp);
        self.Replace(a);
        // pc+=sBx
        // println!("forprep sbx = {}; self.pc = {}", sBx, self.PC());
        self.AddPC(sBx);
        // println!("forprep self.pc = {}", self.PC());
    }

    pub fn forLoop(&mut self, code: u32) {
        let (mut a, sBx) = Instruction::new(code).iAsBx();
        a += 1;
        // println!("forLoop a{}; sbx={}", a, sBx);

        // R(A)+=R(A+2)
        self.PushValue(a+2);
        self.PushValue(a);
        self.Arith(LUA_OPADD as ArithOp);
        // println!("forLoop luaStack = {:?}", self.statck);
        self.Replace(a);
        // println!("forLoop luaStack = {:?}", self.statck);

        // 后面的代码未完成 待填充
        // R(A)=0
        // if
        let positive_step = self.ToNumber(a+2) >= 0.0;
        // println!("positive_step {}", positive_step);
        // println!("vm.compare(a, a + 1, LUA_OPLE) = {}", self.Compare(a, a+1, LUA_OPLE as CompareOp));
        // println!("=====a={}", a);
        if positive_step && self.Compare(a, a+1, LUA_OPLE as CompareOp)
            || !positive_step && self.Compare(a+1, a, LUA_OPLE as CompareOp) {
            // pc+=sBx; R(A+3)=R(A)
            // println!("forloop pc={} 里面", self.pc);
            self.AddPC(sBx);
            self.Copy(a, a+3);
        }
        // println!("forloop pc={} 外面", self.pc);
    }
}
