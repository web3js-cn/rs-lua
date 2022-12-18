//! 为 luaValue 实现# [derive(Eq, Hash, PartialEq)]
//!
//! 从而可以被 HashMap 支持

use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::os::macos::raw::stat;
use std::rc::Rc;
use crate::lua_stack::luaValue;

/// 这里的F64可能会有问题
impl PartialEq for luaValue {
    fn eq(&self, other: &luaValue) -> bool {
        if let (luaValue::NIL(None), luaValue::NIL(None)) = (self, other) {
            true
        } else if let (luaValue::BOOL(x), luaValue::BOOL(y)) = (self, other) {
            x == y
        } else if let (luaValue::I64(x), luaValue::I64(y)) = (self, other) {
            x == y
        } else if let (luaValue::F64(x), luaValue::F64(y)) = (self, other) {
            x == y
        } else if let (luaValue::Str(x), luaValue::Str(y)) = (self, other) {
            x == y
        } else if let (luaValue::Table(x), luaValue::Table(y)) = (self, other)  {
            Rc::ptr_eq(x, y)
        } else {
            false
        }
    }
}

///  the trait `std::cmp::Eq` is not implemented for `f64`
impl Eq for luaValue {}

/// the trait `std::hash::Hash` is not implemented for `f64`
impl Hash for luaValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            luaValue::NIL(_) => 0.hash(state),
            luaValue::BOOL(b) => b.hash(state),
            luaValue::I64(i) => i.hash(state),
            luaValue::F64(f) => f.to_bits().hash(state),
            luaValue::Str(s) => s.hash(state),
            // 别人的实现是 borrow() 好奇怪
            luaValue::Table(t) => t.borrow_mut().hash(state)
        }
    }
}
