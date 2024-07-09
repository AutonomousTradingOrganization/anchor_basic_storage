use anchor_lang::prelude::*;

use crate::constants;	pub use constants::*;
use crate::states;		pub use states::*;
use crate::errors;		pub use errors::BasicError;

pub fn call(ctx: Context<Write>, value: u64) -> Result<()> {
	let storage: &mut Account<MyStorage> = &mut ctx.accounts.storage;
	storage.data = value;
	Ok(())
}