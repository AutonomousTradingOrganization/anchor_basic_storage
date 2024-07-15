use anchor_lang::prelude::*;

use crate::constants;	pub use constants::*;
use crate::states;		pub use states::*;
use crate::errors;		pub use errors::BasicError;


pub fn call(ctx: Context<Initialize>) -> Result<()> {
	let storage: &mut Account<MyStorage> = &mut ctx.accounts.storage;
	storage.data = VALUE;

	let string = "ABCDEF";
	msg!("{}", string);

	string_to_u8!(string, storage.title);
	msg!("{:?}", storage.title);

	// LOGS OUTPUT:
	// Program log: ABCDEF
    // Program log: [65, 66, 67, 68, 69, 70, 0, 0]

	Ok(())
}