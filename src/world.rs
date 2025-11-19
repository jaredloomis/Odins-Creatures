use std::ops::Sub;

use bevy::{
    prelude::*,
};

/* Entities */

// XXX Do we need custom Entities??

/* Components */

#[derive(Component, Default, Sub)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Default)]
pub struct MovementPotential {
    // Tiles per second
    pub speed: f32,
    // Built up movement potential from previous updates
    pub potential: f32,
}

#[derive(Component, Default)]
pub struct MultiIntention(pub Vec<Intention>);

pub enum Intention {
  FromPlayer(PlayerCommand),
  FromSelf(SelfIntention),
  FromOtherDwarf(DwarfCommand),
}

impl Default for Intention {
  fn default() -> Self {
    Intention::FromSelf(SelfIntention::Relax)
  }
}

pub enum DwarfCommand {
  // Dwarf told dwarf to go to position
  GoToPosition {
    destination: Position
  },
}

pub enum SelfIntention {
  Relax,
  Eat,
  // Dwarf told dwarf to go to position
  GoToPosition {
    destination: Position
  },
}

pub enum PlayerCommand {
  // Player told dwarf to go to position
  GoToPosition {
    destination: Position
  },
  Work {
    command: WorkCommand
  }
}

pub enum WorkCommand {
  Mine {
    position: Option<Position>
  }
}

#[derive(Component)]
#[require(Position, MultiIntention)]
pub struct Dwarf;