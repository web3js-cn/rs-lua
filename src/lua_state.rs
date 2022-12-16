//! lua state 封装了整个解释器状态

use crate::chunk::ProtoType;
use crate::lua_stack::{luaStack, luaValue};
use crate::math::arith::{Arith, Operator, operators};
use crate::math::consts::{LUA_OPBNOT, LUA_OPUNM};
use crate::math::Math;

/// 解释器状态
#[derive(Debug)]
pub struct luaState {
    pub statck: luaStack,
    /// 保存函数原型
    pub(crate) proto: ProtoType,
    /// 程序计数器
    pub(crate) pc: i64
}

impl luaState {
    /// 创建 luaState 实例
    pub(crate) fn new(stackSize: i64, proto: ProtoType) -> luaState {
        luaState {
            statck: luaStack::newLuaStack(20),
            proto: proto,
            pc: 0
        }
    }
}

/// 基础栈操作方法定义在 luaState 里
impl luaState {
    /// 返回栈顶索引
    fn GetTop(&mut self) -> i64 {
        self.statck.top
    }

    /// 将索引转为绝对索引
    fn AbsIndex(&mut self, idx: i64) -> i64 {
        self.statck.absIndex(idx)
    }

    /// 检查栈的剩余空间
    pub(crate) fn CheckState(&mut self, n: i64) -> bool {
        self.statck.check(n);
        return true
    }

    /// 从栈顶弹出 n 个值
    pub(crate) fn Pop(&mut self, n: i64) {
        for i in 0..n {
            self.statck.pop();
        }
    }

    /// 将值从一个位置复制到另一个位置
    pub(crate) fn Copy(&mut self, fromIdx: i64, toIdx: i64) {
        let val = self.statck.get(fromIdx);
        self.statck.set(toIdx, val.unwrap());
    }

    /// 将指定索引出的值推入栈顶
    pub(crate) fn PushValue(&mut self, idx: i64) {
        let val = self.statck.get(idx);
        self.statck.push(val.unwrap());
    }

    /// 是 PushValue 的反操作, 将栈顶值弹出, 然后写入到指定位置
    pub(crate) fn Replace(&mut self, idx: i64) {
        let val = self.statck.pop();
        self.statck.set(idx, val);
    }

    /// 将栈顶值弹出, 然后插入到指定位置
    fn Insert(&mut self, idx: i64) {
        self.Rotate(idx, 1);
    }

    /// 删除指定索引处的值, 然后该值上面的值全部下移一个位置
    pub(crate) fn Remove(&mut self, idx: i64) {
        self.Rotate(idx, -1);
        self.Pop(1);
    }

    /// 将 [idx,top]区间内的值朝栈顶方向旋转n个位置, 如果 n 是负数, 那么实际效果就是朝栈顶方向旋转1个位置
    fn Rotate(&mut self, idx: i64, n: i64) {
        let t = self.statck.top-1;
        let p = self.statck.absIndex(idx)-1;
        let mut m;
        if n >= 0 {
            m = t-n;
        } else {
            m = p-n-1;
        }
        self.statck.reverse(p, m);
        self.statck.reverse(m+1, t);
        self.statck.reverse(p, t);
    }

    /// 将栈顶索引设置为指定值, 如果指定值小于top, 则相当于弹出操作, 如果大于top, 相当于推入多个nil
    pub(crate) fn SetTop(&mut self, idx: i64) {
        let newTop = self.statck.absIndex(idx);
        if newTop < 0 {
            panic!("stack underrflow");
        }

        let n = self.statck.top-newTop;
        if n > 0 {
            for i in 0..n {
                self.statck.pop();
            }
        } else if n < 0 {
            for _ in n..0 {
                self.statck.push(luaValue::NIL(None));
            }
        }
    }

    /// 将五种类型的值推入栈顶
    pub(crate) fn PushNil(&mut self) { self.statck.push(luaValue::NIL(None)); }
    pub(crate) fn PushBoolean(&mut self, b: bool) { self.statck.push(luaValue::BOOL(b)); }
    pub(crate) fn PushInteger(&mut self, n: i64) { self.statck.push(luaValue::I64(n)); }
    pub(crate) fn PushNumber(&mut self, n: f64) { self.statck.push(luaValue::F64(n)); }
    pub(crate) fn PushString(&mut self, s: String) { self.statck.push(luaValue::Str(s)); }
}

/// Access 系列方法基本只根据索引访问栈里存储的信息, 不会改变栈的状态
impl luaState {
    /// 将类型转为字符串表达
    fn TypeName(&mut self, tp: LuaType) -> String {
        match tp as i8 {
            LUA_TNONE => "no value".to_string(),
            LUA_TNIL => "nil".to_string(),
            LUA_TBOOLEAN => "boolean".to_string(),
            LUA_TNUMBER => "number".to_string(),
            LUA_TSTRING => "string".to_string(),
            LUA_TTABLE => "table".to_string(),
            LUA_TFUNCTION => "function".to_string(),
            LUA_TTHREAD => "thread".to_string(),
            _ => "userdata".to_string()
        }
    }

