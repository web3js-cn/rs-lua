//! 本模块是 lua vm 的形状, 本文件拓展了 luaState 指令执行相关方法

pub mod inst_operators;
pub mod inst_load;
pub mod r#for;
pub mod inst_misc;
pub mod execute;

use crate::chunk::Constant;
use crate::lua_stack::luaValue;
use crate::lua_state::luaState;

impl luaState {
    pub(crate) fn PC(&mut self) -> i64 {
        self.pc
    }

    fn AddPC(&mut self, n: i64) {
        self.pc += n;
    }

    /// 根据索引从函数原型的指令表里取出当前指令, 然后递增 pc
    pub(crate) fn Fetch(&mut self) -> u32 {
        // print!("Fetch Pc={} ", self.pc);
        let i = self.proto.code[self.pc as usize];
        self.pc += 1;
        i
    }

    /// 根据索引从函数原型的常量表里取出一个常量值, 然后把它推入栈顶
    /// 这个方法可能会有问题 需要留意
    fn GetConst(&mut self, idx: i64) {
        let val = self.proto.constants.get(idx as usize);
        // println!("\nGetConst 调用: {:?}", val.unwrap());
        if let Constant::TAG_LONG_STR(s) = val.unwrap() {
            self.statck.push(luaValue::Str(s.clone().to_owned()));
        }
        if let Constant::TAG_SHORT_STR(s) = val.unwrap() {
            self.statck.push(luaValue::Str(s.clone().to_owned()));
        }
        if let Constant::TAG_BOOLEAN(b) = val.unwrap() {
            if b == &0u8 {
                self.statck.push(luaValue::BOOL(false));
            } else {
                self.statck.push(luaValue::BOOL(true));
            }
        }
        if let Constant::TAG_NUMBER(f) = val.unwrap() {
            self.statck.push(luaValue::F64(f.to_owned()));
        }
        if let Constant::TAG_NUMBER(i) = val.unwrap() {
            self.statck.push(luaValue::I64(i.to_owned() as i64));
        }
        if let Constant::TAG_INTEGER(int) = val.unwrap() {
            self.statck.push(luaValue::I64(int.to_owned() as i64));
        }
    }

    /// 调用 GetConst 将某个常量推入栈顶 或者调用 PushValue 将某个索引处的栈值推入栈顶
    fn GetRK(&mut self, rk: i64) {
        if rk > 0xFF {
            // constant
            self.GetConst(rk & 0xFF);
        } else {
            // register
            // println!("getrk rk={}", rk);
            self.PushValue(rk+1);
        }
    }
}
