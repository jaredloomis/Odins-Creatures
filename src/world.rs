//use std::ops::Sub;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Eq};

use bevy::{
    prelude::*,
    prelude::Vec2

};

/* Entities */

// XXX Do we need custom Entities??

/* Components */

#[derive(Component, Default, Sub, Add, Mul, AddAssign, SubAssign, MulAssign, DivAssign, Div, Deref, DerefMut, Clone, Copy)]
pub struct Position(pub Vec2);

#[derive(Component, Default)]
pub struct MovementPotential {
    // Tiles per second
    pub speed: f32,
    // Built up movement potential from previous updates
    pub potential: f32,
}

#[derive(Component, Default)]
pub struct MultiIntention(pub dashmap::DashMap<Intention, u8>);

#[derive(Eq, Ord)]
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

#[derive(Eq, Ord)]
pub enum DwarfCommand {
  // Dwarf told dwarf to go to position
  GoToPosition {
    destination: Position
  },
}

#[derive(Eq, Ord)]
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