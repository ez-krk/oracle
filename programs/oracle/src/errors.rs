use anchor_lang::error_code;

#[error_code]
pub enum OracleError {
    #[msg("Operator Does Not Belong To This Oracle.")]
    OperatorDoesNotBelongToThisOracle,
    #[msg("Operator Already Added.")]
    OperatorAlreadyAdded,
}
