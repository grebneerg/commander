use std::{thread, time};
use wpilib::{
    dio::DigitalInput,
    ds::{JoystickAxis, JoystickPort, RobotState},
    pwm::PwmSpeedController,
    RobotBase,
};

fn main() {
    println!("main running");

    let robot = RobotBase::new().expect("HAL FAILED");

    RobotBase::start_competition();

    let ds = robot.make_ds();

    // Logitech attack3 joysticks
    let left_stick = JoystickPort::new(0).expect("bad port");
    let right_stick = JoystickPort::new(1).expect("bad port");

    let x_axis = JoystickAxis::new(0).expect("bad axis");
    let y_axis = JoystickAxis::new(1).expect("bad axis");

    let front_bumper = DigitalInput::new(0).expect("oh no");

    // These are actually some old model of victors but it seems to work.
    let mut right1 = PwmSpeedController::new_talon(0).expect("failed making victor");
    let mut right2 = PwmSpeedController::new_talon(1).expect("failed making victor");
    let mut left1 = PwmSpeedController::new_talon(2).expect("failed making victor");
    left1.set_inverted(true);
    let mut left2 = PwmSpeedController::new_talon(3).expect("failed making victor");
    left2.set_inverted(true);

    loop {
        let state = ds.robot_state();

        let mut left = 0.5
            * if let Ok(val) = ds.stick_axis(left_stick, y_axis) {
                val
            } else {
                0.0
            };
        let mut right = 0.5
            * if let Ok(val) = ds.stick_axis(right_stick, y_axis) {
                val
            } else {
                0.0
            };

        println!("Switch pos {}", front_bumper.get().unwrap_or(true));

        // if front_bumper.get().unwrap_or(true) {
        //     if left < 0 {
        //         left = 0
        //     }
        //     if right < 0 {
        //         right = 0
        //     }
        // }

        println!("l:{},r:{}", left, right);

        match state {
            RobotState::Teleop => {
                right1.set(right.into());
                right2.set(right.into());
                left1.set(left.into());
                left2.set(left.into());
            }
            _ => {
                right1.set(0.0);
                right2.set(0.0);
                left1.set(0.0);
                left2.set(0.0);
            }
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}
