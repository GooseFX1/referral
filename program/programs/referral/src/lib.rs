mod events;
mod instructions;
pub use instructions::{
    InitializeProjectParams, UpdateProjectParams, TransferProjectParams, UpdateReferralAccountParams, 
    WithdrawFromProjectParams, InitializeReferralAccountParams, InitializeReferralAccountWithNameParams,
    TransferReferralAccountParams, 
};

use anchor_lang::prelude::*;
use instructions::*;

#[cfg(all(not(feature = "devnet"), not(feature = "mainnet")))]
declare_id!("GMRpg29rcyvoYS5XnzXy8mC1qV58xRB5zWkSVLBsuhc3");

#[cfg(feature = "devnet")]
declare_id!("GMRpg29rcyvoYS5XnzXy8mC1qV58xRB5zWkSVLBsuhc3");

#[cfg(feature = "mainnet")]
declare_id!("GMRpg29rcyvoYS5XnzXy8mC1qV58xRB5zWkSVLBsuhc3");

pub const PROJECT_SEED: &[u8] = b"project";
pub const PROJECT_AUTHORITY_SEED: &[u8] = b"project_authority";
pub const REFERRAL_SEED: &[u8] = b"referral";
pub const REFERRAL_ATA_SEED: &[u8] = b"referral_ata";
pub const AUTHORITY_SEED: &[u8] = b"authority";

#[program]
pub mod referral {
    use super::*;

    // Project account instructions.

    pub fn initialize_project(
        ctx: Context<InitializeProject>,
        params: InitializeProjectParams,
    ) -> Result<()> {
        instructions::initialize_project(ctx, params)
    }

    pub fn update_project(ctx: Context<UpdateProject>, params: UpdateProjectParams) -> Result<()> {
        instructions::update_project(ctx, params)
    }

    pub fn transfer_project(
        ctx: Context<TransferProject>,
        params: TransferProjectParams,
    ) -> Result<()> {
        instructions::transfer_project(ctx, params)
    }

    pub fn update_referral_account(
        ctx: Context<UpdateReferralAccount>,
        params: UpdateReferralAccountParams,
    ) -> Result<()> {
        instructions::update_referral_account(ctx, params)
    }

    pub fn withdraw_from_project(
        ctx: Context<WithdrawFromProject>,
        params: WithdrawFromProjectParams,
    ) -> Result<()> {
        instructions::withdraw_from_project(ctx, params)
    }

    pub fn create_admin_token_account(ctx: Context<CreateAdminTokenAccount>) -> Result<()> {
        instructions::create_admin_token_account(ctx)
    }

    // Referral account instructions.

    pub fn initialize_referral_account(
        ctx: Context<InitializeReferralAccount>,
        params: InitializeReferralAccountParams,
    ) -> Result<()> {
        instructions::initialize_referral_account(ctx, params)
    }

    pub fn initialize_referral_account_with_name(
        ctx: Context<InitializeReferralAccountWithName>,
        params: InitializeReferralAccountWithNameParams,
    ) -> Result<()> {
        instructions::initialize_referral_account_with_name(ctx, params)
    }

    pub fn transfer_referral_account(
        ctx: Context<TransferReferralAccount>,
        params: TransferReferralAccountParams,
    ) -> Result<()> {
        instructions::transfer_referral_account(ctx, params)
    }

    pub fn initialize_referral_token_account(
        ctx: Context<InitializeReferralTokenAccount>,
    ) -> Result<()> {
        instructions::initialize_referral_token_account(ctx)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        instructions::claim(ctx)
    }

    pub fn close_referral_token_account(ctx: Context<CloseReferralTokenAccount>) -> Result<()> {
        instructions::close_referral_token_account(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct Project {
    pub base: Pubkey,
    pub admin: Pubkey,
    pub name: String,
    pub default_share_bps: u16,
}

impl Project {
    const LEN: usize = 8 + 32 + 32 + MAX_PROJECT_NAME_LENGTH + 12;
}

#[account]
pub struct ReferralAccount {
    pub partner: Pubkey,
    pub project: Pubkey,
    pub share_bps: u16,
    pub name: Option<String>,
}

impl ReferralAccount {
    const LEN: usize = 8 + 32 + 32 + MAX_REFERRAL_ACCOUNT_NAME_LENGTH + 2;
}

#[error_code]
pub enum ProgramErrorCode {
    InvalidCalculation,
    InvalidSharePercentage,
    NameTooLong,
}

const MAX_PROJECT_NAME_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_REFERRAL_ACCOUNT_NAME_LENGTH: usize = 50 * 4; // 50 chars max.
