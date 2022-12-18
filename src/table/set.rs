//! LuaTable 的 set 类方法

use crate::lua_stack::luaValue;
use crate::luaState;

/// 拓展表功能
impl luaState {
    /// 把键值对写入表; 键和值从栈里弹出, 表位于指定索引处
    pub fn SetTable(&mut self, idx: i64) {
        let t = self.statck.get(idx).unwrap();
        let v = self.statck.pop();
        let k = self.statck.pop();
        self.setTable(&t, k, v);
    }
    /// 抽取写表逻辑
    pub fn setTable(&mut self, t: &luaValue, k: luaValue, v: luaValue) {
        if let luaValue::Table(tbl) = t {
            // println!("k={:?}; v={:?}", k, v);
            tbl.borrow_mut().put(k, v);
        } else {
            panic!("not a table! src/table/set.rs");
        }
    }

    /// 与 SetTable 类似, 只不过键不是从栈顶弹出 而是参数传入的字符串
    pub fn SetField(&mut self, idx: i64, k: String) {
        let t = self.statck.get(idx).unwrap();
        let v = self.statck.pop();
        let k = luaValue::Str(k);
        self.setTable(&t, k, v);
    }

    /// 与 SetField 类似 只不过由参数传入的是数字而不是字符串 用于按索引修改元素
    pub fn SetI(&mut self, idx: i64, i: i64) {
        let t = self.statck.get(idx).unwrap();
        let v = self.statck.pop();
        let k = luaValue::I64(i);
        self.setTable(&t, k, v);
    }
}
