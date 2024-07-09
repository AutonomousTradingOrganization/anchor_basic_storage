use anchor_lang::error_code;

#[error_code]
pub enum BasicError {

	#[msg("Incorrect size for proposal title.")]
	DefaultError,
	
}
