use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, MintTo, Transfer};
use std::str::FromStr;


declare_id!("8i4YZYcxNpEZXHDYkhP2Ewv62jZVxvR193H8yb7sxqu1");

#[program]
pub mod propulsion {
    use super::*;
    
    pub fn create_project(
        ctx: Context<CreateProject>,
        title: String,
        description: String,
    ) -> Result<()> {
        let project: &mut Account<ProjectData> = &mut ctx.accounts.project_data;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        // validate and save the data
        if title.chars().count() > 50 {
            return Err(ErrorCode::TitleTooLong.into());
        }

        if title.chars().count() > 280 {
            return Err(ErrorCode::DescriptionTooLong.into());
        }

        project.author = *author.key;
        project.timestamp = clock.unix_timestamp;
        project.title = title;
        project.description = description;
        project.members.push(*author.key);
        project.shouldList = true;

        Ok(())
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        start_timestamp: String,
        end_timestamp: String,
        min_votes: u32,
        max_votes: u32,
        only_single_vote: bool, 
        vote_options: Vec<String>, // max of 5
        target_data: Vec<TargetData>,
    ) -> Result<()> {
        // we will be mutating this account
        let proposal: &mut Account<ProposalData> = &mut ctx.accounts.proposal_data;
        let project_data: &Account<ProjectData> = &ctx.accounts.project_data;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();
        // validate and save the data
        
        // check if author is a memeber in the project provided
        if !project_data.members.iter().any(|v| v == &*author.key) {
            return Err(ErrorCode::ProjectPermmissionError.into());
        }   

        // check if vote_options is less than 2 options
        if vote_options.len() < 2 {
            return Err(ErrorCode::VoteOptionLess.into());
        }

        if title.chars().count() > 50 {
            return Err(ErrorCode::TitleTooLong.into());
        }
        
        if description.chars().count() > 280 {
            return Err(ErrorCode::TitleTooLong.into());
        }
        
        if vote_options.len() > 5 {
            return Err(ErrorCode::VoteOptionsTooMuch.into());
        }

        // project: String,
        proposal.project = project_data.to_account_info().key();
        // author: Pubkey,
        proposal.author = *author.key;
        // timestamp: i64,
        proposal.timestamp = clock.unix_timestamp;
        // title: String,
        proposal.title = title;
        // description: String,
        proposal.description = description;
        // start_timestamp: i64,
        proposal.start_timestamp = i64::from_str(&start_timestamp).unwrap();
        // end_timestamp: i64,
        proposal.end_timestamp = i64::from_str(&end_timestamp).unwrap(); 
        // min_votes: i64,
        proposal.min_votes = min_votes;
        // max_votes: i64,
        proposal.max_votes = max_votes;
        // only_single_vote: bool, 
        proposal.only_single_vote = only_single_vote;
        // vote_options: Vec<String>, // max of 5
        proposal.vote_options = vote_options;
        
        // msg!("{:?}", target_data[0].choice_id);
        // do some validation on the target data to 
        // make sure that the length and types are okey
        for target in &target_data {
            match target.program_id {
                0 => println!("Do nothing..."),
                1 => {
                    // Add member...
                    if target.args.len() != 2 {
                        return Err(ErrorCode::LessTargetData.into());
                    }
                },
                2 => {
                    // Remove member...
                    if target.args.len() != 2 {
                        return Err(ErrorCode::LessTargetData.into());
                    }
                },
                3 => {
                    // Change settings...
                },
                4 => {
                    // Burning tokens...
                },
                5 => {
                    // Trasnfering funds...
                },
                6 => {
                    // Minting tokens...
                },
                _ => {
                    // Error: No expresion value provided!
                }
            }
        }
        
        // target_data: Vec<String>
        proposal.target_data = target_data;

        Ok(())
    }

    pub fn trigger_proposal(
        ctx: Context<TriggerProposal>,
    ) -> Result<()> {
        let project_data = &ctx.accounts.project_data;
        let proposal_data = &ctx.accounts.proposal_data;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        // check if author is a memeber in the project provided
        if !project_data.members.iter().any(|v| v == &*author.key) {
            return Err(ErrorCode::ProjectPermmissionError.into());
        }   

        // check if the author is the author of the proposal provided
        if proposal_data.author != author.key() {
            return Err(ErrorCode::ProposalPermmissionError.into());
        }

        // validate and trigger the 
        for target in &proposal_data.target_data {
            match target.program_id {
                0 => println!("Do nothing..."),
                1 => {
                    // Adding member...
                    // validate and save the data
                    let project_data: &mut Account<ProjectData> = &mut ctx.accounts.project_data;
                    let public_key = Pubkey::from_str(&target.args[0]).unwrap();
                    project_data.members.push(public_key.key());
                },
                2 => {
                    // Removing member...
                    // First we transer all the token this member holds
                    // then we remove the member from the listing
                    let project_data: &mut Account<ProjectData> = &mut ctx.accounts.project_data;
                    let public_key = Pubkey::from_str(&target.args[0]).unwrap();
                    for i in 0..project_data.members.len() {
                        if project_data.members[i] == public_key.key() {
                            project_data.members.remove(i);
                        }
                    }
                },
                3 => {
                    // Minting tokens...
                },
                4 => {
                    // Burning tokens...
                },
                5 => {
                    // Trasnfering funds...
                },
                _ => {
                    // Error: No expresion value provided!
                }
            }
        }

        Ok(())
    }

