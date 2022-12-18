//! 按索引批量设置数组元素 SETLIST(iABC)

use crate::{Instruction, luaState};

impl luaState {
    /// SETLIST 指令
    pub fn SETLIST(&mut self, code: u32) {
        let (mut a, b, mut c) = Instruction::new(code).iABC();
        a += 1;
        if c > 0 {
            c = c-1;
        } else {
            c = Instruction::new(self.Fetch()).iAx();
        }

        let mut idx = c*LFIELDS_PER_FLUSH as i64;
        for j in 1..=b {
            idx += 1;
            self.PushValue(a+j);
            self.SetI(a, idx);
        }
        // println!("setlist    a={};b={};c={}", a, b, c);
    }
}

/// 每次批处理的元素数量
pub const LFIELDS_PER_FLUSH: isize = 50;
