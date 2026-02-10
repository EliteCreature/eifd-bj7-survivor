use crate::combat::PowerType;
use bevy::prelude::*;

/// Marker component for enemy entities
#[derive(Component)]
pub struct Enemy;

/// Combat capabilities for enemies
#[derive(Component)]
pub struct EnemyCombat {
    pub power_type: PowerType,
    pub cooldown: Timer,
}

impl Default for EnemyCombat {
    fn default() -> Self {
        Self {
            power_type: PowerType::Shadow, // Graveyard reaper uses shadow magic
            cooldown: Timer::from_seconds(2.0, TimerMode::Once), // Slower than player
        }
    }
}

impl EnemyCombat {
    pub fn new(power_type: PowerType, cooldown_seconds: f32) -> Self {
        Self {
            power_type,
            cooldown: Timer::from_seconds(cooldown_seconds, TimerMode::Once),
        }
    }
}

/// AI behavior state for enemies
#[derive(Component)]
pub struct AIBehavior {
    pub attack_range: f32,
    pub detection_range: f32,
}

impl Default for AIBehavior {
    fn default() -> Self {
        Self {
            attack_range: 150.0,    // Stop and attack within this range
            detection_range: 500.0, // Start following player within this range
        }
    }
}

impl AIBehavior {
    pub fn new(attack_range: f32, detection_range: f32) -> Self {
        Self {
            attack_range,
            detection_range,
        }
    }
}
