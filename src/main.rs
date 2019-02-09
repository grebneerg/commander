use std::{thread, time};
use wpilib::{dio::DigitalOutput, ds, RobotBase, pwm::PwmSpeedController};
use wpilib_sys::bindings::{HAL_GetJoystickAxes, HAL_JoystickAxes};

// fn get_joystick_axis(u8 port, u8 axis) -> Option<f32> {

// }

fn main() {
    println!("main running");

    let robot = RobotBase::new().expect("HAL FAILED");

    RobotBase::start_competition();

    let ds = robot.make_ds();

    // These are actually some old model of victors but it seems to work.
    let mut right1 = PwmSpeedController::new_talon(0).expect("failed making victor");
    let mut right2 = PwmSpeedController::new_talon(1).expect("failed making victor");
    let mut left1 = PwmSpeedController::new_talon(2).expect("failed making victor");
    let mut left2 = PwmSpeedController::new_talon(3).expect("failed making victor");

    loop {
        let state = ds.robot_state();

        println!(
            "State: {}",
            match state {
                ds::RobotState::Disabled => "disabled",
                ds::RobotState::Teleop => "teleop",
                ds::RobotState::Test => "test",
                ds::RobotState::Autonomous => "auto",
                ds::RobotState::EStop => "estop",
            }
        );

        match state {
            ds::RobotState::Teleop => {
                right1.set(0.5);
                right2.set(0.5);
                left1.set(0.5);
                left2.set(0.5);
            },
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
