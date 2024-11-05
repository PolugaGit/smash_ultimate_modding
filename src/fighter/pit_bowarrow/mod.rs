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

unsafe extern "C" fn pit_game_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        println!("arrow go pew");
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 50.0, 60, 88, 0, 4, 1.3, 0.0, 0.0, -1.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        // AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("pit_bowarrow")
        .game_acmd("game_fly", pit_game_fly)
        .game_acmd("pit_pa_fly_arrow", pit_game_fly)
        .install();
}