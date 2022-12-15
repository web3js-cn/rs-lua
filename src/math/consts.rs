//! lua api 为每个算数和位运算符都分配一个运算码, 但只给等于、小于、小于等于分配运算码

/// 算数和位运算符运算码
/// +
pub const LUA_OPADD: u8 = 0;
/// -
pub const LUA_OPSUB: u8 = 1;
/// *
pub const LUA_OPMUL: u8 = 2;
/// &
pub const LUA_OPMOD: u8 = 3;
/// ^
pub const LUA_OPPOW: u8 = 4;
/// //
pub const LUA_OPDIV: u8 = 5;
/// &
pub const LUA_OPBAND: u8 = 6;
/// |
pub const LUA_OPBOR: u8 = 7;
/// ~
pub const LUA_OPBXOR: u8 = 8;
/// <<
pub const LUA_OPSHL: u8 = 9;
/// >>
pub const LUA_OPSHR: u8 = 10;
/// - (unary minus)
pub const LUA_OPUNM: u8 = 11;
/// ~
pub const LUA_OPBNOT: u8 = 12;

/// 判断大小
/// ==
pub const LUA_OPEQ: u8 = 0;
/// <
pub const LUA_OPLT: u8 = 1;
/// <=
pub const LUA_OPLE: u8 = 2;