    pub fn cast_vote(
        ctx: Context<CastVote>,
        vote: Vec<u32>,
        note: String,
    ) -> Result<()> {
        let vote_data: &mut Account<VoteData> = &mut ctx.accounts.vote_data;
        let project_data = &ctx.accounts.project_data;
        let proposal_data = &ctx.accounts.proposal_data;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap(); 

        // validate and save the data

        // check if author is a memeber in the project provided
        if !project_data.members.iter().any(|v| v == &*author.key) {
            return Err(ErrorCode::ProjectPermmissionError.into());
        }   

        if proposal_data.end_timestamp > clock.unix_timestamp {
            return Err(ErrorCode::VotingTimeIsOver.into());
        }
        
        if note.chars().count() > 280 {
            return Err(ErrorCode::TitleTooLong.into());
        }
        // proposal: String, // The proposal that is being voted
        vote_data.proposal = proposal_data.to_account_info().key();
        // author: Pubkey,
        vote_data.author = *author.key;
        // timestamp: i64,
        vote_data.timestamp = clock.unix_timestamp;
        // vote: Vec<u8>,
        vote_data.vote = vote;
        // note: String,
        vote_data.note = note;

        Ok(())
    }
    
    pub fn mint_token(ctx: Context<MintToken>,) -> Result<()> {
        // Create the MintTo struct for our context
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContext we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute anchor's helper function to mint tokens
        token::mint_to(cpi_ctx, 10)?;
        
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()> {
        // Create the Transfer struct for our context
        let transfer_instruction = Transfer{
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.from_authority.to_account_info(),
        };
         
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the Context for our Transfer request
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        // Execute anchor's helper function to transfer tokens
        anchor_spl::token::transfer(cpi_ctx, 5)?;
 
        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(init, payer = author, space = 1000)]
    pub project_data: Account<'info, ProjectData>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProjectData {
    pub author: Pubkey,
    pub timestamp: i64,
    pub title: String,
    pub description: String,
    pub token:  String,
    pub members: Vec<Pubkey>,
    pub shouldList: bool,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = author, space = 1000)]
    pub proposal_data: Account<'info, ProposalData>,
    pub project_data: Account<'info, ProjectData>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TriggerProposal<'info> {
    pub proposal_data: Account<'info, ProposalData>,
    pub project_data: Account<'info, ProjectData>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[account]
pub struct ProposalData {
    pub project: Pubkey,
    pub author: Pubkey,
    pub timestamp: i64,
    pub title: String,
    pub description: String,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub min_votes: u32,
    pub max_votes: u32,
    pub only_single_vote: bool, 
    pub vote_options: Vec<String>, // max of 5
    pub target_data: Vec<TargetData>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(init, payer = author, space = 1000)]
    pub vote_data: Account<'info, VoteData>,
    pub proposal_data: Account<'info, ProposalData>,
    pub project_data: Account<'info, ProjectData>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VoteData {
    pub proposal: Pubkey, // The proposal that is being voted
    pub author: Pubkey,
    pub timestamp: i64,
    pub vote: Vec<u32>,
    pub note: String,
}

// tokens
#[derive(Accounts)]
pub struct MintToken<'info> {
    /// CHECK: This is the token that we want to mint
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the token account that we want to mint tokens to
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: the authority of the mint account
    #[account(mut)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: The associated token account that we are transferring the token from
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: The associated token account that we are transferring the token to
    #[account(mut)]
    pub to: AccountInfo<'info>,
    // the authority of the from account 
    pub from_authority: Signer<'info>,
}

#[derive(Debug)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TargetData {
    choice_id: u32,
    program_id: u32,
    args: Vec<String>,
}

// error
#[error_code]
pub enum ErrorCode {
    #[msg("The provided title should be 50 characters long maximum.")]
    TitleTooLong,
    #[msg("The provided description should be 280 characters long maximum.")]
    DescriptionTooLong,
    #[msg("The provided vote options should be 5 options maximum.")]
    VoteOptionsTooMuch,
    #[msg("You do not have any permision to this project.")]
    ProjectPermmissionError,
    #[msg("The provided vote options should be 2 options minimum.")]
    VoteOptionLess,
    #[msg("Voting was limited for a certian amount of time.")]
    VotingTimeIsOver,
    #[msg("The provided public key exist as a member.")]
    MemberAlreadyExist,
    #[msg("The provided public key doesn't exist as a member.")]
    MemberDoesNotExist,
    #[msg("The proposal can only be triggered by the author of the proposal.")]
    ProposalPermmissionError,
    #[msg("The provided target data is not enough")]
    LessTargetData,
}
