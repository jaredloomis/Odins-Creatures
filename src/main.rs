use bevy::{
    prelude::*,
};

use crate::world::MultiIntention;

mod world;
mod breakout;

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const DWARF_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
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
    commands.spawn((
      world::Dwarf,
      world::Position(Vec2::new(10.0, 10.0)),
      world::MovementPotential {
        speed: 10.0,
        potential: 0.0,
      },
      world::MultiIntention(vec![world::Intention::FromPlayer(world::PlayerCommand::GoToPosition { destination: world::Position(Vec2::new(500.0, 500.0)) })]),
      Sprite {
          color: DWARF_COLOR,
          ..default()
      },
    ));
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
              println!("{}, {}", position.x, position.y);
              let update_potential = movement_potential.speed * time.delta_secs();
              let total_potential = update_potential + movement_potential.potential;
              let direction = (*destination - *position).normalize();
              let delta: Vec2 = direction * total_potential;
              *position += world::Position(delta);
              //*position += update_potential * movement_potential;
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
