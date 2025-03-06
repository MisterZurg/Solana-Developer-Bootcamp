#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub pool_id: u64,
    #[max_len(280)] // cause we dont know how long it could be
    pub description: String,
    pub pool_start: u64, // unix timestamp
    pub pool_end: u64,   // unix timestamp
    pub candidates_number: u64,
}

#[derive(Accounts)]
#[instruction(pool_id: u64)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init, // — creates and initializes account automatically
        payer = signer,
        space = 8 + Pool::INIT_SPACE, // reserved 8 bytes + smth
        seeds = [pool_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(16)] // mus use max_len on String
    name: String,
    votes: u64,
}

#[derive(Accounts)]
#[instruction(name: String, pool_id: u64)] // we should put params,
                                           // in same order as in fn initialize_candidate()
                                           // cause it pulls that way
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [pool_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init, // — creates and initializes account automatically
        payer = signer,
        space = 8 + Candidate::INIT_SPACE, // reserved 8 bytes + smth
        seeds = [pool_id.to_le_bytes().as_ref(), name.as_bytes()],
        bump,
    )]
    pub candidate: Account<'info, Candidate>,

    pub system_program: Program<'info, System>,
}

#[program]
pub mod votingdapp {
    use super::*;

    /// initialize_pool —
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        pool_id: u64,
        description: String,
        pool_start: u64,
        pool_end: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        pool.pool_id = pool_id;
        pool.description = description;
        pool.pool_start = pool_start;
        pool.pool_end = pool_end;

        pool.candidates_number = 0;

        Ok(())
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        name: String,
        _pool_id: u64,
    ) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.name = name;
        candidate.votes = 0;

        Ok(())
    }
}
