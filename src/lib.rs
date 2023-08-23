#[no_mangle]
pub extern "C" fn verify_game_state(tank1_x: i32, tank1_y: i32, 
                                    tank2_x: i32, tank2_y: i32, 
                                    shot1_x: i32, shot1_y: i32, shot1_status: i32, 
                                    shot2_x: i32, shot2_y: i32, shot2_status: i32) -> i32 {

    if !is_within_boundaries(tank1_x, tank1_y) || 
       !is_within_boundaries(tank2_x, tank2_y) {
        return 0;
    }

    if shot1_status == 1 && !is_within_boundaries(shot1_x, shot1_y) {
        return 0;
    }

    if shot2_status == 1 && !is_within_boundaries(shot2_x, shot2_y) {
        return 0;
    }

    return 1;
}

fn is_within_boundaries(x: i32, y: i32) -> bool {
    x >= 0 && x < 100 && y >= 0 && y < 100
}
