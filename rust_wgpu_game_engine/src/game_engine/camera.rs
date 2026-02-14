use glam::{EulerRot, Mat4, Quat, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub pitch: f32, // up or down
    pub yaw: f32,   // left or right
    pub roll: f32,  // spin
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
        }
    }

 pub fn build_view_matrix(&self) -> Mat4 {
    let rotation = Quat::from_euler(EulerRot::XYZ, self.pitch, self.yaw, self.roll);
    // The view matrix is the inverse of where the camera "is" in the world
    Mat4::from_rotation_translation(rotation, self.position).inverse()
}

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn move_camera(&mut self, direction: Vec3) {
        self.position += direction;
    }
}
