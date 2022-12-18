//! 本模块是 LuaTable 实现

pub mod base;
pub mod hashmap_for_luaValue;
pub mod get;
pub mod set;
pub mod new_table;
pub mod fpb;
pub mod get_table;
pub mod set_table;
pub mod set_list;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::lua_stack::luaValue;

/// LuaTable 数据结构 包含数组和HashMao
#[derive(Debug, Clone)]
pub struct LuaTable {
    /// 存放数组部分
    pub arr: Vec<luaValue>,
    /// 存放哈希表部分
    pub map: HashMap<luaValue, luaValue>,
    rdm: usize, // hash code
}

impl Hash for LuaTable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rdm.hash(state);
    }
}
