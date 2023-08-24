const PLAY_AREA_WIDTH: i32 = 100;
const PLAY_AREA_HEIGHT: i32 = 100;
const TANK_MARGIN: i32 = 5; // Margin around tanks
const MAX_SHOT_VELOCITY: i32 = 10; // Maximum allowed shot velocity

pub struct Tank {
    x: i32,
    y: i32,
}

pub struct Shot {
    x: i32,
    y: i32,
    velocity: i32,
    status: i32,
}

impl Tank {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    fn is_within_boundaries(&self) -> bool {
        self.x >= TANK_MARGIN 
        && self.x < PLAY_AREA_WIDTH - TANK_MARGIN 
        && self.y >= TANK_MARGIN 
        && self.y < PLAY_AREA_HEIGHT - TANK_MARGIN
    }

    fn is_colliding_with(&self, other: &Tank) -> bool {
        (self.x - other.x).abs() <= TANK_MARGIN * 2 && (self.y - other.y).abs() <= TANK_MARGIN * 2
    }
}

impl Shot {
    fn new(x: i32, y: i32, velocity: i32, status: i32) -> Self {
        Self { x, y, velocity, status }
    }
    
    fn is_within_boundaries(&self) -> bool {
        self.x >= 0 
        && self.x < PLAY_AREA_WIDTH 
        && self.y >= 0 
        && self.y < PLAY_AREA_HEIGHT
    }

    fn has_valid_velocity(&self) -> bool {
        self.velocity.abs() <= MAX_SHOT_VELOCITY
    }
}

#[no_mangle]
pub extern "C" fn verify_game_state(tank1_x: i32, tank1_y: i32, 
                                    tank2_x: i32, tank2_y: i32, 
                                    shot1_x: i32, shot1_y: i32, shot1_velocity: i32, shot1_status: i32, 
                                    shot2_x: i32, shot2_y: i32, shot2_velocity: i32, shot2_status: i32) -> i32 {

    let tank1 = Tank::new(tank1_x, tank1_y);
    let tank2 = Tank::new(tank2_x, tank2_y);
    let shot1 = Shot::new(shot1_x, shot1_y, shot1_velocity, shot1_status);
    let shot2 = Shot::new(shot2_x, shot2_y, shot2_velocity, shot2_status);

    if !tank1.is_within_boundaries() || !tank2.is_within_boundaries() || tank1.is_colliding_with(&tank2) {
        return 0;
    }

    if shot1.status == 1 && (!shot1.is_within_boundaries() || !shot1.has_valid_velocity()) {
        return 0;
    }

    if shot2.status == 1 && (!shot2.is_within_boundaries() || !shot2.has_valid_velocity()) {
        return 0;
    }

    return 1;
}
