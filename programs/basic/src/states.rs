
use anchor_lang::{
	prelude::*,
	/*solana_program::pubkey*/
};

#[derive(Accounts)]
pub struct Initialize<'info> {

	#[account(
		init,
		payer = authority,
		space = 8 + 8,
		seeds = [],
		bump
	)]
	pub storage: Account<'info, MyStorage>,

	#[account(mut)]
	pub authority: Signer<'info>,

	pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
	pub data: u64,
}


#[derive(Accounts)]
pub struct Read<'info> {

	#[account(mut)]
	pub storage: Account<'info, MyStorage>,
}


#[derive(Accounts)]
pub struct Write<'info> {

    #[account(mut)]
    pub storage: Account<'info, MyStorage>,
}