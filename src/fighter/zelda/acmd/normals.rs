use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::*
};

unsafe extern "C" fn zelda_game_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 2.4, 0.0, 12.0, 4.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 2.4, 0.0, 12.0, 7.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 2.8, 0.0, 5.5, 4.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.2, 0.0, 11.8, 10.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.2, 0.0, 11.8, 10.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.0, 0.0, 11.8, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.0, 0.0, 11.8, 8.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.2, 0.0, 5.5, 5.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.5, 0.0, 11.5, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 14.5, 88, 5, 0, 102, 3.5, 0.0, 11.5, 11.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 2.0, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn zelda_game_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 40, 0, 3.5, 0.0, 12.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 160, 100, 20, 0, 4.0, 0.0, 18.0, -5.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 100, 100, 60, 0, 4.0, 0.0, 18.0, -5.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 160, 100, 20, 0, 4.0, 0.0, 18.0, 5.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 100, 100, 60, 0, 4.0, 0.0, 18.0, 5.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 367, 100, 20, 0, 4.6, 0.0, 19.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.8, 90, 100, 40, 0, 3.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 160, 100, 20, 0, 4.0, 0.0, 18.0, -5.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 100, 100, 60, 0, 4.0, 0.0, 18.0, -5.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 160, 100, 20, 0, 4.0, 0.0, 18.0, 5.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 0.8, 100, 100, 60, 0, 4.0, 0.0, 18.0, 5.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 0.8, 367, 100, 20, 0, 4.6, 0.0, 19.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        let rand = smash::app::sv_math::rand(hash40("fighter"), 8) as u64;
        println!("{}",rand);
        if rand != 7{
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 88, 222, 0, 24, 7.0, 0.0, 21.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_deathball"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 88, 222, 0, 24, 6.5, 0.0, 18.0, -6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_deathball"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 88, 222, 0, 24, 6.5, 0.0, 18.0, 6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_deathball"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 88, 222, 0, 24, 8.0, 0.0, 15.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_deathball"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        } else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 88, 300, 0, 34, 7.0, 0.0, 21.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 88, 300, 0, 34, 6.5, 0.0, 18.0, -6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 88, 300, 0, 34, 6.5, 0.0, 18.0, 6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 88, 300, 0, 34, 8.0, 0.0, 15.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// unsafe extern "C" fn zelda_effect_attackhi4(agent: &mut L2CAgentBase) {
//     if get_value_float(agent, *SO_VAR_FLOAT_LR) < 0.0 {
//         if macros::is_excute(agent) {
//             macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4.3, 21, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
//         }
//         else {
//         if macros::is_excute(agent) {
//             macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4.3, 21, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
//         }
//     }
// }
// }

unsafe extern "C" fn zelda_game_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 5.0);
    let rand = smash::app::sv_math::rand(hash40("fighter"), 8) as u64;
    if macros::is_excute(agent) {
        println!("{}",rand);
        if rand != 7{
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 20, 89, 0, 20, 4.2, 0.0, 3.0, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 361, 89, 0, 20, 3.0, 0.0, 3.0, 11.0, Some(0.0), Some(5.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        } else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 20, 89, 0, 20, 4.2, 0.0, 3.0, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 361, 89, 0, 20, 3.0, 0.0, 3.0, 11.0, Some(0.0), Some(5.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        println!("{}",rand);
        if rand != 7{
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 20, 100, 0, 20, 4.2, 0.0, 3.0, -11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 361, 100, 0, 20, 3.0, 0.0, 3.0, -9.0, Some(0.0), Some(7.0), Some(-4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        } else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 20, 100, 0, 20, 4.2, 0.0, 3.0, -11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 361, 100, 0, 20, 3.0, 0.0, 3.0, -9.0, Some(0.0), Some(7.0), Some(-4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// unsafe extern "C" fn zelda_effect_attacklw4(agent: &mut L2CAgentBase) {
//     if get_value_float(agent, *SO_VAR_FLOAT_LR) < 0.0 {
//         if macros::is_excute(agent) {
//             macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 5, -6.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
//         }
//         else {
//         if macros::is_excute(agent) {
//             macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4, 5, -6.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
//         }
//     }
// }
// }

unsafe extern "C" fn zelda_game_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 16.0);
    for _ in 0..4 {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 366, 100, 60, 0, 4.2, 0.0, 9.1, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 366, 100, 60, 0, 4.5, 0.0, 9.1, 17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
}
if macros::is_excute(agent) {
    let rand = smash::app::sv_math::rand(hash40("fighter"), 8) as u64;
    println!("{}",rand);
    if rand != 7{
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 110, 0, 37, 5.7, 0.0, 9.1, 10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 361, 110, 0, 37, 6.0, 0.0, 9.1, 17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    } else {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 361, 110, 0, 37, 5.7, 0.0, 9.1, 10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 361, 110, 0, 37, 6.0, 0.0, 9.1, 17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_death"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC); 
    }
}
wait(agent.lua_state_agent, 1.0);
if macros::is_excute(agent) {
    AttackModule::clear_all(agent.module_accessor);
}
}

// unsafe extern "C" fn zelda_effect_attacks4(agent: &mut L2CAgentBase) {
//     frame(agent.lua_state_agent, 6.0);
//     if macros::is_excute(agent) {
//         macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
//     }
//     frame(agent.lua_state_agent, 14.0);
//     if macros::is_excute(agent) {
//         macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(agent.lua_state_agent, 15.0);
//     if macros::is_excute(agent) {
//         macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_smash_spark"), Hash40::new("top"), 0, 10, 16, 0, 0, 0, 1, true);
//         macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("zelda_atk_flash_s"), Hash40::new("zelda_atk_flash_s"), Hash40::new("havel"), -0.5, 0, 0, 0, 0, 0, 1, true, *EF_FLIP_YZ);
//     }
// }

pub fn install() {
    Agent::new("zelda")
        .game_acmd("game_attack11", zelda_game_attack11)
        .game_acmd("game_attackhi4", zelda_game_attackhi4)
        .game_acmd("game_attacklw4", zelda_game_attacklw4)
        .game_acmd("game_attacks4", zelda_game_attacks4)
        // .game_acmd("effect_attackhi4", zelda_effect_attackhi4)
        // .game_acmd("effect_attacklw4", zelda_effect_attacklw4)
        // .game_acmd("effect_attacks4", zelda_effect_attacks4)
        .install();
}
