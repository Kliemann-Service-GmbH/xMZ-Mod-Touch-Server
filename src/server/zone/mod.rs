//! Zone der XMZModTouchServer Platform
//!
//! Eine Zone kann `n` Kombisensoren enthalten
//!
pub mod kombisensor;
pub mod zone;

pub use self::zone::{Zone, ZoneStatus};
pub use self::kombisensor::{Kombisensor, KombisensorType, KombisensorStatus};
