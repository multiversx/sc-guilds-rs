#![allow(deprecated)]

use multiversx_sc_scenario::{
    managed_token_id, rust_biguint, whitebox_legacy::TxTokenTransfer, DebugApi,
};

pub mod farm_staking_setup;
use farm_staking_setup::*;
use guild_sc::{
    custom_rewards::{BLOCKS_IN_YEAR, MAX_PERCENT},
    user_actions::unbond_farm::UnbondFarmModule,
};

#[test]
fn test_farm_setup() {
    let _ = FarmStakingSetup::new(
        guild_sc::contract_obj,
        energy_factory::contract_obj,
        guild_sc_config::contract_obj,
    );
}

#[test]
fn test_enter_farm() {
    DebugApi::dummy();
    let mut farm_setup = FarmStakingSetup::new(
        guild_sc::contract_obj,
        energy_factory::contract_obj,
        guild_sc_config::contract_obj,
    );

    let farm_in_amount = 100_000_000;
    let expected_farm_token_nonce = 2;
    farm_setup.stake_farm(farm_in_amount, &[], expected_farm_token_nonce, 0, 0);
    farm_setup.check_farm_token_supply(farm_in_amount + 1);
}

#[test]
fn test_unstake_farm() {
    DebugApi::dummy();
    let mut farm_setup = FarmStakingSetup::new(
        guild_sc::contract_obj,
        energy_factory::contract_obj,
        guild_sc_config::contract_obj,
    );

    let farm_in_amount = 100_000_000;
    let expected_farm_token_nonce = 2;
    farm_setup.stake_farm(farm_in_amount, &[], expected_farm_token_nonce, 0, 0);
    farm_setup.check_farm_token_supply(farm_in_amount + 1);

    let current_block = 10;
    let current_epoch = 5;
    farm_setup.set_block_epoch(current_epoch);
    farm_setup.set_block_nonce(current_block);

    let block_diff = current_block;
    let expected_rewards_unbounded = block_diff * PER_BLOCK_REWARD_AMOUNT;

    // ~= 4 * 10 = 40
    let expected_rewards_max_apr =
        farm_in_amount * MAX_APR / MAX_PERCENT / BLOCKS_IN_YEAR * block_diff - 1;
    let expected_rewards = core::cmp::min(expected_rewards_unbounded, expected_rewards_max_apr);
    assert_eq!(expected_rewards, 39);

    let expected_ride_token_balance =
        rust_biguint!(USER_TOTAL_RIDE_TOKENS) - farm_in_amount + expected_rewards;
    farm_setup.unstake_farm(
        farm_in_amount,
        expected_farm_token_nonce,
        expected_rewards,
        &expected_ride_token_balance,
        &expected_ride_token_balance,
        1,
        farm_in_amount,
    );
    farm_setup.check_farm_token_supply(1);
}

#[test]
fn test_claim_rewards() {
    DebugApi::dummy();
    let mut farm_setup = FarmStakingSetup::new(
        guild_sc::contract_obj,
        energy_factory::contract_obj,
        guild_sc_config::contract_obj,
    );

    let farm_in_amount = 100_000_000;
    let expected_farm_token_nonce = 2;
    farm_setup.stake_farm(farm_in_amount, &[], expected_farm_token_nonce, 0, 0);
    farm_setup.check_farm_token_supply(farm_in_amount + 1);

    farm_setup.set_block_epoch(5);
    farm_setup.set_block_nonce(10);

    // value taken from the "test_unstake_farm" test
    let expected_reward_token_out = 39;
    let expected_farming_token_balance =
        rust_biguint!(USER_TOTAL_RIDE_TOKENS - farm_in_amount + expected_reward_token_out);
    let expected_reward_per_share = 399_999;
    farm_setup.claim_rewards(
        farm_in_amount,
        expected_farm_token_nonce,
        expected_reward_token_out,
        &expected_farming_token_balance,
        &expected_farming_token_balance,
        expected_farm_token_nonce + 1,
        expected_reward_per_share,
    );
    farm_setup.check_farm_token_supply(farm_in_amount + 1);
}

