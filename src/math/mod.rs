//! 本文件为运算符的基础方法 为后续大类运算符作准备
//! Lua 算术运算符都在本模块

pub mod consts;
pub mod arith;
pub mod compare;
pub mod len;
pub mod concat;

use std::num::ParseIntError;
use crate::lua_stack::luaValue;

/// 分为基础运算和自动类型转换两种方法
pub struct Math;

impl Math {
    /// 浮点数转整数
    pub(crate) fn FloatToInteger(f: f64) -> (i64, bool) {
        let i = f as i64;
        (i, i as f64 == f)
    }

    /// 字符串转整数
    fn ParseInteger(a: String) -> (i64, bool) {
        let a: Result<i64, _> = a.parse();
        match a {
            Ok(_a) => {
                (_a, true)
            }
            Err(_) => {
                (0, false)
            }
        }
        // a.parse().unwrap()
    }

    /// 字符串转浮点数
    fn ParseFloat(a: String) -> (f64, bool) {
        let a: Result<f64, _> = a.parse();
        match a {
            Ok(_a) => {
                (_a, true)
            }
            Err(_) => {
                (0.0, false)
            }
        }
        // a.parse().unwrap()
    }

    /// 任意数转浮点数
    pub(crate) fn convertToFloat(val: luaValue) -> (f64, bool) {
        match val {
            luaValue::F64(f) => return (f, true),
            luaValue::I64(i) => return (i as f64, true),
            luaValue::Str(s) => return Math::ParseFloat(s),
            _ => return (0.0, false)
        }
    }

    /// 任意值转整数
    pub(crate) fn covertToInteger(val: luaValue) -> (i64, bool) {
        match val {
            luaValue::I64(i) => (i, true),
            luaValue::F64(f) => Math::FloatToInteger(f),
            // Math::ParseInteger(s)
            luaValue::Str(s) => Math::_stringToInteger(s),
            _ => (0, false)
        }
    }

    /// 对于字符串, 先看看能不能直接解释为整数, 若不能， 再尝试将其解释为浮点数然后转换为整数
    fn _stringToInteger(s: String) -> (i64, bool) {
        let (i, ok) = Math::ParseInteger(s.clone());
        if ok {
            return (i, true);
        }

        let (f, ok) = Math::ParseFloat(s);
        if ok {
            return Math::FloatToInteger(f);
        }

        (0, false)
    }

    /// 整除
    fn IFloorDiv(a: i64, b: i64) -> i64 {
        if a > 0 && b > 0 || a < 0 && b < 0 || a%b == 0 {
            a/b
        } else {
            a/b-1
        }
    }

    /// 向下取整
    fn FFloorDiv(a: f64, b: f64) -> f64 {
        ((a/b) as i64) as f64
    }

    /// 取模
    fn IMod(a: i64, b: i64) -> i64 {
        a-Math::IFloorDiv(a, b)*b
    }

    fn FMod(a: f64, b: f64) -> f64 {
        a-(((a/b) as i64) as f64)*b
    }

    /// 左移 位运算
    fn ShiftLeft(a: i64, n: i64) -> i64 {
        if n >= 0 {
            a << n as u64
        } else {
            Math::ShiftRight(a, -n)
        }
    }

    fn ShiftRight(a: i64, n: i64) -> i64 {
        if n >= 0 {
            (a as u64 >> n as u64) as i64
        } else {
            Math::ShiftLeft(a, -n)
        }
    }
}
