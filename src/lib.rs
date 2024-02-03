// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

// declare and export the program's entrypoint
entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = PayloadIx::unpack(instruction_data)?;

    match ix {
        PayloadIx::Send {
            flag,
            msg,
            num64,
            num32,
        } => {
            msg!("payload.flag: {}", flag);
            msg!("payload.msg: {}", msg);
            msg!("payload.num64: {}", num64);
            msg!("payload.num32: {}", num32);
            // and enum
            // and arr
            // and nested struct with arr and enum
        }
    }

    Ok(())
}

#[derive(BorshDeserialize, Debug)]
struct Payload {
    flag: bool,
    msg: String,
    num64: u64,
    num32: u32,
}

enum PayloadIx {
    Send {
        flag: bool,
        msg: String,
        num64: u64,
        num32: u32,
    },
}

impl PayloadIx {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Take the first byte as the variant to
        // determine which instruction to execute
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        // Use the temporary payload struct to deserialize
        let payload = borsh::from_slice::<Payload>(&rest).unwrap();
        // Match the variant to determine which data struct is expected by
        // the function and return the TestStruct or an error
        Ok(match variant {
            0 => PayloadIx::Send {
                flag: payload.flag,
                msg: payload.msg,
                num64: payload.num64,
                num32: payload.num32,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