fn steps_enter_farm_twice<FarmObjBuilder, EnergyFactoryBuilder, ConfigScBuilder>(
    farm_builder: FarmObjBuilder,
    energy_factory_builder: EnergyFactoryBuilder,
    config_sc_builder: ConfigScBuilder,
) -> FarmStakingSetup<FarmObjBuilder, EnergyFactoryBuilder, ConfigScBuilder>
where
    FarmObjBuilder: 'static + Copy + Fn() -> guild_sc::ContractObj<DebugApi>,
    EnergyFactoryBuilder: 'static + Copy + Fn() -> energy_factory::ContractObj<DebugApi>,
    ConfigScBuilder: 'static + Copy + Fn() -> guild_sc_config::ContractObj<DebugApi>,
{
    let mut farm_setup =
        FarmStakingSetup::new(farm_builder, energy_factory_builder, config_sc_builder);

    let farm_in_amount = 100_000_000;
    let expected_farm_token_nonce = 2;
    farm_setup.stake_farm(farm_in_amount, &[], expected_farm_token_nonce, 0, 0);
    farm_setup.check_farm_token_supply(farm_in_amount + 1);

    farm_setup.set_block_epoch(5);
    farm_setup.set_block_nonce(10);

    let second_farm_in_amount = 200_000_000;
    let prev_farm_tokens = [TxTokenTransfer {
        token_identifier: FARM_TOKEN_ID.to_vec(),
        nonce: expected_farm_token_nonce,
        value: rust_biguint!(farm_in_amount),
    }];

    let total_amount = farm_in_amount + second_farm_in_amount + 1;
    let first_reward_share = 0;
    let second_reward_share = 399_999;
    let expected_reward_per_share = (first_reward_share * farm_in_amount
        + second_reward_share * second_farm_in_amount
        + total_amount
        - 1)
        / total_amount;

    farm_setup.stake_farm(
        second_farm_in_amount,
        &prev_farm_tokens,
        expected_farm_token_nonce + 1,
        expected_reward_per_share,
        0,
    );
    farm_setup.check_farm_token_supply(total_amount);

    farm_setup
}

#[test]
fn test_enter_farm_twice() {
    DebugApi::dummy();
    let _ = steps_enter_farm_twice(
        guild_sc::contract_obj,
        energy_factory::contract_obj,
        guild_sc_config::contract_obj,
    );
}

#[test]
fn test_exit_farm_after_enter_twice() {
    DebugApi::dummy();
    let mut farm_setup = steps_enter_farm_twice(
        guild_sc::contract_obj,
        energy_factory::contract_obj,
        guild_sc_config::contract_obj,
    );
    let farm_in_amount = 100_000_000;
    let second_farm_in_amount = 200_000_000;

    farm_setup.set_block_epoch(8);
    farm_setup.set_block_nonce(25);

    let expected_rewards = 83;
    let expected_ride_token_balance =
        rust_biguint!(USER_TOTAL_RIDE_TOKENS) - farm_in_amount - second_farm_in_amount
            + expected_rewards;
    farm_setup.unstake_farm(
        farm_in_amount,
        3,
        expected_rewards,
        &expected_ride_token_balance,
        &expected_ride_token_balance,
        1,
        farm_in_amount,
    );
    farm_setup.check_farm_token_supply(second_farm_in_amount + 1);
}

#[test]
fn test_unbond() {
    DebugApi::dummy();
    let mut farm_setup = FarmStakingSetup::new(
        guild_sc::contract_obj,
        energy_factory::contract_obj,
        guild_sc_config::contract_obj,
    );

    let farm_in_amount = 100_000_000;
    let expected_farm_token_nonce = 2;
    farm_setup.stake_farm(farm_in_amount, &[], expected_farm_token_nonce, 0, 0);
    farm_setup.check_farm_token_supply(farm_in_amount + 1);

    let current_block = 10;
    let current_epoch = 5;
    farm_setup.set_block_epoch(current_epoch);
    farm_setup.set_block_nonce(current_block);

    let block_diff = current_block;
    let expected_rewards_unbounded = block_diff * PER_BLOCK_REWARD_AMOUNT;

    // ~= 4 * 10 = 40
    let expected_rewards_max_apr =
        farm_in_amount * MAX_APR / MAX_PERCENT / BLOCKS_IN_YEAR * block_diff - 1;
    let expected_rewards = core::cmp::min(expected_rewards_unbounded, expected_rewards_max_apr);
    assert_eq!(expected_rewards, 39);

    let expected_ride_token_balance =
        rust_biguint!(USER_TOTAL_RIDE_TOKENS) - farm_in_amount + expected_rewards;
    farm_setup.unstake_farm(
        farm_in_amount,
        expected_farm_token_nonce,
        expected_rewards,
        &expected_ride_token_balance,
        &expected_ride_token_balance,
        1,
        farm_in_amount,
    );
    farm_setup.check_farm_token_supply(1);

    farm_setup.set_block_epoch(current_epoch + MIN_UNBOND_EPOCHS);

    farm_setup.unbond_farm(
        1,
        farm_in_amount,
        farm_in_amount,
        USER_TOTAL_RIDE_TOKENS + expected_rewards,
    );
}

