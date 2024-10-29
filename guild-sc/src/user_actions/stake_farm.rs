multiversx_sc::imports!();

use common_structs::PaymentsVec;

use crate::farm_base_impl::base_traits_impl::FarmStakingWrapper;

#[multiversx_sc::module]
pub trait StakeFarmModule:
    crate::custom_rewards::CustomRewardsModule
    + crate::rewards::RewardsModule
    + crate::config::ConfigModule
    + crate::events::EventsModule
    + token_send::TokenSendModule
    + crate::tokens::farm_token::FarmTokenModule
    + crate::tokens::request_id::RequestIdModule
    + permissions_module::PermissionsModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + crate::farm_base_impl::base_farm_validation::BaseFarmValidationModule
    + crate::farm_base_impl::enter_farm::BaseEnterFarmModule
    + utils::UtilsModule
    + crate::tiered_rewards::read_config::ReadConfigModule
    + crate::tiered_rewards::total_tokens::TokenPerTierModule
    + crate::tiered_rewards::call_config::CallConfigModule
    + super::close_guild::CloseGuildModule
    + crate::tokens::unbond_token::UnbondTokenModule
{
    #[payable("*")]
    #[endpoint(stakeFarm)]
    fn stake_farm_endpoint(
        &self,
        opt_original_caller: OptionalValue<ManagedAddress>,
    ) -> EsdtTokenPayment {
        let caller = self.blockchain().get_caller();
        let original_caller = self.get_orig_caller_from_opt(&caller, opt_original_caller);
        let payments = self.get_non_empty_payments();

        self.stake_farm_common(original_caller, payments)
    }

    fn stake_farm_common(
        &self,
        original_caller: ManagedAddress,
        payments: PaymentsVec<Self::Api>,
    ) -> EsdtTokenPayment {
        self.require_not_closing();
        self.require_not_globally_paused();
        self.unbond_token().require_issued_or_set();

        let guild_master = self.guild_master_address().get();
        if original_caller != guild_master {
            require!(
                !self.guild_master_tokens().is_empty(),
                "Guild master must stake first"
            );
        } else {
            self.require_guild_master_can_stake();
        }

        let enter_result =
            self.enter_farm_base::<FarmStakingWrapper<Self>>(original_caller.clone(), payments);

        let enter_farm_amount = enter_result.context.farming_token_payment.amount.clone();
        self.add_total_base_staked_tokens(&enter_farm_amount);
        self.add_tokens(&original_caller, &enter_farm_amount);
        self.call_increase_total_staked_tokens(enter_farm_amount);

        self.require_over_min_stake(&original_caller);

        let new_farm_token = enter_result.new_farm_token.payment.clone();
        self.send_payment_non_zero(&original_caller, &new_farm_token);

        self.emit_enter_farm_event(
            &original_caller,
            enter_result.context.farming_token_payment,
            enter_result.new_farm_token,
            enter_result.created_with_merge,
            enter_result.storage_cache,
        );

        new_farm_token
    }

    fn require_guild_master_can_stake(&self) {
        let factory_address = self.blockchain().get_owner_address();
        let own_sc_address = self.blockchain().get_sc_address();
        let guild_id = self
            .external_guild_ids(factory_address.clone())
            .get_id_non_zero(&own_sc_address);

        let active_guilds_mapper = self.external_active_guilds(factory_address.clone());
        if !active_guilds_mapper.contains(&guild_id) {
            let max_guilds = self.external_max_active_guilds(factory_address).get();
            require!(
                active_guilds_mapper.len() < max_guilds,
                "May not create guild at this time, limit exceeded"
            );
        }
    }

    fn get_orig_caller_from_opt(
        &self,
        caller: &ManagedAddress,
        opt_original_caller: OptionalValue<ManagedAddress>,
    ) -> ManagedAddress {
        match opt_original_caller {
            OptionalValue::Some(original_caller) => {
                let factory_sc_address = self.blockchain().get_owner_address();
                require!(
                    caller == &factory_sc_address,
                    "May not use original caller arg"
                );

                original_caller
            }
            OptionalValue::None => caller.clone(),
        }
    }

    // factory storage

    #[storage_mapper_from_address("guildIds")]
    fn external_guild_ids(
        &self,
        sc_address: ManagedAddress,
    ) -> AddressToIdMapper<Self::Api, ManagedAddress>;

    #[storage_mapper_from_address("activeGuilds")]
    fn external_active_guilds(
        &self,
        sc_address: ManagedAddress,
    ) -> UnorderedSetMapper<AddressId, ManagedAddress>;

    #[storage_mapper_from_address("maxActiveGuilds")]
    fn external_max_active_guilds(
        &self,
        sc_address: ManagedAddress,
    ) -> SingleValueMapper<usize, ManagedAddress>;
}
