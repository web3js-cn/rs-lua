//! 根据键从表里取值 GETTABLE(iABC)

use crate::{Instruction, luaState};

impl luaState {
    /// GETTABLE 指令
    pub fn GETTABLE(&mut self, code: u32) {
        let (mut a, mut b, c) = Instruction::new(code).iABC();
        a += 1;
        b += 1;

        self.GetRK(c);
        self.GetTable(b);
        self.Replace(a);
    }
}
