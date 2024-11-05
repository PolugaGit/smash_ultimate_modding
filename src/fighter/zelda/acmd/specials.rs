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

unsafe extern "C" fn zelda_game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 25.0);
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        let rand = smash::app::sv_math::rand(hash40("agent"), 12) as u64;
        println!("{}",rand);
        if rand == 0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor), y: -200.0, z: PostureModule::pos_z(agent.module_accessor) });
        } else if rand == 1 || rand == 2 {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 700.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 700.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            }
            wait(agent.lua_state_agent, 10.0);
            if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            }
        } else if rand == 3 || rand == 4 || rand == 5 {
            let mut randSize = (smash::app::sv_math::rand(hash40("agent"), 10) as f32) / 5 as f32;
            if randSize == 0 as f32 {
                randSize = 0.1 as f32
            }
            println!("{}", PostureModule::scale(agent.module_accessor));
            println!("{}", randSize);
            PostureModule::set_scale(agent.module_accessor, randSize, false);
        } else if rand == 6 || rand == 7{
            let items = [
                ITEM_KIND_BEAMSWORD,
                ITEM_KIND_BOOMERANG,
                ITEM_KIND_CAPSULE,
                ITEM_KIND_CHICKEN,
                ITEM_KIND_DEATHSCYTHE,
                ITEM_KIND_DEKU,
                ITEM_KIND_HONEYCOMB,
                ITEM_KIND_MASTERBALL,
                ITEM_KIND_BANANAGUN,
                ITEM_KIND_POKEBALL,
                ITEM_KIND_POWBLOCK,
                ITEM_KIND_WARPSTAR,
                ITEM_KIND_WOOD,
                ITEM_KIND_SMOKESCREEN,
                ITEM_KIND_RAYGUN,
                ITEM_KIND_SOCCERBALL,
            ];
            let randItem = smash::app::sv_math::rand(hash40("agent"), 16) as usize;
            println!("{}", randItem);
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*items[randItem]), 0, 0, false, false);
        } else if rand == 8 || rand == 9 {
            DamageModule::add_damage(agent.module_accessor, 20.0, 0);
        } else {
            DamageModule::heal(agent.module_accessor, -20.0, 0);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    frame(agent.lua_state_agent, 43.0);
}

unsafe extern "C" fn zelda_game_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 25.0);
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        let rand = smash::app::sv_math::rand(hash40("agent"), 12) as u64;
        println!("{}",rand);
        if rand == 0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor), y: -200.0, z: PostureModule::pos_z(agent.module_accessor) });
        } else if rand == 1 || rand == 2 {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 700.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 700.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            }
            wait(agent.lua_state_agent, 10.0);
            if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            }
        } else if rand == 3 || rand == 4 || rand == 5 {
            let mut randSize = (smash::app::sv_math::rand(hash40("agent"), 10) as f32) / 5 as f32;
            if randSize == 0 as f32 {
                randSize = 0.1 as f32
            }
            println!("{}", PostureModule::scale(agent.module_accessor));
            println!("{}", randSize);
            PostureModule::set_scale(agent.module_accessor, randSize, false);
        } else if rand == 6 || rand == 7{
            let items = [
                ITEM_KIND_BEAMSWORD,
                ITEM_KIND_BOOMERANG,
                ITEM_KIND_CAPSULE,
                ITEM_KIND_CHICKEN,
                ITEM_KIND_DEATHSCYTHE,
                ITEM_KIND_DEKU,
                ITEM_KIND_HONEYCOMB,
                ITEM_KIND_MASTERBALL,
                ITEM_KIND_BANANAGUN,
                ITEM_KIND_POKEBALL,
                ITEM_KIND_POWBLOCK,
                ITEM_KIND_WARPSTAR,
                ITEM_KIND_WOOD,
                ITEM_KIND_SMOKESCREEN,
                ITEM_KIND_RAYGUN,
                ITEM_KIND_SOCCERBALL,
            ];
            let randItem = smash::app::sv_math::rand(hash40("agent"), 16) as usize;
            println!("{}", randItem);
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*items[randItem]), 0, 0, false, false);
        } else if rand == 8 || rand == 9 {
            DamageModule::add_damage(agent.module_accessor, 20.0, 0);
        } else {
            DamageModule::heal(agent.module_accessor, -20.0, 0);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    frame(agent.lua_state_agent, 43.0);
}

unsafe extern "C" fn zelda_effect_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_throw_trace"), Hash40::new("haver"), 0, 0, 1, 0, 0, 0, 0.5, true);
    }
}

unsafe extern "C" fn zelda_effect_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        println!("effect");
        macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_throw_trace"), Hash40::new("haver"), 0, 0, 1, 0, 0, 0, 0.5, true);
    }
}

unsafe extern "C" fn zelda_sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_zelda_appeal_h01"));
        macros::PLAY_SE(agent, Hash40::new("se_zelda_ware01"));
        macros::PLAY_SE(agent, Hash40::new("se_zelda_appeal_h01"));
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_zelda_ware01"));
    }
}

unsafe extern "C" fn zelda_sound_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_zelda_appeal_h01"));
        macros::PLAY_SE(agent, Hash40::new("se_zelda_ware01"));
        macros::PLAY_SE(agent, Hash40::new("se_zelda_appeal_h01"));
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_zelda_ware01"));
    }
}

pub fn install() {
    Agent::new("zelda")
        .game_acmd("game_specialn", zelda_game_specialn)
        .game_acmd("game_specialairn", zelda_game_specialairn)
        .effect_acmd("effect_specialn", zelda_effect_specialn)
        .effect_acmd("effect_specialairn", zelda_effect_specialairn)
        .sound_acmd("sound_specialn", zelda_sound_specialn)
        .sound_acmd("sound_specialairn", zelda_sound_specialairn)
        .install();
}