#[test]
fn test_withdraw_rewards() {
    DebugApi::dummy();
    let mut farm_setup = FarmStakingSetup::new(
        guild_sc::contract_obj,
        energy_factory::contract_obj,
        guild_sc_config::contract_obj,
    );

    let initial_rewards_capacity = 1_000_000_000_000u64;
    farm_setup.check_rewards_capacity(initial_rewards_capacity);

    let withdraw_amount = rust_biguint!(TOTAL_REWARDS_AMOUNT);
    farm_setup.withdraw_rewards(&withdraw_amount);

    let final_rewards_capacity = 0u64;
    farm_setup.check_rewards_capacity(final_rewards_capacity);
}

#[test]
fn test_withdraw_after_produced_rewards() {
    DebugApi::dummy();
    let mut farm_setup = FarmStakingSetup::new(
        guild_sc::contract_obj,
        energy_factory::contract_obj,
        guild_sc_config::contract_obj,
    );

    let initial_rewards_capacity = 1_000_000_000_000u64;
    farm_setup.check_rewards_capacity(initial_rewards_capacity);

    let farm_in_amount = 100_000_000;
    let expected_farm_token_nonce = 2;
    farm_setup.stake_farm(farm_in_amount, &[], expected_farm_token_nonce, 0, 0);
    farm_setup.check_farm_token_supply(farm_in_amount + 1);

    farm_setup.set_block_epoch(5);
    farm_setup.set_block_nonce(10);

    let withdraw_amount = rust_biguint!(TOTAL_REWARDS_AMOUNT);
    farm_setup.withdraw_rewards_with_error(&withdraw_amount, 4, WITHDRAW_AMOUNT_TOO_HIGH);

    let expected_reward_token_out = 40;

    let withdraw_amount =
        rust_biguint!(TOTAL_REWARDS_AMOUNT) - rust_biguint!(expected_reward_token_out);
    farm_setup.withdraw_rewards(&withdraw_amount);

    // Only the user's rewards will remain
    let final_rewards_capacity = expected_reward_token_out;
    farm_setup.check_rewards_capacity(final_rewards_capacity);
}

#[test]
fn cancel_unbond_test() {
    DebugApi::dummy();
    let mut farm_setup = FarmStakingSetup::new(
        guild_sc::contract_obj,
        energy_factory::contract_obj,
        guild_sc_config::contract_obj,
    );

    let farm_in_amount = 100_000_000;
    let expected_farm_token_nonce = 2;
    farm_setup.stake_farm(farm_in_amount, &[], expected_farm_token_nonce, 0, 0);
    farm_setup.check_farm_token_supply(farm_in_amount + 1);

    let current_block = 10;
    let current_epoch = 5;
    farm_setup.set_block_epoch(current_epoch);
    farm_setup.set_block_nonce(current_block);

    let block_diff = current_block;
    let expected_rewards_unbounded = block_diff * PER_BLOCK_REWARD_AMOUNT;

    // ~= 4 * 10 = 40
    let expected_rewards_max_apr =
        farm_in_amount * MAX_APR / MAX_PERCENT / BLOCKS_IN_YEAR * block_diff - 1;
    let expected_rewards = core::cmp::min(expected_rewards_unbounded, expected_rewards_max_apr);
    assert_eq!(expected_rewards, 39);

    let expected_ride_token_balance =
        rust_biguint!(USER_TOTAL_RIDE_TOKENS) - farm_in_amount + expected_rewards;
    farm_setup.unstake_farm(
        farm_in_amount,
        expected_farm_token_nonce,
        expected_rewards,
        &expected_ride_token_balance,
        &expected_ride_token_balance,
        1,
        farm_in_amount,
    );
    farm_setup.check_farm_token_supply(1);

    farm_setup.set_block_epoch(current_epoch + MIN_UNBOND_EPOCHS);

    farm_setup
        .b_mock
        .execute_esdt_transfer(
            &farm_setup.user_address,
            &farm_setup.farm_wrapper,
            UNBOND_TOKEN_ID,
            1,
            &rust_biguint!(farm_in_amount),
            |sc| {
                let original_farm_token = sc.cancel_unbond();
                assert_eq!(
                    original_farm_token.token_identifier,
                    managed_token_id!(FARM_TOKEN_ID)
                );
            },
        )
        .assert_ok();
}
