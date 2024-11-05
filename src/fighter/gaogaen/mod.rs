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

static mut count : i32 = 0;

unsafe extern "C" fn gaogaen_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        if count != 0 {
            smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 3.0);
            println!("{}", count);
            count -= 1;
        }
        if count == 1 {
            smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 1.0);
        }
        if StatusModule::status_kind(fighter.module_accessor) == 491 {
            count = 500;
        }
    }
}

pub fn install() {
    Agent::new("gaogaen")
        .on_line(Main, gaogaen_frame)
        .install();
}