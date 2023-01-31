use crate::*;

/// This file includes NFT related features but not required in the Nomicon Standards

// #[near_bindgen]
impl Contract {
    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        memo: Option<String>,
    ) -> Token {
        // 1. get lease condistion to infer token info
        let lease_condition = self
            .lease_map
            .get(&token_id)
            .expect("No matching lease for the given LEASE token id!");

        let owner_id = lease_condition.lender_id.clone();
        assert_eq!(
            &owner_id, &sender_id,
            "Only Lease token owner can transfer!"
        );
        assert_ne!(
            &owner_id, &receiver_id,
            "Token owner can not be the receiver!"
        );

        // 2. remove token_id from the old owner's record
        self.internal_remove_token_from_owner(&sender_id, &token_id);

        // 3. add token_id to the new owner's record
        self.internal_add_token_to_owner(&receiver_id, &token_id);

        // 4. update lease.lender to new owner, to reflect lender and token owner change
        let new_lease_condition = LeaseCondition {
            lender_id: receiver_id.clone(),
            ..lease_condition
        };
        self.lease_map.insert(&token_id, &new_lease_condition);

        // 5. if there was memo, log it
        if let Some(memo) = memo {
            env::log_str(&format!("Memo: {}", memo).to_string());
        }

        Token {
            token_id: token_id.clone(),
            owner_id: receiver_id.clone(),
            metadata: None,
        }
    }

    pub(crate) fn internal_remove_token_from_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        let mut token_ids_set = self
            .token_ids_per_owner
            .get(account_id)
            .expect("Token is not owned by the sender!");

        token_ids_set.remove(token_id);

        if token_ids_set.is_empty() {
            self.token_ids_per_owner.remove(account_id);
        } else {
            self.token_ids_per_owner.insert(account_id, &token_ids_set);
        }
    }

    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        let mut token_ids_set = self
            .token_ids_per_owner
            .get(&account_id)
            .unwrap_or_else(|| {
                // if the receiver doesn't have any tokens, create a new record
                UnorderedSet::new(
                    StorageKey::TokenIdsPerOwnerInner {
                        account_id_hash: utils::hash_account_id(&account_id),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });

        token_ids_set.insert(token_id);
        self.token_ids_per_owner.insert(account_id, &token_ids_set);
    }
}
