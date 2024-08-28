pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    clock::Clock,
    sysvar::Sysvar
};

// declare and export the program's entrypoint
entrypoint!(process_instruction);
 
// program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    // log a message to the blockchain
    msg!("Hello, world!");

    
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;
    msg!("Current Timestamp: {}", current_timestamp);

    solana_program::program::set_return_data(b"Custom return value");
 
    // gracefully exit the program
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
