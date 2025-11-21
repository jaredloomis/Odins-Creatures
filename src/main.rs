use bevy::{
    prelude::*,
};

mod world;
mod breakout;

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const DWARF_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const DWARF_SIZE: f32 = 50.0;
const DWARF_STARTING_POSITION: Vec2 = Vec2::new(0.0, -50.0);
const DWARF_SPEED: f32 = 10.0;
const DWARF_SPAWN_RADIUS: f32 = 50.0;
const INITIAL_DWARF_COUNT: u32 = 10;

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

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Camera
    commands.spawn(Camera2d);

    // Dwarves
    for i in 0..INITIAL_DWARF_COUNT {
      let rand_delta_x = rand::random::<f32>() * DWARF_SPAWN_RADIUS - DWARF_SPAWN_RADIUS;
      let rand_delta_y = rand::random::<f32>() * DWARF_SPAWN_RADIUS - DWARF_SPAWN_RADIUS;
      let rand_delta = Vec2::new(rand_delta_x, rand_delta_y);

      let intentions: dashmap::DashMap<String, world::Intention> = dashmap::DashMap::new();
      intentions.insert("".to_string(), world::Intention::FromPlayer(world::PlayerCommand::GoToPosition { destination: world::Position(Vec2::new(500.0, 500.0)) }));

      commands.spawn((
        world::Dwarf,
        world::Position(DWARF_STARTING_POSITION + rand_delta),
        world::MovementPotential {
          speed: 0.0, //DWARF_SPEED,
          potential: 0.0,
        },
        world::MultiIntention(intentions),
        Sprite::from_color(DWARF_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, -30., 0.0),
            scale: Vec2::splat(DWARF_SIZE).extend(1.0),
            ..default()
        },
      ));     
    }
}

fn update_dwarves_intention(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&world::MultiIntention, &mut world::Position, &world::MovementPotential)>,
) {
    for (world::MultiIntention(intents), mut position, movement_potential) in &mut query {
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
