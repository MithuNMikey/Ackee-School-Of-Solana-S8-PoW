use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let tweet_reaction = &ctx.accounts.tweet_reaction;
    
    // Decrement the appropriate counter based on reaction type
    match tweet_reaction.reaction {
        ReactionType::Like => {
            require!(
                tweet.likes > 0,
                TwitterError::MinLikesReached
            );
            tweet.likes = tweet.likes.checked_sub(1).unwrap();
        }
        ReactionType::Dislike => {
            require!(
                tweet.dislikes > 0,
                TwitterError::MinDislikesReached
            );
            tweet.dislikes = tweet.dislikes.checked_sub(1).unwrap();
        }
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    
    #[account(
        mut,
        close = reaction_author,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref()
        ],
        bump = tweet_reaction.bump,
        has_one = reaction_author
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}
