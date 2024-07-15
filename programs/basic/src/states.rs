
use anchor_lang::{
	prelude::*,
	/*solana_program::pubkey*/
};
use std::mem::size_of;

use crate::STR_SIZE_TITLE;

#[account]
pub struct MyStorage {
	pub data : u64,
	pub title: [u8; STR_SIZE_TITLE],
}

#[derive(Accounts)]
pub struct Initialize<'info> {

	#[account(
		init,
		payer = authority,
		space = 8 + size_of::<MyStorage>(),
		seeds = [],
		bump
	)]
	pub storage: Account<'info, MyStorage>,

	#[account(mut)]
	pub authority: Signer<'info>,

	pub system_program: Program<'info, System>,
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