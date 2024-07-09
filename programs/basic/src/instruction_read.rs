use anchor_lang::prelude::*;

use crate::constants;	pub use constants::*;
use crate::states;		pub use states::*;
use crate::errors;		pub use errors::BasicError;

pub fn call(ctx: Context<Read>) -> Result<u64> {
	let storage: &Account<MyStorage> = &ctx.accounts.storage;
	msg!(&storage.data.to_string());
	Ok(storage.data)
}