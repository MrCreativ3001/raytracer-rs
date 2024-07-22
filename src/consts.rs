pub mod window {
    pub const WINDOW_NAME: &str = "Raytracer";
    pub const WINDOW_SIZE: [f64; 2] = [1000.0, 800.0];
}

pub mod render {
    use crate::angle::Angle;
    use crate::consts::window::WINDOW_SIZE;

    // TODO: Find good values
    pub const FOV: Angle = Angle::from_degree(90.0);
    pub const CHANGE_ANGLE_PER_RAY: Angle = Angle::from_degree(0.25);
    // TODO: dynamically calculate these values in World::render
    pub const RAY_SCREEN_WIDTH: f64 =
        WINDOW_SIZE[0] / (FOV.degree() / CHANGE_ANGLE_PER_RAY.degree());
    pub const RAY_SCREEN_HEIGHT: f64 = WINDOW_SIZE[1];
    pub const CHANGE_SCREEN_X_PER_RAY: f64 = RAY_SCREEN_WIDTH;

    pub const NEAR_PLANE: f64 = 0.1;
    pub const FAR_PLANE: f64 = 25.0;

    pub const FLOOR_HEIGHT: f64 = 0.3;

    pub const MINIMAP_WIDTH: f64 = 100.0;
    pub const MINIMAP_HEIGHT: f64 = 100.0;
}

pub mod player {
    use crate::angle::Angle;

    pub const MOVE_SPEED: f64 = 0.01;
    pub const TURN_SPEED: Angle = Angle::from_degree(2.5);
}

pub mod controls {
    use piston::Key;

    pub const FORWARD: Key = Key::W;
    pub const BACKWARD: Key = Key::S;
    pub const TURN_RIGHT: Key = Key::D;
    pub const TURN_LEFT: Key = Key::A;
}
