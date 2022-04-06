use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use metaplex_token_metadata::{state::Metadata, ID as MetadataID};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_verify {
    use std::str::FromStr;

    use super::*;

    pub fn verify_nft(ctx: Context<VerifyNFT>) -> Result<()> {
        let nft_token_account = &ctx.accounts.nft_token_account;
        let user = &ctx.accounts.user;
        let nft_mint_account = &ctx.accounts.nft_mint;

        assert_eq!(nft_token_account.owner, user.key());
        assert_eq!(nft_token_account.mint, nft_mint_account.key());
        assert_eq!(nft_token_account.amount, 1);

        let nft_metadata_account = &ctx.accounts.nft_metadata_account;
        let nft_mint_account_pubkey = ctx.accounts.nft_mint.key();
        let metadata_seed = &[
            "metadata".as_bytes(),
            ctx.accounts.token_metadata_program.key.as_ref(),
            nft_mint_account_pubkey.as_ref(),
        ];

        let (metadata_derived_key, _bump_seed) =
            Pubkey::find_program_address(metadata_seed, ctx.accounts.token_metadata_program.key);

        assert_eq!(metadata_derived_key, nft_metadata_account.key());

        // if ctx.accounts.nft_metadata_account.data_is_empty() {
        //     return;
        // }
        let metadata_full_account =
            &mut Metadata::from_account_info(&ctx.accounts.nft_metadata_account)?;
        let full_metadata_clone = metadata_full_account.clone();
        let expected_creator =
            Pubkey::from_str("123412413241324123413243124134134431441341341413241341241").unwrap();
        assert_eq!(
            full_metadata_clone.data.creators.as_ref().unwrap()[0].address,
            expected_creator
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyNFT<'info> {
    // The owner of the nft
    pub user: Signer<'info>,
    // The mint account of the NFT
    pub nft_mint: Account<'info, Mint>,
    // The token account ie. account that the user uses to hold the NFT
    pub nft_token_account: Account<'info, TokenAccount>,
    // The metadata account of the nft
    pub nft_metadata_account: AccountInfo<'info>,

    #[account(address = MetadataID)]
    pub token_metadata_program: AccountInfo<'info>,
}
