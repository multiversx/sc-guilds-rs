// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           23
// Async Callback (empty):               1
// Total number of exported functions:  26

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    guild_factory
    (
        init => init
        upgrade => upgrade
        deployConfigSc => deploy_config_sc
        upgradeConfigSc => upgrade_config_sc
        callConfigFunction => call_config_function
        getConfigAddress => config_sc_address
        setMaxActiveGuilds => set_max_active_guilds
        upgradeGuild => upgrade_guild
        deployGuild => deploy_guild
        resumeGuild => resume_guild_endpoint
        getAllGuilds => get_all_guilds
        getGuildId => get_guild_id
        getCurrentActiveGuilds => get_current_active_guilds
        getRemainingRewards => remaining_rewards
        getMaxActiveGuilds => max_active_guilds
        requestRewards => request_rewards
        migrateToOtherGuild => migrate_to_other_guild
        depositRewardsGuild => deposit_rewards_guild
        closeGuildNoRewardsRemaining => close_guild_no_rewards_remaining
        depositRewardsAdmins => deposit_rewards_admins
        getClosedGuilds => closed_guilds
        isAdmin => is_admin
        addAdmin => add_admin
        removeAdmin => remove_admin
        getAdmins => admins
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
