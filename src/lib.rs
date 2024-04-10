use solana_program::{
    program::invoke,
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey
};


entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {


    let mut final_ixs = vec![];
    let target_key = Pubkey::from([11,116,205,230,58,32,135,174,169,27,23,84,62,171,97,192,161,195,87,42,157,255,218,160,175,202,144,146,164,131,106,247]); // this is mineRHF5r6S7HyD9SppBfVMXMavDkJsxwGesEvxZr2A
    for ac in accounts.iter() {
        if (final_ixs.len()) == accounts.len() - 1 {
            break;
        }
        if ac.key == program_id {
            final_ixs.push (
                AccountMeta {
                    pubkey: target_key,
                    is_signer: ac.is_signer,
                    is_writable: ac.is_writable
                }
            )
        }
        else {
            final_ixs.push (
                AccountMeta {
                    pubkey: *ac.key,
                    is_signer: ac.is_signer,
                    is_writable: ac.is_writable
                }
            )
        }
    }

    invoke(
        &Instruction {
            program_id: target_key,
            accounts: final_ixs,
            data: instruction_data.to_vec(),
        },
        accounts,
    )
}
