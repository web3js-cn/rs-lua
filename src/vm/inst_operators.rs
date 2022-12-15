//! 运算符相关指令

use crate::{ArithOp, CompareOp, LUA_OPADD, LUA_OPBNOT, LUA_OPEQ};
use crate::lua_state::luaState;
use crate::math::consts::{LUA_OPBAND, LUA_OPBOR, LUA_OPBXOR, LUA_OPDIV, LUA_OPIDIV, LUA_OPLE, LUA_OPLT, LUA_OPMOD, LUA_OPMUL, LUA_OPPOW, LUA_OPSHL, LUA_OPSHR, LUA_OPSUB, LUA_OPUNM};
use crate::opcodes::Instruction;

impl luaState {
    /// 通过这个来实现二元算术运算符相关指令
    pub fn _binaryArith(&mut self, code: u32, op: ArithOp) {
        let (mut a, b, c) = Instruction::new(code).iABC();
        a += 1;

        self.GetRK(b);
        self.GetRK(c);
        self.Arith(op);
        self.Replace(a);
    }

    /// 通过这个来实现一元算术运算符相关指令
    pub fn _unaryArith(&mut self, code: u32, op: ArithOp) {
        let (mut a, mut b, _) = Instruction::new(code).iABC();
        a += 1;
        b += 1;

        self.PushValue(b);
        self.Arith(op);
        self.Replace(a);
    }

    /// 下面都是算术运算指令实现

    /// +
    pub(crate) fn add(&mut self, code: u32) {
        self._binaryArith(code, LUA_OPADD as ArithOp);
    }

    /// -
    pub fn sub(&mut self, code: u32) {
        self._binaryArith(code, LUA_OPSUB as ArithOp);
    }

    /// *
    pub fn mul(&mut self, code: u32) {
        self._binaryArith(code, LUA_OPMUL as ArithOp);
    }

    /// %
    pub fn MOD(&mut self, code: u32) {
        self._binaryArith(code, LUA_OPMOD as ArithOp);
    }

    /// ^
    pub fn pow(&mut self, code: u32) {
        self._binaryArith(code, LUA_OPPOW as ArithOp);
    }

    /// /
    pub fn div(&mut self, code: u32) {
        self._binaryArith(code, LUA_OPDIV as ArithOp);
    }

    /// //
    pub fn idiv(&mut self, code: u32) {
        // LUA_OPIDIV这个指令好像没定义,先搁置
        self._binaryArith(code, LUA_OPIDIV as ArithOp);
    }

    /// &
    pub fn band(&mut self, code: u32) {
        self._binaryArith(code, LUA_OPBAND as ArithOp);
    }

    /// |
    pub fn bor(&mut self, code: u32) {
        self._binaryArith(code, LUA_OPBOR as ArithOp);
    }

    /// ~
    pub fn bxor(&mut self, code: u32) {
        // LUA_OPXOR好像没定义
        self._binaryArith(code, LUA_OPBXOR as ArithOp);
    }

    /// <<
    pub fn shl(&mut self, code: u32) {
        self._binaryArith(code, LUA_OPSHL as ArithOp);
    }

    /// >>
    pub fn shr(&mut self, code: u32) {
        self._binaryArith(code, LUA_OPSHR as ArithOp);
    }

    /// -
    pub fn unm(&mut self, code: u32) {
        self._unaryArith(code, LUA_OPUNM as ArithOp);
    }

    /// ~
    pub fn bnot(&mut self, code: u32) {
        self._unaryArith(code, LUA_OPBNOT as ArithOp);
    }
}

/// 长度和拼接指令
impl luaState {
    /// LEN 指令对于 Lua 中的长度运算符
    pub fn _len(&mut self, code: u32) {
        let (mut a, mut b, _) = Instruction::new(code).iABC();
        a += 1;
        b += 1;

        self.Len(b);
        self.Replace(a);
    }

    /// CONCAT 指令将连续 n 个寄存器的值拼接, 将结果放入另一个寄存器
    pub fn concat(&mut self, code: u32) {
        let (mut a, mut b, mut c) = Instruction::new(code).iABC();
        a += 1;
        b += 1;
        c += 1;

        let n = c-b+1;
        self.CheckState(n);
        for i in b..=c {
            self.PushValue(i);
        }
        self.Concat(n);
        self.Replace(a);
    }
}

/// 比较指令
impl luaState {
    /// 通过这个来实现比较指令
    pub fn _compare(&mut self, code: u32, op: CompareOp) {
        let (a, b, c) = Instruction::new(code).iABC();

        self.GetRK(b);
        self.GetRK(c);

        if self.Compare(-2, -1, op) != (a != 0) {
            self.AddPC(1);
        }
        self.Pop(2);
    }

    /// 剩下三条比较指令

    /// ==
    pub fn eq(&mut self, code: u32) {
        self._compare(code, LUA_OPEQ as ArithOp);
    }

    /// <
    pub fn lt(&mut self, code: u32) {
        self._compare(code, LUA_OPLT as ArithOp);
    }

    /// <=
    pub fn le(&mut self, code: u32) {
        self._compare(code, LUA_OPLE as ArithOp);
    }
}

/// 逻辑运算指令
impl luaState {
    /// NOT 对应 Lua 的逻辑非运算符
    pub fn not(&mut self, code: u32) {
        let (mut a, mut b, c) = Instruction::new(code).iABC();
        a += 1;
        b += 1;

        let b = self.ToBoolean(b);
        self.PushBoolean(!b);
        self.Replace(a);
    }

    /// TESTSET(iABC) 判断寄存器B的值转换为布尔值后是否和操作数C表示的布尔值一致
    pub fn testSet(&mut self, code: u32) {
        let (mut a, mut b, c) = Instruction::new(code).iABC();
        a += 1;
        b += 1;

        if self.ToBoolean(b) == (c != 0) {
            self.Copy(b, a);
        } else {
            self.AddPC(1);
        }
    }

    /// TEST(iABC) 判断寄存器A的值转换为布尔值后是否和操作数C表示的布尔值一致
    pub fn test(&mut self, code: u32) {
        let (mut a, _, c) = Instruction::new(code).iABC();
        a += 1;

        if self.ToBoolean(a) != (c != 0) {
            self.AddPC(1);
        }
    }
}
