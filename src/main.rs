pub mod header;
pub mod chunk;
pub mod opcodes;

fn main() {
    // 从 chunk 中读取头部 Header 并检查 check()
    // let header = header::Header::new("src/luac.out");
    // header.check();
    // println!("{}", header);


    // 读取 chunk 到 Chunk 结构体, 包含了头部、主函数 upvalues、主函数原型
    let mut chunk = chunk::Chunk::new("src/luac.out");
    // println!("{:#?}", chunk);


    let mut mainFunc = chunk::MainFunc::new("src/luac.out");
    let code = mainFunc.readProto("@hello.lua".to_string()).code;
    let mut idx = 0;
    for i in code.iter() {
        let mut instruction = opcodes::Instruction::new(*i);
        println!("[{}] 指令名: {}; 指令模式: {};   参数: {:?};   B操作数类型: {}, C操作数类型: {}" ,
                 idx, instruction.op_name(),
                 instruction.print_op_mode(),
                 instruction.printOperands(),
                 instruction.b_c_mode_print().0,
                 instruction.b_c_mode_print().1
        );
        idx += 1;
    }
}
