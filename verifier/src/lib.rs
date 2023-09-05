const PLAY_AREA_WIDTH: i32 = 100;
const PLAY_AREA_HEIGHT: i32 = 100;
const TANK_MARGIN: i32 = 5;
const MAX_SHOT_VELOCITY: i32 = 10;
const INITIAL_HEALTH: i32 = 100;
const SHOT_DAMAGE: i32 = 25;

pub struct Tank {
    x: i32,
    y: i32,
    health: i32,
}

pub struct Shot {
    x: i32,
    y: i32,
    velocity: i32,
    status: i32,
}

impl Tank {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, health: INITIAL_HEALTH }
    }

    fn is_within_boundaries(&self) -> bool {
        self.x >= TANK_MARGIN && self.x < PLAY_AREA_WIDTH - TANK_MARGIN && self.y >= TANK_MARGIN && self.y < PLAY_AREA_HEIGHT - TANK_MARGIN
    }

    fn is_colliding_with(&self, other: &Tank) -> bool {
        (self.x - other.x).abs() <= TANK_MARGIN * 2 && (self.y - other.y).abs() <= TANK_MARGIN * 2
    }

    fn is_hit_by(&mut self, shot: &Shot) {
        if (self.x - shot.x).abs() <= TANK_MARGIN && (self.y - shot.y).abs() <= TANK_MARGIN {
            self.health -= SHOT_DAMAGE;
        }
    }
}

impl Shot {
    fn new(x: i32, y: i32, velocity: i32, status: i32) -> Self {
        Self { x, y, velocity, status }
    }

    fn is_within_boundaries(&self) -> bool {
        self.x >= 0 && self.x < PLAY_AREA_WIDTH && self.y >= 0 && self.y < PLAY_AREA_HEIGHT
    }

    fn has_valid_velocity(&self) -> bool {
        self.velocity.abs() <= MAX_SHOT_VELOCITY
    }
}

#[no_mangle]
pub extern "C" fn verify_game_state(
    tank1_x: i32, tank1_y: i32,
    tank2_x: i32, tank2_y: i32,
    shot1_x: i32, shot1_y: i32, shot1_velocity: i32, shot1_status: i32,
    shot2_x: i32, shot2_y: i32, shot2_velocity: i32, shot2_status: i32
) -> i32 {
    let mut tank1 = Tank::new(tank1_x, tank1_y);
    let mut tank2 = Tank::new(tank2_x, tank2_y);
    let shot1 = Shot::new(shot1_x, shot1_y, shot1_velocity, shot1_status);
    let shot2 = Shot::new(shot2_x, shot2_y, shot2_velocity, shot2_status);

    let mut success: i32 = 1;
    let mut tank1_health = tank1.health;
    let mut tank2_health = tank2.health;

    if !tank1.is_within_boundaries() || !tank2.is_within_boundaries() || tank1.is_colliding_with(&tank2) {
        success = 0;
        tank1_health = 0;
        tank2_health = 0;
        return (success << 30) | (tank1_health << 20) | (tank2_health << 10);
    }

    if shot1.status == 1 {
        if !shot1.is_within_boundaries() || !shot1.has_valid_velocity() {
            success = 0;
            tank1_health = 0;
            tank2_health = 0;
            return (success << 30) | (tank1_health << 20) | (tank2_health << 10);
        }
        tank2.is_hit_by(&shot1);
    }

    if shot2.status == 1 {
        if !shot2.is_within_boundaries() || !shot2.has_valid_velocity() {
            success = 0;
            tank1_health = 0;
            tank2_health = 0;
            return (success << 30) | (tank1_health << 20) | (tank2_health << 10);
        }
        tank1.is_hit_by(&shot2);
    }

    tank1_health = tank1.health;
    tank2_health = tank2.health;

    (success << 30) | (tank1_health << 20) | (tank2_health << 10)
}
