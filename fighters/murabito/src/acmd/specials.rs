
use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::utility::*;
use smash_script::*;
use smashline::*;
use smashline::*;
use smash_script::macros::*;
use crate::utils::hdr;
use crate::vars::*;

#[acmd_script( agent = "murabito", script = "game_speciallw1" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_special_lw1_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MURABITO_STATUS_SPECIAL_LW_PLANT_FLAG_CHECK_PLANT);
        FT_MOTION_RATE(fighter, 0.661);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SEED, false, 0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 72, 20, 0, 80, 4.0, 0.0, 3.0, 12.0, Some(0.0), Some(3.0), Some(3.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MURABITO_STATUS_SPECIAL_LW_PLANT_FLAG_PLANT);
    }
    
}

#[acmd_script( agent = "murabito", script = "game_speciallw2" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_special_lw2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_WATER);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 5.0, 69, 80, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_WATER);
    }
    
}

#[acmd_script( agent = "murabito", script = "game_specialairlw2" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_special_air_lw2_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_WATER);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 5.0, 69, 80, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_WATER);
    }
    
}

#[acmd_script( agent = "murabito", script = "game_speciallw3hit" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_special_lw3_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !ItemModule::is_have_item(boma, 0){
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_WOOD), 0, 0, false, false);
        }
    }
    
}

#[acmd_script( agent = "murabito", script = "game_specialairlw3hit" , category = ACMD_GAME , low_priority)]
unsafe fn murabito_special_air_lw3_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !ItemModule::is_have_item(boma, 0){
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_WOOD), 0, 0, false, false);
        }
    }
    
}


pub fn install() {
    install_acmd_scripts!(
        murabito_special_lw1_game,
        murabito_special_lw2_game,
        murabito_special_air_lw2_game,
        murabito_special_lw3_hit_game,
        murabito_special_air_lw3_hit_game,
    );
}
