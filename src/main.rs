use std::{thread, time};
use wpilib::{dio::DigitalOutput, ds, RobotBase};

fn main() {
    println!("main running");

    let robot = RobotBase::new().expect("HAL FAILED");

    RobotBase::start_competition();

    let ds = robot.make_ds();

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

        thread::sleep(time::Duration::from_millis(100));
    }
}
