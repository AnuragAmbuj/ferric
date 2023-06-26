
use strum_macros::EnumString;
use crate::string_ops::Operation::Invalid;

#[derive(Debug, EnumString)]
enum Operation {
    #[strum(serialize = "GET")]
    Get,

    #[strum(serialize = "SET")]
    Set,

    #[strum(serialize = "EXISTS")]
    Exists,

    #[strum(serialize = "UNKNOWN OPERATION")]
    Invalid
}

impl Operation {
    fn find(operation: &str) -> Operation{
        let opcode: Result<Operation,_> = operation.parse();
        match opcode {
            Ok(opcode) => {
                return opcode;
            }
            Err(_) => {
                Invalid
            }
        }
    }
}