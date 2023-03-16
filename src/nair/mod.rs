use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "edge", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn edge_attackairn(fighter: &mut L2CAgentBase) {
frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 0.75);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 8.0, 40, 100, 0, 25, 3.8, 3.0, 0.0, 0.0, Some(3.0), Some(26.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "edge", script = "effect_attackairn", category = ACMD_EFFECT, low_priority )]
unsafe fn edge_attackairn_fx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordr1"), 0.0, 0.0, 0.0, 0.0, 180, -90, 1, true);
		macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.2);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 12, Hash40::new("swordr1"), -4.0, 0.0, -0.6, Hash40::new("swordr1"), 29.0, 0.0, -1.4, true, Hash40::new("null"), Hash40::new("swordr1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
	}
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        edge_attackairn,
		edge_attackairn_fx,
    );
}
