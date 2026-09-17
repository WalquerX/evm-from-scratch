use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
    pub return_data: String,
    pub logs: Vec<serde_json::Value>,
    pub state: serde_json::Map<String, serde_json::Value>,
}

pub fn evm(code: &[u8]) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc = 0;

    while pc < code.len() {
        let opcode = code[pc];
        pc += 1;

        // TODO: implement the EVM here!

        if opcode == 0x5f /* PUSH0 */ {
            stack.push(U256::zero());
        } else if opcode == 0x60 /* PUSH1 1 */ {

            stack.push(U256::from_big_endian(&code[pc..pc + 1]));
            pc += 1;
        
        } else if opcode == 0x61 /* PUSH2 */ {
            stack.push(U256::from_big_endian(&code[pc..pc + 2]));
            pc += 2;
        } else if opcode == 0x62 /* PUSH3 */ {
            stack.push(U256::from_big_endian(&code[pc..pc + 3]));
            pc += 3;
        } else if opcode == 0x63 /* PUSH4 */ {
            stack.push(U256::from_big_endian(&code[pc..pc + 4]));
            pc += 4;
        } else if opcode == 0x64 /* PUSH5 */ {
            stack.push(U256::from_big_endian(&code[pc..pc + 5]));
            pc += 5;
        } else if opcode == 0x65 /* PUSH6 */ {
            stack.push(U256::from_big_endian(&code[pc..pc + 6]));
            pc += 6;
        } else if opcode == 0x66 /* PUSH7 */ {
            stack.push(U256::from_big_endian(&code[pc..pc + 7]));
            pc += 7;
        } else {}


    }

    EvmResult {
        stack,
        success: true,
        return_data: String::new(),
        logs: Vec::new(),
        state: serde_json::Map::new(),
    }
}
