use anchor_lang::prelude::*;

declare_id!("FrBfVZtRDKzcuZB7E4hqQgKuiHg5rWJZZu2vL4uk22XA");
/// We'll update our declare_id! later. Once you run `anchor build`, your Program ID can be found in /target/idl/[your_project_name].json
/// Moving on to the #[program] macro below, this is where we define our program.
/// Each method inside here defines an RPC request handler (aka instruction handler) which can be invoked by clients
#[program]
pub mod anchor_heroes {
    use super::*;
    /// The first parameter for every RPC handler is the Context struct. We define Initialize and LevelUp below at #[derive(Accounts)]
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let hero_account = &mut ctx.accounts.hero_account;
        hero_account.level = 1;
        Ok(())
    }
    /// All account validation logic is handled below at the #[account(...)] macros, letting us just focus on the business logic
    pub fn level_up(ctx: Context<LevelUp>) -> ProgramResult {
        let hero_account = &mut ctx.accounts.hero_account;
        hero_account.level += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user, space = 16+16)]
    pub hero_account: Account<'info, HeroAccount>,

    /// Marking accounts as mut persists any changes made upon exiting the program, allowing our votes to be recorded
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct LevelUp<'info> {

    #[account(mut)]
    pub hero_account: Account<'info, HeroAccount>
}

/// Here we define what our HeroAccount looks like
/// We define a struct with two public properties: crunchy and smooth
/// These properties will keep track of their respective votes as unsigned 64-bit integers
/// This VoteAccount will be passed inside each Transaction Instruction to record votes as they occur
#[account]
pub struct HeroAccount {
    pub level: u64,
}