// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           47
// Async Callback:                       1
// Total number of exported functions:  49

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    guild_sc
    (
        init => init
        upgrade => upgrade
        mergeFarmTokens => merge_farm_tokens_endpoint
        checkLocalRolesSet => check_local_roles_set
        calculateRewardsForGivenPosition => calculate_rewards_for_given_position
        topUpRewards => top_up_rewards
        startProduceRewards => start_produce_rewards_endpoint
        withdrawRewards => withdraw_rewards
        getAccumulatedRewards => accumulated_rewards
        getRewardCapacity => reward_capacity
        getRewardPerShare => reward_per_share
        getRewardReserve => reward_reserve
        allowExternalClaimBoostedRewards => allow_external_claim_boosted_rewards
        getAllowExternalClaimRewards => get_allow_external_claim_rewards
        getFarmingTokenId => farming_token_id
        getRewardTokenId => reward_token_id
        getPerBlockRewardAmount => per_block_reward_amount
        getLastRewardBlockNonce => last_reward_block_nonce
        getDivisionSafetyConstant => division_safety_constant
        getUserTotalFarmPosition => user_total_farm_position
        getFarmPositionMigrationNonce => farm_position_migration_nonce
        registerFarmToken => register_farm_token
        setTransferRoleFarmToken => set_transfer_role_farm_token
        getFarmTokenId => farm_token
        getFarmTokenSupply => farm_token_supply
        addSCAddressToWhitelist => add_sc_address_to_whitelist
        removeSCAddressFromWhitelist => remove_sc_address_from_whitelist
        isSCAddressWhitelisted => is_sc_address_whitelisted
        addToPauseWhitelist => add_to_pause_whitelist
        removeFromPauseWhitelist => remove_from_pause_whitelist
        pause => pause
        resume => resume
        getState => state
        addAdmin => add_admin_endpoint
        removeAdmin => remove_admin_endpoint
        updateOwnerOrAdmin => update_owner_or_admin_endpoint
        getPermissions => permissions
        stakeFarm => stake_farm_endpoint
        claimRewards => claim_rewards
        compoundRewards => compound_rewards
        unstakeFarm => unstake_farm
        unbondFarm => unbond_farm
        cancelUnbond => cancel_unbond
        registerUnbondToken => register_unbond_token
        setTransferRoleUnbondToken => set_transfer_role_unbond_token
        getUnbondTokenId => unbond_token
        closeGuild => close_guild
        migrateToOtherGuild => migrate_to_other_guild
    )
}

multiversx_sc_wasm_adapter::async_callback! { guild_sc }
