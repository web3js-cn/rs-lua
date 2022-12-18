//! 实现 Len() 获取长度 访问指定索引处的值, 取其长度, 然后推入栈顶

use crate::lua_stack::luaValue;
use crate::lua_state::luaState;

impl luaState {
    pub fn Len(&mut self, idx: i64) {
        let val = self.statck.get(idx).unwrap();
        match val {
            luaValue::Str(x) => {
                self.statck.push(luaValue::I64(x.len() as i64))
            },
            luaValue::Table(t) => {
                self.statck.push(luaValue::I64(t.borrow_mut().len()));
            }
            _ => panic!("length error")
        }
    }
}
