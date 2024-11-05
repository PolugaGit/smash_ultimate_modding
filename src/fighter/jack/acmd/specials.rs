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

unsafe extern "C" fn jack_game_specialn1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.1, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(agent) {
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    
    frame(agent.lua_state_agent, 26.0);
    
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_game_specialn2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        println!("hitbox");
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.5, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(agent) {
        println!("ground check");
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        println!("attack clear");
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_game_specialn3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.5, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 4.5, 9.0, Some(0.0), Some(4.5), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.5, 0.0, 4.5, 9.0, Some(0.0), Some(4.5), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 4.5, 9.0, Some(0.0), Some(4.5), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(agent) {
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_game_specialairn1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 3, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 3, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(agent) {
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_game_specialairn2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 3, 2.5, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 3, 2.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(agent) {
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_game_specialairn3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 3, 2.5, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 4.5, 9.0, Some(0.0), Some(4.5), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.5, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 3, 2.5, 0.0, 4.5, 9.0, Some(0.0), Some(4.5), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 4.5, 9.0, Some(0.0), Some(4.5), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(agent) {
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_game_specialn1_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 50, 0, 60, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    
    frame(agent.lua_state_agent, 18.0);
    
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_game_specialn2_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 50, 0, 60, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    
    frame(agent.lua_state_agent, 18.0);
    
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_game_specialn3_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 50, 0, 60, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    
    frame(agent.lua_state_agent, 18.0);
    
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_game_specialairn1_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 50, 0, 60, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 3, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(agent.lua_state_agent, 18.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_game_specialairn2_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 50, 0, 60, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 3, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(agent.lua_state_agent, 18.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 75, 0, 5, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 50, 0, 3, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 75, 0, 5, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 50, 0, 3, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_game_specialairn3_ex(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 50, 0, 60, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 3, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.2, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    frame(agent.lua_state_agent, 18.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn jack_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_LUGIA), 0, 0, false, false);
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n_2") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n_3") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n_2") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n_3") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n2") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n2_2") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n2_3") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n2") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n2_2") ||
        MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n2_3") {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if MotionModule::frame(fighter.module_accessor) > 8.0 && MotionModule::frame(fighter.module_accessor) < 18.0 {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }
        }
    }
}

pub fn install() {
    Agent::new("jack")
        .on_line(Main, jack_frame)
        .game_acmd("game_specialn1", jack_game_specialn1)
        .game_acmd("game_specialn2", jack_game_specialn2)
        .game_acmd("game_specialn3", jack_game_specialn3)
        .game_acmd("game_specialairn1", jack_game_specialairn1)
        .game_acmd("game_specialairn2", jack_game_specialairn2)
        .game_acmd("game_specialairn3", jack_game_specialairn3)
        .game_acmd("game_specialn1_ex", jack_game_specialn1_ex)
        .game_acmd("game_specialn2_ex", jack_game_specialn2_ex)
        .game_acmd("game_specialn3_ex", jack_game_specialn3_ex)
        .game_acmd("game_specialairn1_ex", jack_game_specialairn1_ex)
        .game_acmd("game_specialairn2_ex", jack_game_specialairn2_ex)
        .game_acmd("game_specialairn3_ex", jack_game_specialairn3_ex)
        .install();
}