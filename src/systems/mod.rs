use crate::prelude::*;
mod breakable;
mod cleanup;
mod collisions;
mod dropping_loot;
mod movement;
mod player_input;
mod render;
mod turn_end;

pub fn build_player_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(render::render_system())
        .build()
}

pub fn build_player_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(breakable::breakable_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(movement::movement_system())
        .add_system(dropping_loot::dropping_loot_system())
        .flush()
        .add_system(cleanup::cleanup_system())
        .flush()
        .add_system(render::render_system())
        .add_system(turn_end::turn_end_system())
        .build()
}

pub fn build_npc_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(movement::movement_system())
        .add_system(dropping_loot::dropping_loot_system())
        .flush()
        .add_system(cleanup::cleanup_system())
        .flush()
        .add_system(render::render_system())
        .add_system(turn_end::turn_end_system())
        .build()
}
