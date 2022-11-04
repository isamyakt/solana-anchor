use anchor_lang::prelude::*;

declare_id!("GAXBtrt9LBtRjKL6oWxSe7J1tBncGq61CcjpV7juFew8");

#[program]
pub mod flipper {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let switch_account = &mut ctx.accounts.switch_account;
        switch_account.state = false;
        Ok(())
    }

    pub fn flip(ctx: Context<Flip>) -> Result<()> {
        let switch_account = &mut ctx.accounts.switch_account;
        if switch_account.state {
            switch_account.state = true;
        } else {
            switch_account.state = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub switch_account: Account<'info, SwitchAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Flip<'info> {
    #[account(mut)]
    pub switch_account: Account<'info, SwitchAccount>,
}
#[account]
pub struct SwitchAccount {
    pub state: bool,
}