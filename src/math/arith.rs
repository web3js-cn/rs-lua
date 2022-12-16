//! 执行算术和按位运算, 给 lua_state.rs 调用

use crate::lua_stack::luaValue;
use crate::lua_state::luaState;
use crate::math::consts::{LUA_OPBNOT, LUA_OPUNM};
use crate::math::Math;

/// 新增的类型别名
pub type ArithOp = i64;

/// Arith 实现
/// 通过调用 math/arith.rs
/// 这是本文件的精髓
impl luaState {
    pub fn Arith(&mut self, op: ArithOp) {
        // println!("arith ---");
        let mut a: luaValue;
        let mut b = self.statck.pop();

        if op != LUA_OPUNM as ArithOp && op != LUA_OPBNOT as ArithOp {
            a = self.statck.pop();
            // println!("{:?}", a);
        } else {
            // println!("b clone()");
            a = b.clone();
        }

        let mut operator = &mut operators[op as usize];
        let result = luaState::_arith(a.clone(), b.clone(), operator);
        if let luaValue::NIL(_) = result {
            panic!("arithmetic error");
        }
        self.statck.push(result);
    }

    fn _arith(a: luaValue, b: luaValue, op: &mut Operator) -> luaValue {
        if op.floatFunc == Arith::fnull {
            let (x, ok) = Math::covertToInteger(a);
            if ok {
                let (y, ok) = Math::covertToInteger(b);
                if ok {
                    return luaValue::I64((op.integerFunc)(x, y));
                }
            }
        } else {
            if op.integerFunc != Arith::inull {
                if let luaValue::I64(x) = a {
                    if let luaValue::I64(y) =  b {
                        return luaValue::I64((op.integerFunc)(x, y));
                    }
                }
            }
            let (x, ok) = Math::convertToFloat(a);
            // println!("{}", x);
            if ok {
                let (y, ok) = Math::convertToFloat(b);
                if ok {
                    // println!("{}", (op.floatFunc)(x, y));
                    return luaValue::F64((op.floatFunc)(x, y));
                }
            }
        }

        luaValue::NIL(None)
    }
}

/// 执行算术和按位运算
pub struct Arith;

impl Arith {
    pub fn iadd(a: i64, b: i64) -> i64 { a+b }
    pub fn fadd(a: f64, b: f64) -> f64 { a+b }
    pub fn isub(a: i64, b: i64) -> i64 { a-b }
    pub fn fsub(a: f64, b: f64) -> f64 { a-b }
    pub fn imul(a: i64, b: i64) -> i64 { a*b }
    pub fn fmul(a: f64, b: f64) -> f64 { a*b }
    pub fn imod(a: i64, b: i64) -> i64 { Math::IMod(a, b) }
    pub fn fmod(a: f64, b: f64) -> f64 { Math::FMod(a, b) }
    pub fn pow(a: f64, b: f64) -> f64 { a.powf(b) }
    pub fn div(a: f64, b: f64) -> f64 { a/b }
    pub fn iidiv(a: i64, b: i64) -> i64 { Math::IFloorDiv(a, b) }
    pub fn fidiv(a: f64, b: f64) -> f64 { Math::FFloorDiv(a, b) }
    pub fn band(a: i64, b: i64) -> i64 { a&b }
    pub fn bor(a: i64, b: i64) -> i64 { a|b }
    pub fn bxor(a: i64, b: i64) -> i64 { a^b }
    pub fn shl(a: i64, b: i64) -> i64 { Math::ShiftLeft(a, b) }
    pub fn shr(a: i64, b: i64) -> i64 { Math::ShiftRight(a, b) }
    pub fn inum(a: i64, _: i64) -> i64 { -a }
    pub fn fnum(a: f64, _: f64) -> f64 { -a }
    // 按位取反 这个有问题 但是目前解决不了
    pub fn bnot(a: i64, _: i64) -> i64 { !a }

    // 空指针辅助函数
    pub fn inull(_: i64, _: i64) -> i64 { 0 }
    pub fn fnull(_: f64, _: f64) -> f64 { 0.0 }
}

/// lua 算术运算符定义
#[derive(Debug)]
pub struct Operator {
    pub(crate) integerFunc: fn(i64, i64) -> i64,
    pub(crate) floatFunc: fn(f64, f64) -> f64,
}

/// 各种运算
pub const operators: [Operator; 14] = [
    Operator { integerFunc: Arith::iadd, floatFunc: Arith::fadd },
    Operator { integerFunc: Arith::isub, floatFunc: Arith::fsub },
    Operator { integerFunc: Arith::imul, floatFunc: Arith::fmul },
    Operator { integerFunc: Arith::imod, floatFunc: Arith::fmod },
    Operator { integerFunc: Arith::inull, floatFunc: Arith::pow },
    Operator { integerFunc: Arith::inull, floatFunc: Arith::div },
    Operator { integerFunc: Arith::iidiv, floatFunc: Arith::fidiv },
    Operator { integerFunc: Arith::band, floatFunc: Arith::fnull },
    Operator { integerFunc: Arith::bor, floatFunc: Arith::fnull },
    Operator { integerFunc: Arith::bxor, floatFunc: Arith::fnull },
    Operator { integerFunc: Arith::shl, floatFunc: Arith::fnull },
    Operator { integerFunc: Arith::shr, floatFunc: Arith::fnull },
    Operator { integerFunc: Arith::inum, floatFunc: Arith::fnum },
    Operator { integerFunc: Arith::bnot, floatFunc: Arith::fnull },
];
