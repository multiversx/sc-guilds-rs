multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug)]
pub struct TotalTokens<M: ManagedTypeApi> {
    pub base: BigUint<M>,
    pub compounded: BigUint<M>,
}

impl<M: ManagedTypeApi> Default for TotalTokens<M> {
    fn default() -> Self {
        Self {
            base: BigUint::zero(),
            compounded: BigUint::zero(),
        }
    }
}

impl<M: ManagedTypeApi> TotalTokens<M> {
    pub fn new(base_amount: BigUint<M>, compounded_amount: BigUint<M>) -> Self {
        Self {
            base: base_amount,
            compounded: compounded_amount,
        }
    }

    pub fn new_base(base_amount: BigUint<M>) -> Self {
        Self {
            base: base_amount,
            compounded: BigUint::zero(),
        }
    }

    pub fn new_compounded(compounded_amount: BigUint<M>) -> Self {
        Self {
            base: BigUint::zero(),
            compounded: compounded_amount,
        }
    }

    pub fn is_default(&self) -> bool {
        let big_zero = BigUint::zero();

        self.base == big_zero && self.compounded == big_zero
    }

    pub fn total(&self) -> BigUint<M> {
        &self.base + &self.compounded
    }
}

#[multiversx_sc::module]
pub trait TokenPerTierModule: super::read_config::ReadConfigModule {
    #[view(getUserStakedTokens)]
    fn get_user_staked_tokens(&self, user: ManagedAddress) -> TotalTokens<Self::Api> {
        let guild_master = self.guild_master().get();
        let mapper = if user != guild_master {
            self.user_tokens(&user)
        } else {
            self.guild_master_tokens()
        };

        if !mapper.is_empty() {
            mapper.get()
        } else {
            TotalTokens::default()
        }
    }

    fn add_total_base_staked_tokens(&self, amount: &BigUint) {
        let max_staked_tokens = self.get_max_staked_tokens();
        self.total_base_staked_tokens().update(|total| {
            *total += amount;

            require!(
                *total <= max_staked_tokens,
                "May not stake more in this guild"
            );
        });
    }

    #[inline]
    fn remove_total_base_staked_tokens(&self, amount: &BigUint) {
        self.total_base_staked_tokens().update(|total| {
            *total -= amount;
        });
    }

    fn add_tokens(&self, caller: &ManagedAddress, tokens: &TotalTokens<Self::Api>) {
        let guild_master = self.guild_master().get();
        if caller != &guild_master {
            let user_tokens_mapper = self.user_tokens(caller);
            self.add_tokens_common(tokens, &user_tokens_mapper);
        } else {
            let guild_master_tokens_mapper = self.guild_master_tokens();
            self.add_tokens_common(tokens, &guild_master_tokens_mapper);
        }
    }

    fn add_tokens_common(
        &self,
        tokens: &TotalTokens<Self::Api>,
        mapper: &SingleValueMapper<TotalTokens<Self::Api>>,
    ) {
        if !mapper.is_empty() {
            mapper.update(|total_tokens| {
                total_tokens.base += &tokens.base;
                total_tokens.compounded += &tokens.compounded;
            });
        } else {
            mapper.set(tokens);
        }
    }

    fn remove_tokens(&self, caller: &ManagedAddress, tokens: &TotalTokens<Self::Api>) {
        let guild_master = self.guild_master().get();
        if caller != &guild_master {
            let user_tokens_mapper = self.user_tokens(caller);
            self.remove_tokens_common(tokens, &user_tokens_mapper);
        } else {
            let guild_master_tokens_mapper = self.guild_master_tokens();
            self.remove_tokens_common(tokens, &guild_master_tokens_mapper);
        }
    }

    fn remove_tokens_common(
        &self,
        tokens: &TotalTokens<Self::Api>,
        mapper: &SingleValueMapper<TotalTokens<Self::Api>>,
    ) {
        mapper.update(|total_tokens| {
            total_tokens.base -= &tokens.base;
            total_tokens.compounded -= &tokens.compounded;
        });
    }

    fn get_total_stake_for_user(&self, user: &ManagedAddress) -> BigUint {
        let guild_master = self.guild_master().get();
        let total_tokens = if user != &guild_master {
            self.user_tokens(user).get()
        } else {
            self.guild_master_tokens().get()
        };

        total_tokens.total()
    }

    fn require_over_min_stake(&self, user: &ManagedAddress) {
        let total_stake = self.get_total_stake_for_user(user);
        let guild_master = self.guild_master().get();
        if user != &guild_master && total_stake == 0 {
            return;
        }

        let min_stake = self.get_min_stake_for_user(user);
        require!(total_stake >= min_stake, "Not enough stake");
    }

    #[storage_mapper("totalBaseStakedTokens")]
    fn total_base_staked_tokens(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("totalCompoundedTokens")]
    fn total_compounded_tokens(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("guildMasterTokens")]
    fn guild_master_tokens(&self) -> SingleValueMapper<TotalTokens<Self::Api>>;

    #[storage_mapper("userTokens")]
    fn user_tokens(&self, user: &ManagedAddress) -> SingleValueMapper<TotalTokens<Self::Api>>;
}
