use super::argument::Argument;
use parity_scale_codec::{Decode, Encode};

pub trait OpCode {
    type Args: Encode + Decode;
    type Res: Encode + Decode;
    const CODE: u32;

    fn handler(args: Self::Args) -> Self::Res;
}

pub struct OpPush;
pub struct OpAdd;
pub struct OpSub;
pub struct OpEql;
pub struct OpNql;

impl OpCode for OpPush {
    type Args = Argument;
    type Res = Argument;
    const CODE: u32 = 0;

    fn handler(args: Argument) -> Argument {
        args
    }
}

impl OpCode for OpAdd {
    type Args = (u64, u64);
    type Res = u64;
    const CODE: u32 = 1;

    fn handler(args: Self::Args) -> Self::Res {
        args.0 + args.1
    }
}

impl OpCode for OpSub {
    type Args = (u64, u64);
    type Res = u64;
    const CODE: u32 = 2;

    fn handler(args: Self::Args) -> Self::Res {
        args.0 - args.1
    }
}

impl OpCode for OpEql {
    type Args = (Vec<u8>, Vec<u8>);
    type Res = bool;
    const CODE: u32 = 3;

    fn handler(args: Self::Args) -> Self::Res {
        args.0 == args.1
    }
}

impl OpCode for OpNql {
    type Args = (Vec<u8>, Vec<u8>);
    type Res = bool;
    const CODE: u32 = 4;

    fn handler(args: Self::Args) -> Self::Res {
        args.0 != args.1
    }
}
