use anchor_lang::prelude::*;

use crate::constants;	pub use constants::*;
use crate::states;		pub use states::*;
use crate::errors;		pub use errors::BasicError;

pub fn call(ctx: Context<Initialize>) -> Result<()> {
	let storage: &mut Account<MyStorage> = &mut ctx.accounts.storage;
	storage.data = VALUE;

	let string = "ABCDEF";
	msg!("{}", string);

	let bytes: &[u8] = string.as_bytes();
	let len = bytes.len().min(STR_SIZE_TITLE);
	storage.title[..len].copy_from_slice(&bytes[..len]);
	msg!("{:?}", storage.title);

	// Program log: ABCDEF
    // Program log: [65, 66, 67, 68, 69, 70, 0, 0]

	Ok(())
}