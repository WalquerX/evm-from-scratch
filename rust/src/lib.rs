use primitive_types::U256;
use std::ops::Div;

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
        match opcode {
            0x00 /* STOP  */ => break,
            0x01 /* ADD */   => {
                let first_value = stack.pop().unwrap();
                let second_value = stack.pop().unwrap();
                let (result, _) = first_value.overflowing_add(second_value);
                stack.push(result);
            },
            0x02 /* MUL */ => {
                let first_value = stack.pop().unwrap();
                let second_value = stack.pop().unwrap();
                let (result, _) = first_value.overflowing_mul(second_value);
                stack.push(result);
            },
            0x03 /* SUB */ => {
                let first_value = stack.pop().unwrap();
                let second_value = stack.pop().unwrap();
                let (result, _) = first_value.overflowing_sub(second_value);
                stack.push(result);
            },
            0x04 /* DIV */ => {
                let first_value = stack.pop().unwrap();
                let second_value = stack.pop().unwrap();
                let mut result = U256::zero();
                // TODO: fix this, it is redundant
                if second_value == U256::zero() {
                    result = U256::zero();
                } else {
                    result = first_value.div(second_value);
                }
                stack.push(result);
            },
            0x5f /* PUSH0 */ => stack.push(U256::zero()),
            0x06 /* MOD */ => {
                let first_value = stack.pop().unwrap();
                let second_value = stack.pop().unwrap();

                let mut result = U256::zero();
                // TODO: fix this, it is redundant
                if second_value == U256::zero() {
                    result = U256::zero();
                } else {
                    result = first_value % second_value;
                }
                stack.push(result);
            },



            0x50 /* POP */ => {stack.pop();},
            0x60..=0x7f /* PUSH1 1 .. PUSH32*/ => {
                let n = (opcode - 0x5f) as usize;

                // How many bytes are really available in the code.
                let available = (code.len() - pc).min(n);

                // A buffer of n zeros. vec! makes the length at runtime.
                let mut buffer = vec![0u8; n];

                // Copy the available bytes to the START of the buffer.
                // The missing bytes stay zero, at the end.
                buffer[..available].copy_from_slice(&code[pc..pc + available]);

                stack.push(U256::from_big_endian(&buffer));
                pc += n;
            },
            _ => {},
        }


    }
    stack.reverse();
    EvmResult {
        stack,
        success: true,
        return_data: String::new(),
        logs: Vec::new(),
        state: serde_json::Map::new(),
    }
}
