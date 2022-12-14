pub mod header;
pub mod chunk;
pub mod opcodes;
pub mod lua_stack;
pub mod lua_state;

fn main() {
    // 从 chunk 中读取头部 Header 并检查 check()
    // let header = header::Header::new("src/luac.out");
    // header.check();
    // println!("{}", header);


    // 读取 chunk 到 Chunk 结构体, 包含了头部、主函数 upvalues、主函数原型
    // let mut chunk = chunk::Chunk::new("src/luac.out");
    // println!("{:#?}", chunk);


    // let mut mainFunc = chunk::MainFunc::new("src/luac.out");
    // let code = mainFunc.readProto("@hello.lua".to_string()).code;
    // let mut idx = 0;
    // for i in code.iter() {
    //     let mut instruction = opcodes::Instruction::new(*i);
    //     println!("[{}] 指令名: {}; 指令模式: {};   参数: {:?};   B操作数类型: {}, C操作数类型: {}" ,
    //              idx, instruction.op_name(),
    //              instruction.print_op_mode(),
    //              instruction.printOperands(),
    //              instruction.b_c_mode_print().0,
    //              instruction.b_c_mode_print().1
    //     );
    //     idx += 1;
    // }


    // 测试简易 luastate 的各种栈操作
    let mut ls = lua_state::luaState::new();
    ls.PushBoolean(true); println!("{:?}", ls.statck);
    ls.PushInteger(10); println!("{:?}", ls.statck);
    ls.PushNil(); println!("{:?}", ls.statck);
    ls.PushString("hello".to_string()); println!("{:?}", ls.statck);
    ls.PushValue(-4); println!("{:?}", ls.statck);
    ls.Replace(3); println!("{:?}", ls.statck);
    ls.SetTop(6); println!("{:?}", ls.statck);
    ls.Remove(-3); println!("{:?}", ls.statck);
    ls.SetTop(-5); println!("{:?}", ls.statck);
}
