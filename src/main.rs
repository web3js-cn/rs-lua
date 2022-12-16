use crate::chunk::ProtoType;
use crate::lua_state::luaState;
use crate::math::consts::{LUA_OPADD, LUA_OPBNOT, LUA_OPEQ};
use crate::math::arith::ArithOp;
use crate::math::compare::CompareOp;
use crate::opcodes::{Instruction, OP_RETURN};

pub mod header;
pub mod chunk;
pub mod opcodes;
pub mod lua_stack;
pub mod lua_state;
pub mod math;
pub mod vm;

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
    // let mut ls = lua_state::luaState::new();
    // ls.PushBoolean(true); println!("{:?}", ls.statck);
    // ls.PushInteger(10); println!("{:?}", ls.statck);
    // ls.PushNil(); println!("{:?}", ls.statck);
    // ls.PushString("hello".to_string()); println!("{:?}", ls.statck);
    // ls.PushValue(-4); println!("{:?}", ls.statck);
    // ls.Replace(3); println!("{:?}", ls.statck);
    // ls.SetTop(6); println!("{:?}", ls.statck);
    // ls.Remove(-3); println!("{:?}", ls.statck);
    // ls.SetTop(-5); println!("{:?}", ls.statck);


    // 测试运算符
    // let mut ls = lua_state::luaState::new();
    // ls.PushInteger(1);
    // ls.PushString("2.0".to_string());
    // ls.PushString("3.0".to_string());
    // ls.PushNumber(4.0);
    // println!("{:?}", ls.statck);
    // ls.Arith(LUA_OPADD as ArithOp); println!("{:?}", ls.statck);
    // ls.Arith(LUA_OPBNOT as ArithOp); println!("{:?}", ls.statck);
    // ls.Len(2); println!("{:?}", ls.statck);
    // ls.Concat(3); println!("{:?}", ls.statck);
    // let b = ls.Compare(1, 2, LUA_OPEQ as CompareOp);
    // ls.PushBoolean(b); println!("{:?}", ls.statck);


    // 执行 lua 主函数
    let mut chunk = chunk::Chunk::new("src/luac.out");
    let mut mainFunc = chunk.mainFunc;
    luacMain(mainFunc);

    // let mut test = Instruction::new(65549);
    // println!("指令名={}", test.op_name());
}

/// 运行 lua 主函数
fn luacMain(proto: ProtoType) {
    let nRegs = proto.max_stack_size as i64;
    let mut ls = luaState::new(nRegs+8, proto);

    ls.SetTop(nRegs); println!("LuaStack 状态={:?}", ls.statck.slots);
    let mut i = 0;
    println!("指令表长度={}; 指令表={:?}", ls.proto.code.len(), ls.proto.code);
    loop {
        // println!("============= {} =================", i);
        let pc = ls.PC();
        let inst = ls.Fetch();
        let mut inst = Instruction::new(inst);
        // println!("pc={}", ls.PC());
        // print!("指令名={}", inst.op_name());
        if inst.opcode() != OP_RETURN as i64 {
            print!("[{}]\t指令={}\t", i+1, inst.op_name());
            print!("指令模式={}\t", inst.print_op_mode());
            print!("ABC操作数={:?}\t", inst.printOperands());
            ls.Execute(inst.code);
            println!("栈情况={:?}", ls.statck.slots);
            // println!("pc={}", ls.pc);
        } else {
            break;
        }
        i += 1;
    }
}
