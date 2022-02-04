use anchor_lang::prelude::*;

declare_id!("4njC5D9uNCm95t6xo2pPvMdfzvFByNKXZJ2htEVpzACv");

#[program]
pub mod fib {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let fib_account = &mut ctx.accounts.fibonacci;
        fib_account.prev_number = 0;
        fib_account.cur_number = 0;
        Ok(())
    }

    pub fn next_fibonacci(ctx: Context<NextFibonacci>) -> ProgramResult {
        let fib_account = &mut ctx.accounts.fibonacci;
        if fib_account.prev_number == 0 && fib_account.cur_number == 0 {
            fib_account.cur_number = 1;
            Ok(())
        } else {
            let result = fib_account.prev_number + fib_account.cur_number;
            fib_account.prev_number = fib_account.cur_number;
            fib_account.cur_number = result;
            Ok(()) 
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 128)]
    pub fibonacci: Account<'info, Fibonacci>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct NextFibonacci<'info> {
    #[account(mut)]
    pub fibonacci:Account<'info, Fibonacci>
}

#[account]
pub struct Fibonacci {
    pub prev_number: u64,
    pub cur_number: u64
}