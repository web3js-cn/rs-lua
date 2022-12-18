//! 创建空表的指令 NEWTABLE(iABC)

use crate::{Instruction, luaState};
use crate::table::fpb::fb2int;

impl luaState {
    /// NEWTABLE 指令
    pub fn NEWTABLE(&mut self, code: u32) {
        let (mut a, b, c) = Instruction::new(code).iABC();
        a += 1;
        self.CreateTable(fb2int(b as usize) as i64, fb2int(c as usize) as i64);
        self.Replace(a);
    }
}
