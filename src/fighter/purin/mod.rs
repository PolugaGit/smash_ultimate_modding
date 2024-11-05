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


unsafe extern "C" fn purin_game_speciallwl(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 20.0, 88, 66, 0, 100, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        //JostleModule::set_status(agent.module_accessor, true);
    }
}

unsafe extern "C" fn purin_effect_speciallwl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("purin_nemuru_start"), Hash40::new("top"), 0, 9, 6.5, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FLW_POS(agent, Hash40::new("sys_sleep"), Hash40::new("body"), 0, 3, -6, 0, 0, 0, 1, false);
    }
//     frame(agent.lua_state_agent, 40.0);
//     for _ in 0..3 {
//     if macros::is_excute(agent) {
//         macros::FLASH(agent, 0.502, 0.314, 0.392, 0.196);
//     }
//     wait(agent.lua_state_agent, 2.0);
//     if macros::is_excute(agent) {
//         macros::FLASH_FRM(agent, 12, 0.941, 0.235, 0.549, 0.392);
//     }
//     wait(agent.lua_state_agent, 12.0);
//     if macros::is_excute(agent) {
//         macros::FLASH_FRM(agent, 12, 0.941, 0.118, 0.549, 0);
//     }
//     wait(agent.lua_state_agent, 12.0);
//     if macros::is_excute(agent) {
//         macros::COL_NORMAL(agent);
//     }
//     wait(agent.lua_state_agent, 6.0);
// }
// frame(agent.lua_state_agent, 130.0);
// if macros::is_excute(agent) {
//     macros::EFFECT_FLW_POS(agent, Hash40::new("sys_sleep"), Hash40::new("body"), 0, 3, -6, 0, 0, 0, 1, false);
//     macros::FLASH(agent, 0.502, 0.314, 0.392, 0.196);
// }
// wait(agent.lua_state_agent, 2.0);
// if macros::is_excute(agent) {
//     macros::FLASH_FRM(agent, 12, 0.941, 0.235, 0.549, 0.392);
// }
// wait(agent.lua_state_agent, 12.0);
// if macros::is_excute(agent) {
//     macros::FLASH_FRM(agent, 12, 0.941, 0.118, 0.549, 0);
// }
// wait(agent.lua_state_agent, 12.0);
// if macros::is_excute(agent) {
//     macros::COL_NORMAL(agent);
// }
// frame(agent.lua_state_agent, 185.0);
}

unsafe extern "C" fn purin_expression_speciallwl(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
    }
    // frame(agent.lua_state_agent, 34.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_sleep"), 30, true, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    // frame(agent.lua_state_agent, 134.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_sleep"), 30, true, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    // frame(agent.lua_state_agent, 187.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    // frame(agent.lua_state_agent, 210.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    // frame(agent.lua_state_agent, 223.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
}

unsafe extern "C" fn purin_game_speciallwr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 20.0, 88, 66, 0, 100, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        // JostleModule::set_status(agent.module_accessor, true);
    }
}

unsafe extern "C" fn purin_effect_speciallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("purin_nemuru_start"), Hash40::new("top"), 0, 9, 6.5, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FLW_POS(agent, Hash40::new("sys_sleep"), Hash40::new("body"), 0, 3, 6, 0, 0, 0, 1, false);
    }
//     frame(agent.lua_state_agent, 40.0);
//     for _ in 0..3 {
//     if macros::is_excute(agent) {
//         macros::FLASH(agent, 0.502, 0.314, 0.392, 0.196);
//     }
//     wait(agent.lua_state_agent, 2.0);
//     if macros::is_excute(agent) {
//         macros::FLASH_FRM(agent, 12, 0.941, 0.235, 0.549, 0.392);
//     }
//     wait(agent.lua_state_agent, 12.0);
//     if macros::is_excute(agent) {
//         macros::FLASH_FRM(agent, 12, 0.941, 0.118, 0.549, 0);
//     }
//     wait(agent.lua_state_agent, 12.0);
//     if macros::is_excute(agent) {
//         macros::COL_NORMAL(agent);
//     }
//     wait(agent.lua_state_agent, 6.0);
// }
// frame(agent.lua_state_agent, 130.0);
// if macros::is_excute(agent) {
//     //macros::EFFECT_FLW_POS(agent, Hash40::new("sys_sleep"), Hash40::new("body"), 0, 3, 6, 0, 0, 0, 1, false);
//     macros::FLASH(agent, 0.502, 0.314, 0.392, 0.196);
// }
// wait(agent.lua_state_agent, 2.0);
// if macros::is_excute(agent) {
//     macros::FLASH_FRM(agent, 12, 0.941, 0.235, 0.549, 0.392);
// }
// wait(agent.lua_state_agent, 12.0);
// if macros::is_excute(agent) {
//     macros::FLASH_FRM(agent, 12, 0.941, 0.118, 0.549, 0);
// }
// wait(agent.lua_state_agent, 12.0);
// if macros::is_excute(agent) {
//     macros::COL_NORMAL(agent);
// }
// frame(agent.lua_state_agent, 185.0);
}

unsafe extern "C" fn purin_expression_speciallwr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
    }
    // frame(agent.lua_state_agent, 34.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_sleep"), 30, true, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    // frame(agent.lua_state_agent, 134.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_sleep"), 30, true, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    // frame(agent.lua_state_agent, 187.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    // frame(agent.lua_state_agent, 210.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
    // frame(agent.lua_state_agent, 223.0);
    // if macros::is_excute(agent) {
    //     ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    // }
}

pub fn install() {
    Agent::new("purin")
        .game_acmd("game_speciallwl", purin_game_speciallwl)
        .effect_acmd("effect_speciallwl", purin_effect_speciallwl)
        .expression_acmd("exxpression_speciallwl", purin_expression_speciallwl)
        .game_acmd("game_speciallwr", purin_game_speciallwr)
        .effect_acmd("effect_speciallwr", purin_effect_speciallwr)
        .expression_acmd("exxpression_speciallwr", purin_expression_speciallwr)
        .install();
}