use crate::*;
use near_sdk::PromiseOrValue;

/// NFT contract interface for XCC
#[ext_contract(ext_nft)]
pub trait NonFungibleToken {
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        msg: String,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
}

/// FT contract interface for XCC
#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer_call(
        &mut self, 
        receiver_id: AccountId, 
        amount: U128, 
        memo: Option<String>,
        msg: String,
    );
}

/// Interface of this marketplace contract, for XCC by the contract itself.
#[ext_contract(ext_self)]
trait ExtSelf {
    fn transfer_rent_after_nft_transfer(
        &mut self,
        ft_contract_id: AccountId,
        amount: U128,
        memo: Option<String>,
        listing_id: ListingId,
    ) -> PromiseOrValue<U128>;
}
