use crate::prelude::*;
mod player_input;
mod map_render;
mod entity_render;
mod random_move;
mod end_turn;
mod movement;
mod hud;
mod tooltips;
mod combat;
mod chasing;
mod fov;
mod use_item;

pub fn build_input_scheduler() -> Schedule{
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(hud::hud_system())
    .add_system(tooltips::tooltips_system())
    .add_system(fov::fov_system())
    .add_system(use_item::use_items_system())
    .build()
}
pub fn build_player_scheduler() -> Schedule{
    Schedule::builder()
    .add_system(combat::combat_system())
    .flush()
    .add_system(movement::movement_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(end_turn::end_turn_system())
    .add_system(hud::hud_system())
    .add_system(fov::fov_system())
    .build()
}
pub fn build_monster_scheduler() -> Schedule{
    Schedule::builder()
    .add_system(combat::combat_system())
    .flush()
    .add_system(chasing::chasing_system())
    .add_system(random_move::random_move_system()).flush()
    .add_system(movement::movement_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(end_turn::end_turn_system())
    .add_system(hud::hud_system())  
    .add_system(fov::fov_system())
    .build()
}