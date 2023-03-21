use near_sdk::PromiseOrValue;

use crate::externals::*;
use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ListingAcceptanceJson {
    listing_id: String,
}

/// The trait for receiving FT payment
pub trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

/**
 * This method will triger the acceptance of a listing.
 * 1. Borrower(Sender) calls `ft_transfer_call` on FT contract
 * 2. FT contract transfers `amount` tokens from Borrower to Marketplace(reciever)
 * 3. FT contract calls `ft_on_transfer` on Marketplace contract
 * 4. Marketplace contract makes XCC (nft_transfer_call) to transfer the leasing NFT to Core contract
 *    & transfer ft token to Core contract
 * 5. Marketplace contract resolves the promise returned from Core ands return Promise accordingly
*/
#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    /// Function that initiates the transaction of activating a listed lease.
    #[payable]
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // enforce cross contract call
        let ft_contract_id = env::predecessor_account_id();
        assert_ne!(
            ft_contract_id,
            env::current_account_id(),
            "ft_on_transfer should only be called via XCC"
        );

        let listing_acceptance_json: ListingAcceptanceJson =
            near_sdk::serde_json::from_str(&msg).expect("Invalid lease listing");

        let listing: Listing = self
            .listings
            .get(&listing_acceptance_json.listing_id)
            .unwrap();

        assert_eq!(
            ft_contract_id.clone(),
            listing.ft_contract_id,
            "Wrong FT contract id!"
        );
        assert_eq!(
            amount.0, listing.price.0,
            "Transferred amount doesn't match the asked rent!"
        );

        // Transfer both the to be rented NFT and the rent payment (FT) to the rental contract.
        // And the rental contract will active the lease.
        // When it returns successfully, remove the listing.
        // 1. Marketplace transfers the NFT to Core contract
        // 2. Marketplace transfers rent(FT) to Core contract
        // 3. Core contract create an active lease after receiving both NFT and FT
        //      & returns success Promise after lease activation
        // 4. Marketplace reolves the Promise return accordingly

        // msg to be passed in nft_transfer_call
        let msg_lease_json = json!({
            "contract_addr": listing.nft_contract_id.clone(),
            "token_id": listing.nft_token_id.clone(),
            "lender_id": listing.owner_id.clone(),
            "borrower_id": sender_id.clone(),
            "approval_id": listing.approval_id.clone(),
            "ft_contract_addr": listing.ft_contract_id.clone(),
            "start_ts_nano": listing.lease_start_ts_nano.clone(),
            "end_ts_nano": listing.lease_end_ts_nano.clone(),
            "price": listing.price.clone(),
            "listing_id": listing_acceptance_json.listing_id.clone(),
        })
        .to_string();

        // Transfer leasing nft to Core contract
        ext_nft::ext(listing.nft_contract_id.clone())
            .with_static_gas(Gas(10 * TGAS))
            .with_attached_deposit(1)
            .nft_transfer_call(
                self.rental_contract_id.clone(),   // receiver_id
                listing.nft_token_id.clone(),      // token_id
                msg_lease_json,                    // msg
                Some(listing.approval_id.clone()), //approval_id
                None,                              // memo
            )
            .then(
                // Trasnfer rent to Core contract, after resolving the returned promise
                ext_self::ext(env::current_account_id())
                    .with_static_gas(Gas(10 * TGAS))
                    .with_attached_deposit(1)
                    .transfer_rent_after_nft_transfer(
                        listing.ft_contract_id.clone(), // ft_contract_id
                        listing.price.clone(),          // amount
                        None,                           //memo
                    ),
            )
            .as_return()
            .into()
    }
}
