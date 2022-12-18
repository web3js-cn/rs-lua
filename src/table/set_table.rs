//! 根据键往表里赋值 SETTABLE(iABC)

use crate::{Instruction, luaState};

impl luaState {
    /// SETTABLE 指令
    pub fn SETTABLE(&mut self, code: u32) {
        let (mut a, b, c) = Instruction::new(code).iABC();
        a += 1;
        // println!("\nIABC: a={}; b={}; c={}", a, b, c);

        self.GetRK(b);
        self.GetRK(c);
        self.SetTable(a);
    }
}
