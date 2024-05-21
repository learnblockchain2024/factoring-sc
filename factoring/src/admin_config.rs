multiversx_sc::imports!();

use crate::errors::*;

#[multiversx_sc::module]
pub trait AdminConfigModule :
    super::events::EventsModule
    + crate::stable_farming::StableFarmingModule
 {

    #[payable("*")]
    #[endpoint(addFunds)]
    fn add_funds(&self) {
        self.require_caller_is_admin();
        self.sc_add_funds_event();
    }

    #[endpoint(mintWithUnusedLiquidity)]
    fn mint_with_unused_liquidity(&self){
        self.require_caller_is_admin();

        self.mint();
    }

    #[endpoint(enterMarketWithUnusedLiquidity)]
    fn enter_market_with_unused_liquidity(&self){
        self.require_caller_is_admin();

        self.enter_market();
    }

    #[endpoint(exitMarketFarm)]
    fn exit_market_farm(&self){
        self.require_caller_is_admin();

        self.exit_market();
    }

    #[endpoint(withdrawLiquidity)]
    fn withdraw_liquidity(&self){
        self.require_caller_is_admin();

        self.redeem_liquidity();
    }

    #[endpoint(claimFarmingRewards)]
    fn claim_farming_rewards(&self){
        self.require_caller_is_admin();

        self.claim_rewards();
    }

    #[endpoint(addUserToAdminList)]
    fn add_user_to_admin_list(&self, address: ManagedAddress) {
        self.require_caller_is_admin();
        self.admin_whitelist().add(&address);
        self.sc_add_admin_event(address);
    }

    #[endpoint(removeUserFromAdminList)]
    fn remove_user_from_admin_list(&self, address: ManagedAddress) {
        self.require_caller_is_admin();
        self.admin_whitelist().remove(&address);
    }

    #[endpoint(addEuriborRate)]
    fn add_euribor_rate(&self, timestamp: u64, rate: u8) {
        self.require_caller_is_admin();

        let (_rate, old_timestamp) = self.euribor_rate().get();

        require!(timestamp > old_timestamp, TIMESTAMP_MUST_BE_HIGHER);

        self.euribor_rate().set((rate, timestamp));
    }

    #[endpoint(addAllowedToken)]
    fn add_allowed_tokens(&self, token: TokenIdentifier) {
        self.require_caller_is_admin();
        require!(token.is_valid_esdt_identifier(), INVALID_TOKEN_ID);

        self.allowed_tokens().add(&token);
    }

    #[endpoint(removeAllowedToken)]
    fn remove_allowed_tokens(&self, token: TokenIdentifier) {
        self.require_caller_is_admin();
        require!(token.is_valid_esdt_identifier(), INVALID_TOKEN_ID);

        self.allowed_tokens().remove(&token);
    }

    fn require_caller_is_admin(&self) {
        let caller = self.blockchain().get_caller();
        let sc_owner = self.blockchain().get_owner_address();
        if caller == sc_owner {
            return;
        }

        self.admin_whitelist().require_whitelisted(&caller);
    }

    #[storage_mapper("adminWhitelist")]
    fn admin_whitelist(&self) -> WhitelistMapper<Self::Api, ManagedAddress>;

    #[storage_mapper("euriborRate")]
    fn euribor_rate(&self) -> SingleValueMapper<(u8, u64)>;

    #[storage_mapper("allowedTokens")]
    fn allowed_tokens(&self) -> WhitelistMapper<TokenIdentifier>;
}