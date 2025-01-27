use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;

declare_id!("HxtjikNnqQszksmGnwcMsC2pTEpU1CoGc6DFp9ZwFRL3");

#[program]
pub mod bindentity {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        initialize_handler(ctx, params)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
        update_config_handler(ctx, params)
    }

    pub fn create_provider(
        ctx: Context<CreateProvider>,
        params: CreateProviderParams,
    ) -> Result<()> {
        create_provider_handler(ctx, params)
    }

    pub fn create_validator(
        ctx: Context<CreateValidator>,
        params: CreateValidatorParams,
    ) -> Result<()> {
        create_validator_handler(ctx, params)
    }

    pub fn create_identity(
        ctx: Context<CreateIdentity>,
        params: CreateIdentityParams,
    ) -> Result<()> {
        create_identity_handler(ctx, params)
    }

    pub fn verify_provider(
        ctx: Context<VerifyProvider>,
        params: VerifyProviderParams,
    ) -> Result<()> {
        verify_provider_handler(ctx, params)
    }

    pub fn update_provider(
        ctx: Context<UpdateProvider>,
        params: UpdateProviderParams,
    ) -> Result<()> {
        update_provider_handler(ctx, params)
    }

    pub fn update_validator(
        ctx: Context<UpdateValidator>,
        params: UpdateValidatorParams,
    ) -> Result<()> {
        update_validator_handler(ctx, params)
    }

    pub fn void_identity(ctx: Context<VoidIdentity>, params: VoidIdentityParams) -> Result<()> {
        void_identity_handler(ctx, params)
    }
}

#[error_code]
pub enum CustomError {
    #[msg("Provider is unpublished")]
    ProviderUnpublished,

    #[msg("Validator is not allowed to create an identity")]
    ValidatorDisabled,

    #[msg("Hash of the ID does not match")]
    InvalidIdHash,

    #[msg("Signers are not authorized to void")]
    VoidUnauthorized,
}
