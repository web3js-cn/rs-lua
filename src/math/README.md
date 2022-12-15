# Lua 运算符

+ mod.rs 为运算符的基础方法 为后续大类运算符作准备
+ arith.rs 执行算术和按位运算, 给 lua_state.rs 调用
+ compare.rs 比较运算符
+ concat.rs 从栈顶弹出 n 个值, 对这些值进行拼接, 然后把结果推入栈顶
+ consts.rs lua api 为每个算数和位运算符都分配一个运算码, 但只给等于、小于、小于等于分配运算码
+ len.rs 实现 Len() 获取长度 访问指定索引处的值, 取其长度, 然后推入栈顶
