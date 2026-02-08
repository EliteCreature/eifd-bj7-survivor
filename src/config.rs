/// Camera configuration
pub mod camera {
    /// How fast the camera interpolates toward the player (higher = snappier)
    pub const CAMERA_LERP_SPEED: f32 = 6.0;

    /// Z position for the camera (must be high to see all layers)
    pub const CAMERA_Z: f32 = 1000.0;
}
