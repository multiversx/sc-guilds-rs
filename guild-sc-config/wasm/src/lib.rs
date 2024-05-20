// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           22
// Async Callback (empty):               1
// Total number of exported functions:  24

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    guild_sc_config
    (
        init => init
        upgrade => upgrade
        addGuildMasterTiers => add_guild_master_tiers
        setGuildMasterTierApr => set_guild_master_tier_apr
        addUserTiers => add_user_tiers
        setUserTierApr => set_user_tier_apr
        getGuildMasterTiers => guild_master_tiers
        getUserTiers => user_tiers
        setMinUnbondEpochsUser => set_min_unbond_epochs_user
        setMinUnbondEpochsGuildMaster => set_min_unbond_epochs_guild_master
        setMinStakeUser => set_min_stake_user
        setMinStakeGuildMaster => set_min_stake_guild_master
        setTotalStakingTokenMinted => set_total_staking_token_minted
        increaseStakedTokens => increase_staked_tokens
        decreaseStakedTokens => decrease_staked_tokens
        getTotalStakedPercent => get_total_staked_percent
        getMaxStakedTokens => max_staked_tokens
        getMinUnbondEpochsUser => min_unbond_epochs_user
        getMinUnbondEpochsGuildMaster => min_unbond_epochs_guild_master
        getMinStakeUser => min_stake_user
        getMinStakeGuildMaster => min_stake_guild_master
        getTotalStakingTokenMinted => total_staking_token_minted
        getTotalStakingTokenStaked => total_staking_token_staked
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