    /// 根据索引返回值的类型, 如果无效, 返回LUA_TNONE
    fn Type(&mut self, idx: i64) -> LuaType {
        if self.statck.isValid(idx) {
            let val = self.statck.get(idx).unwrap();
            match val {
                luaValue::NIL(_) => return LUA_TNIL as LuaType,
                luaValue::Str(_) => return LUA_TSTRING as LuaType,
                luaValue::F64(_) => return LUA_TNUMBER as LuaType,
                luaValue::I64(_) => return LUA_TNUMBER as LuaType,
                luaValue::BOOL(_) => return LUA_TBOOLEAN as LuaType,
                _ => return 0
            }
        }
        return LUA_TNONE as LuaType;
    }

    /// 判断指定索引处的值是否为指定类型
    fn IsNone(&mut self, idx: i64) -> bool {
        self.Type(idx) == LUA_TNONE as LuaType
    }
    fn IsNil(&mut self, idx: i64) -> bool {
        self.Type(idx) == LUA_TNIL as LuaType
    }
    fn IsNoneOrNil(&mut self, idx: i64) -> bool {
        self.Type(idx) <= LUA_TNIL as LuaType
    }
    fn IsBoolean(&mut self, idx: i64) -> bool {
        self.Type(idx) == LUA_TBOOLEAN as LuaType
    }
    pub(crate) fn IsString(&mut self, idx: i64) -> bool {
        let t = self.Type(idx);
        t == LUA_TSTRING as LuaType || t == LUA_TNUMBER as LuaType
    }
    fn IsNumber(&mut self, idx: i64) -> bool {
        true
    }
    fn IsInteger(&mut self, idx: i64) -> bool {
        true
    }
    pub(crate) fn ToBoolean(&mut self, idx: i64) -> bool {
        let val = self.statck.get(idx);
        luaState::covertToBoolean(val.unwrap())
    }

    fn covertToBoolean(val: luaValue) -> bool {
        match val {
            luaValue::NIL(_) => false,
            luaValue::BOOL(b) => b,
            _ => true
        }
    }

    /// 从指定索引处取出一个值, 不是数字类型的话就进行转换
    pub(crate) fn ToNumber(&mut self, idx: i64) -> f64 {
        let (n, _) = self.ToNumberX(idx);
        n
    }
    fn ToNumberX(&mut self, idx: i64) -> (f64, bool) {
        // fn T(a: luaValue) -> String {
        //     match a {
        //         NiL(_) => { return "nil".to_string(); },
        //         luaValue::BOOL(_) => { return "bool".to_string(); },
        //         luaValue::I64(_) => { return "i64".to_string(); },
        //         luaValue::F64(_) => { return "f64".to_string(); },
        //         luaValue::Str(_) => { return "String".to_string(); }
        //     }
        // }
        // let val = self.statck.get(idx).unwrap();
        // match val {
        //     luaValue::F64(f) => (f, true),
        //     luaValue::I64(i) => (i as f64, true),
        //     _ => (0 as f64, false)
        // }

        let val = self.statck.get(idx);
        // match val.unwrap() {
        //     luaValue::I64(i) => { (i as f64, true) },
        //     luaValue::F64(f) => { (f, true)},
        //     _ => (0.0, false)
        // }
        Math::convertToFloat(val.unwrap())
    }

    // 从指定索引处取一个整数值
    fn ToInteger(&mut self, idx: i64) -> i64 {
        self.ToIntegerX(idx).0
    }
    /// 从指定索引处取一个整数值, 暂时只进行类型判断
    fn ToIntegerX(&mut self, idx: i64) -> (i64, bool) {
        match self.statck.get(idx).unwrap() {
            luaValue::I64(i) => (i, true),
            _ => (0, true)
        }
    }

    fn ToStringX(&mut self, idx: i64) -> (String, bool) {
        let val = self.statck.get(idx).unwrap();
        match val {
            luaValue::Str(s) => (s, true),
            luaValue::I64(i) => {
                self.statck.set(idx, luaValue::Str(i.to_string()));
                (i.to_string(), true)
            },
            luaValue::F64(f) => {
                self.statck.set(idx, luaValue::Str(f.to_string()));
                (f.to_string(), true)
            },
            _ => ("".to_string(), false)
        }
    }
    pub(crate) fn ToString(&mut self, idx: i64) -> String {
        self.ToStringX(idx).0
    }
}

/// Lua 官方实现里为每种数据类型都定义了一个常量值
/// 无效索引对应 LUA_TNONE(invalid)
const LUA_TNONE: i8 = -1;
const LUA_TNIL: i8 = 0;
const LUA_TBOOLEAN: i8 = 1;
const LUA_TLIGHTUSERDATA: i8 = 2;
const LUA_TNUMBER: i8 = 3;
const LUA_TSTRING: i8 = 4;
const LUA_TTABLE: i8 = 5;
const LUA_TFUNCTION: i8 = 6;
const LUA_TUSERDATA: i8 = 7;
const LUA_TTHREAD: i8 = 8;

/// Lua值类型的别名
type LuaType = i64;
