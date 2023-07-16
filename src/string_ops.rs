
use strum_macros::EnumString;
use crate::string_ops::Operation::Invalid;

#[derive(Debug, EnumString, Eq, PartialEq)]
pub enum Operation {
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
    pub fn from_raw_str(operation: &str) -> Operation{
        let opcode: Result<Operation,_> = operation.parse();
        match opcode {
            Ok(opcode) => {
                opcode
            }
            Err(_) => {
                Invalid
            }
        }
    }

    pub fn from_string(operation: String) -> Operation {
        let opcode: Result<Operation,_> = operation.parse();
        match opcode {
            Ok(opcode) => {
                opcode
            }
            Err(_) => {
                Invalid
            }
        }
    }
}