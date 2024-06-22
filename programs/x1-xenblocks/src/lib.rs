use anchor_lang::prelude::*;

declare_id!("BjsjDcLy3qb2pgKxqjLRwMX6bKdcZLbJmqcRFCitVXUw");

#[program]
pub mod x1_xenblocks {
    use super::*;

    pub fn initialize(ctx: Context<InitXenBlocks>) -> Result<()> {
        require!(ctx.accounts.xen_blocks.block_id == 0, XenBlocksError::AlreadyInitialized);

        Ok(())
    }

    pub fn submit_block(ctx: Context<SubmitXenBlock>, id: u128, key: [u8; 32], block_type: u8, miner: String) -> Result<()> {
        require!(ctx.accounts.xen_block_info.key == key, XenBlocksError::AlreadySubmitted);
        // TODO: do we need to check linearity ???
        // TODO: check block id != 0 ???
        // acquire current slot
        let slot = Clock::get().unwrap().slot;
        // update latest block id
        ctx.accounts.xen_blocks.block_id = id;
        // init block info
        ctx.accounts.xen_block_info.key = key.clone();
        ctx.accounts.xen_block_info.block_type = block_type;
        // emit event
        emit!(NewBlock { id, block_type, miner, key, slot });

        Ok(())
    }

    pub fn vote_for_block(ctx: Context<VoteForBlock>, id: u128) -> Result<()> {
        // TODO: check block id != 0 ???
        // check if the vote has bot been yet registered
        require!(ctx.accounts.vote_info.slot == 0, XenBlocksError::AlreadyVoted);
        // check if the vote is not for oneself
        require!(ctx.accounts.xen_block_info.miner != ctx.accounts.voter.key.to_string(), XenBlocksError::CannotSelfVote);

        // acquire current slot
        let slot = Clock::get().unwrap().slot;
        // cast a vote
        ctx.accounts.xen_block_info.votes += 1;
        ctx.accounts.vote_info.key = ctx.accounts.xen_block_info.key;
        ctx.accounts.vote_info.slot = slot;
        // emit event
        emit!(NewVote {
            id,
            voter: ctx.accounts.voter.key.to_string(),
            key: ctx.accounts.xen_block_info.key,
            slot
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitXenBlocks<'info> {
    #[account(
        init_if_needed,
        seeds = [
            b"xen-blocks-state",
        ],
        payer = admin,
        space = 8 + XenBlocksState::INIT_SPACE,
        bump
    )]
    pub xen_blocks: Box<Account<'info, XenBlocksState>>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: u128, miner: String)]
pub struct SubmitXenBlock<'info> {
    #[account(
        init_if_needed,
        seeds = [
            b"xen-block",
            id.to_le_bytes().as_ref(),
            // miner.as_bytes()
        ],
        payer = admin,
        space = 8 + XenBlockInfo::INIT_SPACE,
        bump
    )]
    pub xen_block_info: Box<Account<'info, XenBlockInfo>>,
    #[account(mut)]
    pub xen_blocks: Box<Account<'info, XenBlocksState>>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: u128)]
pub struct VoteForBlock<'info> {
    #[account(mut)]
    pub xen_block_info: Box<Account<'info, XenBlockInfo>>,
    #[account(
        init_if_needed,
        seeds = [
            b"xen-block-vote",
            id.to_le_bytes().as_ref(),
            voter.key().as_ref()
        ],
        payer = voter,
        space = 8 + VoteInfo::INIT_SPACE,
        bump
    )]
    pub vote_info: Box<Account<'info, VoteInfo>>,
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace, Default)]
pub struct XenBlocksState {
    pub block_id: u128
}

#[account]
#[derive(InitSpace, Default)]
pub struct XenBlockInfo {
    pub key: [u8; 32],
    pub block_type: u8,
    #[max_len(44)]
    pub miner: String,
    pub votes: u32
}

#[account]
#[derive(InitSpace, Default)]
pub struct VoteInfo {
    pub key: [u8; 32],
    pub slot: u64,
}

#[event]
pub struct NewBlock {
    id: u128,
    miner: String,
    key: [u8; 32],
    block_type: u8,
    // TODO: do we need slot here ???
    slot: u64,
}

#[event]
pub struct NewVote {
    id: u128,
    voter: String,
    key: [u8; 32],
    // TODO: do we need slot here ???
    slot: u64,
}

#[error_code]
pub enum XenBlocksError {
    #[msg("XenBlocks have already been initialized")]
    AlreadyInitialized,
    #[msg("Block has been already submitted")]
    AlreadySubmitted,
    #[msg("Vote has been already cast")]
    AlreadyVoted,
    #[msg("Cannot vote for oneself")]
    CannotSelfVote
}
