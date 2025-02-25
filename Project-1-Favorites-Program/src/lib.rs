use anchor_lang::prelude::*;

// Addres of programm;
declare_id!("9qUnx4W8RaipbC8UGoSZRGauk19ebEamuvKEkoRMaRpQ");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// #[program] — macro makes module solana program aka smart contract
// and makes set_favorites solana instruction handler
#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}", context.program_id);

        let user_public_key = context.accounts.user.key();
        msg!(
            "User {user_public_key}'s favorite number is {number}, 
                favorite color is {color}
                hobbies are {hobbies:?}"
        );

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    #[max_len(50)] // 50 bytes
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

// By convention we call struct of accounts same as the handler function itself
// set_favorites()
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed, // — create if not already exists
        payer = user, // who — signs the transaction
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()], // gives addres to this account on the blockchain, public key
        bump, // calc the seeds
    )]
    pub favorites: Account<'info, Favorites>,
    pub system_program: Program<'info, System>,
}
