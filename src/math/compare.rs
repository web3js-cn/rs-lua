//! 比较运算符

use crate::lua_stack::luaValue;
use crate::lua_state::luaState;

/// 新增的类型别名
pub type CompareOp = i64;

/// 核心是给 luaState 添加一些方法
impl luaState {
    /// 对索引处的两个值进行比较
    pub(crate) fn Compare(&mut self, idx1: i64, idx2: i64, op: CompareOp) -> bool {
        if !self.statck.isValid(idx1) || !self.statck.isValid(idx2) {
            return false;
        }

        let a = self.statck.get(idx1).unwrap();
        let b = self.statck.get(idx2).unwrap();
        // println!("letA={:?}; letB={:?}", a, b);
        match op as u8 {
            LUA_OPEQ  => { return luaState::_eq(a, b); },
            LUA_OPLT => { return luaState::_lt(a, b); },
            LUA_OPLE => { return luaState::_le(a, b); },
            _ => panic!("invalid compare op, 比较运算符错误")
        }
    }

    /// 判断两个值是否相等
    /// 只有当两个操作数的类型相等时才有可能返回true
    fn _eq(a: luaValue, b: luaValue) -> bool {
        match a {
            luaValue::NIL(_) => {
                // 没有实现 PartialEq<_> 因此只能写成这样
                if let luaValue::NIL(bb) = b {
                    return true;
                }
            }
            luaValue::BOOL(aa) => {
                if let luaValue::BOOL(bb) = b {
                    return aa == bb;
                }
                return false;
            }
            luaValue::I64(x) => {
                if let luaValue::I64(y) = b {
                    return x == y;
                }
                if let luaValue::F64(y) = b {
                    return x as f64 == y;
                }
                return false;
            }
            luaValue::F64(x) => {
                if let luaValue::F64(y) = b {
                    return x == y;
                }
                if let luaValue::I64(y) = b {
                    return x == y as f64;
                }
                return false;
            }
            luaValue::Str(x) => {
                if let luaValue::Str(y) = b {
                    return x == y;
                }
                return false;
            }
        }
        if let a = b {
            return true;
        } else {
            return false;
        }
    }

    /// 小于的操作 <
    /// 仅对数字和字符串有意义
    fn _lt(a: luaValue, b: luaValue) -> bool {
        match a {
            luaValue::I64(x) => {
                if let luaValue::I64(y) = b {
                    return x < y;
                }
                if let luaValue::F64(y) = b {
                    return (x as f64) < y;
                }
            },
            luaValue::F64(x) => {
                if let luaValue::I64(y) = b {
                    return x < (y as f64);
                }
                if let luaValue::F64(y) = b {
                    return x < y;
                }
            },
            luaValue::Str(x) => {
                if let luaValue::Str(y) = b {
                    return x < y;
                }
            },
            _ => panic!("bool, error")
        }
        panic!("comparison error, 比较运算符失败, _lt()")
    }

    /// 大于的操作
    /// 仅对数字和字符串有意义
    fn _le(a: luaValue, b: luaValue) -> bool {
        // println!("//////////////_le() a={:?}; b={:?}", a, b);
        match a {
            luaValue::I64(x) => {
                if let luaValue::I64(y) = b {
                    return x <= y;
                }
                if let luaValue::F64(y) = b {
                    return (x as f64) <= y;
                }
            },
            luaValue::F64(x) => {
                if let luaValue::I64(y) = b {
                    return x <= (y as f64);
                }
                if let luaValue::F64(y) = b {
                    return x <= y;
                }
            },
            luaValue::Str(x) => {
                if let luaValue::Str(y) = b {
                    return x < y;
                }
            },
            _ => panic!("bool, error")
        }
        panic!("comparison error, 比较运算符失败, _lt()")
    }
}


/// 判断大小
/// ==
pub const LUA_OPEQ: u8 = 0;
/// <
pub const LUA_OPLT: u8 = 1;
/// <=
pub const LUA_OPLE: u8 = 2;
