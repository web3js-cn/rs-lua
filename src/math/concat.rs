//! 从栈顶弹出 n 个值, 对这些值进行拼接, 然后把结果推入栈顶

use crate::lua_stack::luaValue;
use crate::lua_state::luaState;

impl luaState {
    pub fn Concat(&mut self, n: i64) {
        if n == 0 {
            self.statck.push(luaValue::Str("".to_string()));
        } else if n >= 2 {
            for i in 1..n {
                if self.IsString(-1) && self.IsString(-2) {
                    let s1 = self.ToString(-1);
                    let s2 = self.ToString(-2);
                    self.statck.pop();
                    self.statck.pop();
                    self.statck.push(luaValue::Str(format!("{}{}", s1, s2)));
                    continue;
                }
                panic!("concatenation error");
            }
        }
        //n = 1, do nothing
    }
}
