use crate::{instructions::*, state::*, stubs::*, errors::*};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_pack::Pack,
    pubkey::Pubkey,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = SecondOwnerInstruction::unpack(instruction_data)?;

        match instruction {
            SecondOwnerInstruction::Increase {} => {
                Self::process_increase(accounts)?;
            }
            SecondOwnerInstruction::ChangeOwnership {} => {
                Self::process_change_ownership(accounts, program_id)?;
            }
            SecondOwnerInstruction::ZeroOutAccount {} => {
                Self::process_zero_out_account(accounts)?;
            }
        }

        Ok(())
    }

    pub fn process_increase(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("process_increase ix...");
        let account_info_iter = &mut accounts.iter();
        let counter = next_account_info(account_info_iter)?;

        let mut counter_data = Counter::unpack_unchecked(&counter.try_borrow_data()?)?;

        increase::increase(&mut counter_data)?;

        Counter::pack(counter_data, &mut counter.try_borrow_mut_data()?)?;

        Ok(())
    }

    pub fn process_change_ownership(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        msg!("process_change_ownership ix...");
        let account_info_iter = &mut accounts.iter();
        let counter = next_account_info(account_info_iter)?;
        let new_owner = next_account_info(account_info_iter)?;

        if counter.owner != program_id {
            return Err(SecondOwnerError::WrongAccountOwner.into());
        }

        counter.assign(new_owner.key);

        Ok(())
    }

    pub fn process_zero_out_account(
        accounts: &[AccountInfo]
    ) -> ProgramResult {
        msg!("process_zero_out_account ix...");
        let account_info_iter = &mut accounts.iter();
        let counter = next_account_info(account_info_iter)?;

        let mut counter_data = Counter::unpack_unchecked(&counter.try_borrow_data()?)?;

        counter_data.number = 0;

        Counter::pack(counter_data, &mut counter.try_borrow_mut_data()?)?;

        Ok(())
    }
}
