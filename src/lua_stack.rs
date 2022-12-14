//! Lua State 最基础的是内部封装了个特殊的栈

use std::mem::swap;

/// lua 值的数据类型
#[derive(Debug, Clone)]
pub enum luaValue {
    NIL(Option<i64>),
    BOOL(bool),
    I64(i64),
    F64(f64),
    Str(String)
}

impl luaValue {
    /// 打印 Lua 值的类型
    pub fn T(a: luaValue) -> String {
        match a {
            NiL => { return "nil".to_string(); },
            luaValue::BOOL(_) => { return "bool".to_string(); },
            luaValue::I64(_) => { return "i64".to_string(); },
            luaValue::F64(_) => { return "f64".to_string(); },
            luaValue::Str(_) => { return "String".to_string(); }
        }
    }
}

/// luaStack 虚拟栈
#[derive(Debug)]
pub struct luaStack {
    /// 存放值
    slots: Vec<luaValue>,
    /// 栈顶索引
    pub(crate) top: i64
}

impl luaStack {
    /// 创建指定容量的栈
    pub fn newLuaStack(size: i64) -> luaStack {
        luaStack {
            slots: Vec::with_capacity(size as usize),
            top: 0
        }
    }

    /// 检查栈的空闲空间是否还可以推入至少n个值, 不满足的话, 扩容
    pub fn check(&mut self, n: i64) {
        let mut free = self.slots.len() as i64-self.top;
        while free < n as usize as i64 {
            self.slots.push(luaValue::NIL(None));
            free += 1;
        }
    }

    /// 将值推入栈顶
    pub(crate) fn push(&mut self, val: luaValue) {
        // println!("top={} len={}", self.top, self.slots.len());
        // if self.top == self.slots.len() as i64 {
        //     panic!("stack overflow, 栈溢出");
        // }
        // self.slots.push(val);
        // // self.slots[self.top as usize] = val;
        // // self.slots.push(val);
        // self.top += 1;
        self.slots.push(val);
        self.top += 1;
    }

    /// 从栈顶弹出一个值
    pub(crate) fn pop(&mut self) -> luaValue {
        if self.top < 1 {
            panic!("stack underflow, 栈为空");
        }
        self.top -= 1;
        self.slots.pop().unwrap()
    }

    /// 将索引转为绝对索引 没有考虑索引是否有效
    pub(crate) fn absIndex(&mut self, idx: i64) -> i64 {
        if idx >= 0 {
            return idx;
        }
        return idx + self.top + 1;
    }

    /// 判断索引是否有效
    pub(crate) fn isValid(&mut self, idx: i64) -> bool {
        let absInx = self.absIndex(idx);
        absInx > 0 && absInx <= self.top
    }

    /// 由索引从栈取值，无效则返回None
    pub(crate) fn get(&mut self, idx: i64) -> Option<luaValue> {
        let idx = self.absIndex(idx);
        if self.isValid(idx) {
            return Some(self.slots[(idx - 1) as usize].clone());
        }
        None
    }

    /// 向栈写入值
    pub(crate) fn set(&mut self, idx: i64, val: luaValue) {
        let idx = self.absIndex(idx);
        if idx > 0 && idx <= self.top {
            self.slots[(idx-1) as usize] = val;
            return;
        }
        panic!("invalid index, 索引无效")
    }

    pub(crate) fn reverse(&mut self, mut from: i64, mut to: i64) {
        while from < to {
            self.slots.swap(from as usize, to as usize);
            from += 1;
            to -= 1;
        }
    }

    // 以上是 luaStack 的基本操作, 通过这些东西来进一步实现 lua State 和 Lua API
}
