//! LuaTable 的 get 类方法

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use crate::lua_stack::luaValue;
use crate::lua_state::LuaType;
use crate::luaState;
use crate::table::LuaTable;

/// 拓展表功能
impl luaState {
    /// 创建空表 将其推到栈顶
    pub fn CreateTable(&mut self, nArr: i64, nRec: i64) {
        let table = LuaTable::new(nArr, nRec);
        // 别人的实现
        self.statck.push(luaValue::Table(Rc::new(RefCell::new(LuaTable::new(nArr, nRec)))));
    }

    /// CreateTable 的特例 用于无法预先估计表的用法和容量
    pub fn NewTable(&mut self) {
        self.CreateTable(0, 0);
    }

    /// 根据键(从栈顶弹出) 从表里取值 然后j将值推入栈顶并返回值类型
    pub fn GetTable(&mut self, idx: i64) -> LuaType {
        // 不知道是不是夺走数据/所有权
        let mut table = self.statck.get(idx).unwrap();
        let k = self.statck.pop();
        self.getTable(&table, &k)
    }

    /// 从表里取值
    pub fn getTable(&mut self, t: &luaValue, k: &luaValue) -> LuaType {
        // 这个可能有问题
        if let luaValue::Table(tbl) = t {
            let v = tbl.borrow().get(k);
            self.statck.push(v);
            // 这个返回栈顶索引可能有问题
            let top = self.GetTop();
            return self.Type(top);
        }
        panic!("不是 table!")
    }

    /// 和 GetTable 类似; 只不过键不是从栈顶弹出的任意值 而是由参数传入的字符串
    pub fn GetField(&mut self, idx: i64, k: String) -> LuaType {
        let t = self.statck.get(idx).unwrap();
        self.getTable(&t, &luaValue::Str(k))
    }

    /// 和 GetField 类似 只不过由参数传入的键是数字而非字符串 该方法为数组准备 用来根据索引获取数组元素
    pub fn GetI(&mut self, idx: i64, i: i64) -> LuaType {
        let t = self.statck.get(idx).unwrap();
        return self.getTable(&t, &luaValue::I64(i));
    }
}
