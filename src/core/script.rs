use parity_scale_codec::{Decode, Encode, Input};

struct Script {
    data: Vec<u8>,
}

enum ScriptError {
    UnknownOpCode(OpCode),
}

#[derive(Decode, Encode, PartialEq, Debug)]
struct OpCode {
    code: u32,
}

static OP_ADD: OpCode = OpCode { code: 1 };
static OP_SUB: OpCode = OpCode { code: 2 };
static OP_MUL: OpCode = OpCode { code: 3 };
static OP_DIV: OpCode = OpCode { code: 4 };
static OP_EQL: OpCode = OpCode { code: 5 };
static OP_NQL: OpCode = OpCode { code: 6 };

impl Script {
    pub fn evaluate(&self) -> Result<(), ScriptError> {
        let mut data = self.data.as_slice();

        let mut args_stack = Vec::new();

        // while not end of the stream
        while data.remaining_len() != Ok(Some(0)) {
            // try to decode argument
            if let Ok(arg) = Vec::<u8>::decode(&mut data) {
                args_stack.push(arg);
                continue;
            }

            match OpCode::decode(&mut data).unwrap() {
                code if code == OP_ADD => {}
                code if code == OP_SUB => {}
                code if code == OP_MUL => {}
                code if code == OP_DIV => {}
                code if code == OP_EQL => {}
                code if code == OP_NQL => {}
                code => return Err(ScriptError::UnknownOpCode(code)),
            }
        }

        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn script_evaluate_test() {
        let code1 = OpCode { code: 10 };
        let code2 = OpCode { code: 12 };
        let code3 = OpCode { code: 13 };

        let mut data = Vec::new();
        data.append(&mut code1.encode());
        data.append(&mut code2.encode());
        data.append(&mut code3.encode());

        let script = Script { data: data.clone() };

        assert!(script.evaluate().is_ok());
    }
}