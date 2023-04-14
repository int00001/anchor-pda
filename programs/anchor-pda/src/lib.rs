use anchor_lang::prelude::*;

declare_id!("3vzZJat5tfRWHDrfpSMeeWU7ZCFhewZm264ELinxDKkK");

#[program]
pub mod anchor_pda {
    use super::*;

    pub fn post_review(
        ctx: Context<ReviewAccounts>,
        restaurant: String,
        review: String,
        rating: u8,
    ) -> Result<()> {
        let new_review = &mut ctx.accounts.review;
        new_review.reviewer = ctx.accounts.signer.key();
        new_review.restaurant = restaurant;
        new_review.review = review;
        new_review.rating = rating;
        msg!(
            "review for {} - {} stars",
            new_review.restaurant,
            new_review.rating
        );
        msg!("review: {}", new_review.review);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(restaurant: String, review: String)]
pub struct ReviewAccounts<'info> {
    // pass in review account
    #[account(
        init_if_needed, // create account if doesn't exist
        payer = signer, // payer for account rent
        space = 500,    // bytes for account
        seeds = [restaurant.as_bytes().as_ref(), signer.key().as_ref()], // pda derive
        bump, // pda derive
    )]
    pub review: Account<'info, Review>,
    // signer is mutable (writable - sol debit to fund new account)
    #[account(mut)]
    pub signer: Signer<'info>,
    // system program will get called to create the new review account
    pub system_program: Program<'info, System>,
}

// Review account definition
// account will hold these 4 pieces of data
#[account]
pub struct Review {
    pub reviewer: Pubkey,
    pub restaurant: String,
    pub review: String,
    pub rating: u8,
}
