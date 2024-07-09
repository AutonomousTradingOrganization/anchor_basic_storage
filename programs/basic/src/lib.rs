use anchor_lang::prelude::*;

mod constants;	pub use constants::*;
mod states;     pub use states::*;

#[macro_use]
mod macros;

pub mod instructions;
use instructions::*;

mod errors;		//pub use errors::BasicError;


//declare_id!("DSX41g54wAid4HzHuGt71M7YiCsoqJ7NMBXa8G9cbsLf");
declare_id!("ymNCSzK5qLoa8NqspphBLbKKLEdbRLzvNsQKnquYuHq");


#[program]
pub mod basic {

	use super::*;

	pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
		initialize::call(ctx)
	}

	pub fn read(ctx: Context<Read>) -> Result<u64> {
		read::call(ctx)
	}

	pub fn write(ctx: Context<Write>, value: u64) -> Result<()> {
		write::call(ctx, value)
	}
}
