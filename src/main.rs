use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::world::MultiIntention;

mod world;
mod breakout;

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            update_dwarves_intention
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Dwarves
    commands.spawn(world::Dwarf);
}

fn update_dwarves_intention(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&world::MultiIntention, &mut world::Position, &world::MovementPotential)>,
) {
    for (MultiIntention(intents), mut position, movement_potential) in &mut query {
      intents.iter().for_each(|target|
        match &target {
          &world::Intention::FromPlayer(cmd) => match cmd {
            world::PlayerCommand::GoToPosition { destination } => {
              let update_potential = (destination - position).length();
              let total_potential = 
                movement_potential.potential + 
              if movement_potential.potential
              *position = (*p)
            },
            _ => {},
          }
          _ => {},
        }
      );
    }
}

fn update_intention(intention: world::Intention) {
  match intention {
    world::Intention::FromPlayer(player_command) => {}
    world::Intention::FromSelf(self_intention) => {}
    world::Intention::FromOtherDwarf(dwarf_command) => {}
  }
}
