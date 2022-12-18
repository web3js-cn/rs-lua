//! LuaTable 的基础实现

use std::collections::HashMap;
use crate::lua_stack::{luaStack, luaValue};
use crate::luaState;
use crate::math::Math;
use crate::table::LuaTable;

impl Math {
    pub fn random() -> usize {
        let ptr = Box::into_raw(Box::new(123));
        ptr as usize
    }
}


/// 基础方法 new get put set
impl LuaTable {
    /// 创建空表 接受两个参数估算表容量
    ///
    /// if nArr > 0 { 当作数组使用 }
    ///
    /// if nRec > 0 { 当作记录 Record 使用 }
    pub fn new(nArr: i64, nRec: i64) -> LuaTable {
        let mut table = LuaTable {
            arr: vec![],
            map: Default::default(),
            rdm: Math::random()
        };

        if nArr > 0 {
            table.arr = Vec::with_capacity(nArr as usize);
        }
        if nRec > 0 {
            table.map = HashMap::with_capacity(nRec as usize);
        }

        table
    }

    /// 根据键从表里查找值
    ///
    /// 如果 key 是整数那么直接按索引访问数组部分; 否则从哈希表里查找
    pub fn get(&self, key: &luaValue) -> luaValue {
        let key = LuaTable::_floatToInteger(key.clone());
        if let luaValue::I64(idx) = key {
            if idx >= 1 && idx <= self.arr.len() as i64 {
                return self.arr[(idx-1) as usize].clone();
            }
        }

        if let Some(val) = self.map.get(&key) {
            val.clone()
        } else {
            luaValue::NIL(None)
        }
    }

    /// 向表里存入键值对
    pub fn put(&mut self, key: luaValue, val: luaValue) {
        if key == luaValue::NIL(None) {
            panic!("插入表时键为 nil!");
        }

        if let luaValue::F64(f) = key {
            if f.is_nan() {
                panic!("table index is nan!");
            }
        }
        let key = LuaTable::_floatToInteger(key);

        if let luaValue::I64(idx) = key {
            // 检验是否有效
            if idx >= 1 {
                let arrLen = self.arr.len();
                if idx as usize <= arrLen {
                    self.arr[(idx-1) as usize] = val.clone();
                    if idx as usize == arrLen && val == luaValue::NIL(None) {
                        self._shrinkArray();
                    }
                    return;
                }
                if idx as usize == arrLen+1 {
                    self.map.remove(&key);
                    if val != luaValue::NIL(None) {
                        self.arr.push(val);
                        self._expandArray();
                    }
                    return;
                }
            }
        }

        if val != luaValue::NIL(None) {
            // if self.map.is_empty() {
            //     self.map = HashMap::with_capacity(8);
            // }
            self.map.insert(key, val);
        } else {
            self.map.remove(&key);
        }
    }

    pub fn len(&mut self) -> i64 {
        self.arr.len() as i64
    }
}

/// 辅助类的方法
impl LuaTable {
    /// 尝试将浮点数转换为整数
    pub fn _floatToInteger(key: luaValue) -> luaValue {
        if let luaValue::F64(f) = key {
            return luaValue::I64(Math::FloatToInteger(f).0);
        }
        key
    }

    /// 如果洞在函数末尾的话 调用此方法将尾部的洞全部清除
    pub fn _shrinkArray(&mut self) {
        // 空的话无意义
        // 使用数组要考虑为空的情况
        while !self.arr.is_empty() {
            if self.arr.last().unwrap() == &luaValue::NIL(None) {
                self.arr.pop();
            } else {
                break;
            }
        }

        // let mut i = self.arr.len()-1;
        // while i >= 0 {
        //     if self.arr[i] == luaValue::NIL(None) {
        //         // 可能会有问题
        //         self.arr.remove(i);
        //     }
        //     i -= 1;
        // }
    }

    /// 数组部分动态拓展后, 把原本存在哈希表里的值娜到数组
    pub fn _expandArray(&mut self) {
        let mut idx = self.arr.len()+1;
        loop {
            let key = luaValue::I64(idx as i64);
            if self.map.contains_key(&key) {
                let val = self.map.remove(&key).unwrap();
                self.arr.push(val);
                idx += 1;
            } else {
                break;
            }
        }
    }
}
