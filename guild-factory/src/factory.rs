use guild_sc::custom_rewards::ProxyTrait as _;
use guild_sc_config::tier_types::{GuildMasterRewardTier, UserRewardTier};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

static UNKNOWN_GUILD_ERR_MSG: &[u8] = b"Unknown guild";

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct GuildLocalConfig<M: ManagedTypeApi> {
    pub farming_token_id: TokenIdentifier<M>,
    pub division_safety_constant: BigUint<M>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct GetGuildResultType<M: ManagedTypeApi> {
    pub guild: ManagedAddress<M>,
    pub guild_master: ManagedAddress<M>,
}

#[multiversx_sc::module]
pub trait FactoryModule: crate::config::ConfigModule + utils::UtilsModule {
    #[only_owner]
    #[endpoint(setMaxActiveGuilds)]
    fn set_max_active_guilds(&self, max_active_guilds: usize) {
        let current_active_guilds = self.get_current_active_guilds();
        require!(
            max_active_guilds >= current_active_guilds,
            "May not set active guilds number below current active guilds"
        );

        self.max_active_guilds().set(max_active_guilds);
    }

    #[only_owner]
    #[endpoint(setGuildScSourceAddress)]
    fn set_guild_sc_source_address(&self, sc_addr: ManagedAddress) {
        self.require_sc_address(&sc_addr);

        self.guild_sc_source_address().set(sc_addr);
    }

    #[only_owner]
    #[endpoint(upgradeGuild)]
    fn upgrade_guild(&self, guild_address: ManagedAddress) {
        let guild_id = self.guild_ids().get_id_non_zero(&guild_address);
        self.require_known_guild(guild_id);

        let source_contract_address = self.guild_sc_source_address().get();
        let gas_left = self.blockchain().get_gas_left();
        let code_metadata = self.get_default_code_metadata();
        self.send_raw().upgrade_from_source_contract(
            &guild_address,
            gas_left,
            &BigUint::zero(),
            &source_contract_address,
            code_metadata,
            &ManagedArgBuffer::new(),
        );
    }

    #[endpoint(deployGuild)]
    fn deploy_guild(&self) -> ManagedAddress {
        let caller = self.blockchain().get_caller();
        let caller_id = self.user_ids().get_id_or_insert(&caller);
        let guild_mapper = self.guild_sc_for_user(caller_id);
        require!(guild_mapper.is_empty(), "Already have a guild deployed");

        let config_sc_mapper = self.config_sc_address();
        require!(!config_sc_mapper.is_empty(), "Config not deployed yet");

        self.require_config_setup_complete();

        let guild_config = self.guild_local_config().get();
        let config_sc_address = config_sc_mapper.get();
        let source_address = self.guild_sc_source_address().get();
        let code_metadata = self.get_default_code_metadata();
        let (guild_address, _) = self
            .guild_proxy()
            .init(
                guild_config.farming_token_id,
                guild_config.division_safety_constant,
                config_sc_address,
                caller,
                MultiValueEncoded::new(),
            )
            .deploy_from_source::<()>(&source_address, code_metadata);

        let guild_id = self.guild_ids().insert_new(&guild_address);
        let _ = self.deployed_guilds().insert(guild_id);
        self.guild_master_for_guild(guild_id).set(caller_id);
        guild_mapper.set(guild_id);

        guild_address
    }

    #[endpoint(resumeGuild)]
    fn resume_guild_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let guild_id = self.guild_ids().get_id_non_zero(&caller);
        self.require_known_guild(guild_id);

        let current_active_guilds = self.get_current_active_guilds();
        let max_active_guilds = self.max_active_guilds().get();
        require!(
            current_active_guilds < max_active_guilds,
            "May not start another guild at this point"
        );
        require!(
            !self.active_guilds().contains(&guild_id),
            "Guild already active"
        );
        self.require_config_setup_complete();
        self.require_guild_setup_complete(caller.clone());

        self.start_produce_rewards(caller);

        self.active_guilds().insert(guild_id);
    }

    #[view(getAllGuilds)]
    fn get_all_guilds(&self) -> MultiValueEncoded<GetGuildResultType<Self::Api>> {
        let mut result = MultiValueEncoded::new();
        for guild_id in self.deployed_guilds().iter() {
            let guild_master_id = self.guild_master_for_guild(guild_id).get();
            let opt_guild_address = self.guild_ids().get_address(guild_id);
            let opt_guild_master_address = self.user_ids().get_address(guild_master_id);
            require!(
                opt_guild_address.is_some() && opt_guild_master_address.is_some(),
                "Invalid setup"
            );

            let guild_address = unsafe { opt_guild_address.unwrap_unchecked() };
            let guild_master_address = unsafe { opt_guild_master_address.unwrap_unchecked() };
            result.push(GetGuildResultType {
                guild: guild_address,
                guild_master: guild_master_address,
            });
        }

        result
    }

    #[view(getGuildId)]
    fn get_guild_id(&self, guild_address: ManagedAddress) -> AddressId {
        self.guild_ids().get_id_non_zero(&guild_address)
    }

    #[view(getCurrentActiveGuilds)]
    fn get_current_active_guilds(&self) -> usize {
        self.active_guilds().len()
    }

    fn remove_guild_common(&self, guild: ManagedAddress) {
        let guild_master = self.external_guild_master_address(guild.clone()).get();
        let guild_id = self.guild_ids().remove_by_address(&guild);
        let user_id = self.user_ids().remove_by_address(&guild_master);

        let removed = self.deployed_guilds().swap_remove(&guild_id);
        require!(removed, UNKNOWN_GUILD_ERR_MSG);

        let mapper = self.guild_sc_for_user(user_id);
        require!(!mapper.is_empty(), "Unknown guild master");

        mapper.clear();

        self.guild_master_for_guild(guild_id).clear();
    }

    fn require_known_guild(&self, guild_id: AddressId) {
        require!(
            self.deployed_guilds().contains(&guild_id),
            UNKNOWN_GUILD_ERR_MSG
        );
    }

    fn require_config_setup_complete(&self) {
        let config_sc_address = self.config_sc_address().get();
        let guild_master_tiers_mapper = self.external_guild_master_tiers(config_sc_address.clone());
        let user_tiers_mapper = self.external_user_tiers(config_sc_address);
        require!(
            !guild_master_tiers_mapper.is_empty() && !user_tiers_mapper.is_empty(),
            "Config setup not complete"
        );
    }

    fn require_guild_setup_complete(&self, guild: ManagedAddress) {
        let _: IgnoreValue = self
            .guild_proxy()
            .contract(guild)
            .check_local_roles_set()
            .execute_on_dest_context();
    }

    fn start_produce_rewards(&self, guild: ManagedAddress) {
        let _: IgnoreValue = self
            .guild_proxy()
            .contract(guild)
            .start_produce_rewards_endpoint()
            .execute_on_dest_context();
    }

    #[proxy]
    fn guild_proxy(&self) -> guild_sc::Proxy<Self::Api>;

    #[view(getGuildScSourceAddress)]
    #[storage_mapper("guildScSourceAddress")]
    fn guild_sc_source_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("guildLocalConfig")]
    fn guild_local_config(&self) -> SingleValueMapper<GuildLocalConfig<Self::Api>>;

    #[storage_mapper("deployedGuilds")]
    fn deployed_guilds(&self) -> UnorderedSetMapper<AddressId>;

    #[storage_mapper("activeGuilds")]
    fn active_guilds(&self) -> UnorderedSetMapper<AddressId>;

    #[storage_mapper("guildScForUser")]
    fn guild_sc_for_user(&self, user_id: AddressId) -> SingleValueMapper<AddressId>;

    #[storage_mapper("guildMasterForGuild")]
    fn guild_master_for_guild(&self, guild_id: AddressId) -> SingleValueMapper<AddressId>;

    #[view(getRemainingRewards)]
    #[storage_mapper("remainingRewards")]
    fn remaining_rewards(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("userIds")]
    fn user_ids(&self) -> AddressToIdMapper<Self::Api>;

    #[storage_mapper("guildIds")]
    fn guild_ids(&self) -> AddressToIdMapper<Self::Api>;

    #[view(getMaxActiveGuilds)]
    #[storage_mapper("maxActiveGuilds")]
    fn max_active_guilds(&self) -> SingleValueMapper<usize>;

    // guild storage

    #[storage_mapper_from_address("guildMasterAddress")]
    fn external_guild_master_address(
        &self,
        sc_addr: ManagedAddress,
    ) -> SingleValueMapper<ManagedAddress, ManagedAddress>;

    #[storage_mapper_from_address("guildMasterTiers")]
    fn external_guild_master_tiers(
        &self,
        sc_addr: ManagedAddress,
    ) -> VecMapper<GuildMasterRewardTier<Self::Api>, ManagedAddress>;

    #[storage_mapper_from_address("userTiers")]
    fn external_user_tiers(
        &self,
        sc_addr: ManagedAddress,
    ) -> VecMapper<UserRewardTier, ManagedAddress>;

    // proxy

    #[proxy]
    fn config_proxy_factory(&self, sc_addr: ManagedAddress) -> guild_sc_config::Proxy<Self::Api>;
}
