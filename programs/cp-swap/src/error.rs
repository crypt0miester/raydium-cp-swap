/// Errors that may be returned by the TokenSwap program.
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Not approved")]
    NotApproved,
    /// The owner of the input isn't set to the program address generated by the
    /// program.
    #[msg("Input account owner is not the program address")]
    InvalidOwner,
    /// The input token account is empty.
    #[msg("Input token account empty")]
    EmptySupply,
    /// The input token is invalid for swap.
    #[msg("InvalidInput")]
    InvalidInput,
    /// Address of the provided pool token mint is incorrect
    #[msg("Address of the provided lp token mint is incorrect")]
    IncorrectLpMint,
    /// Exceeds desired slippage limit
    #[msg("Exceeds desired slippage limit")]
    ExceededSlippage,
    /// Given pool token amount results in zero trading tokens
    #[msg("Given pool token amount results in zero trading tokens")]
    ZeroTradingTokens,
    #[msg("Not support token_2022 mint extension")]
    NotSupportMint,
    #[msg("invaild vault")]
    InvalidVault,
    #[msg("Init lp amount is too less(Because 100 amount lp will be locked)")]
    InitLpAmountTooLess,
    #[msg("Sell result is none")]
    SellResultNone,
    #[msg("Buy result is none")]
    BuyResultNone,
}
