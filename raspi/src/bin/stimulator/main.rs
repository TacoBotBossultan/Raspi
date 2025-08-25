use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, poll, read},
    execute,
};
use std::{
    fs,
    io::{Write, stdin, stdout},
    path::Path,
    str::FromStr,
    sync::Arc,
};

use raspi::{
    chassis::{
        chassis_traits::{MotorIndex, Position},
        simulated_chassis::SimulatedChassis,
    },
    navigation_computing::navigation_computer::NavigationComputer,
    utils::{
        logging::clear_screen_and_return_to_zero,
        stimulator_config::{
            CONFIG_FILE_PATH, Config, KBRD_READ_TIME, MotorEfficiencies, StartConfig, TargetConfig,
            read_config_from_file, set_motor_efficiencies_from_config, set_motor_efficiency,
            wait_for_confirmation_of_using_config,
        },
    },
};

use tokio::{
    sync::{self, Mutex},
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() {
    clear_screen_and_return_to_zero();

    let chassis: Arc<sync::Mutex<SimulatedChassis>> =
        Arc::new(sync::Mutex::new(SimulatedChassis::new()));
    let navigation_computer = Arc::new(NavigationComputer::new());

    if Path::new(CONFIG_FILE_PATH).exists() {
        match read_config_from_file() {
            Ok(parsed_config) => {
                println!(
                    "Buun ti-am scos config din {CONFIG_FILE_PATH:#?}, sigur sigur vrei sa-l folosesti pe asta?"
                );
                println!("{parsed_config:#?}");
                match wait_for_confirmation_of_using_config().await {
                    Ok(()) => {
                        set_chassis_and_target_from_config(
                            &chassis,
                            &navigation_computer,
                            parsed_config,
                        )
                        .await
                    }
                    Err(()) => {
                        read_set_and_save_chassis_and_target_from_keyboard(
                            &chassis,
                            &navigation_computer,
                        )
                        .await
                    }
                };
            }

            Err(err) => {
                println!("{err:#?}");
                read_set_and_save_chassis_and_target_from_keyboard(&chassis, &navigation_computer)
                    .await;
            }
        }
    } else {
        println!("Nu ti-am gasit niciun fisier de {CONFIG_FILE_PATH:#?}, lasa ca ti-l face acm");
        read_set_and_save_chassis_and_target_from_keyboard(&chassis, &navigation_computer).await;
    }

    clear_screen_and_return_to_zero();

    sleep(Duration::from_secs(1)).await;
    clear_screen_and_return_to_zero();

    let chassis: Arc<sync::Mutex<SimulatedChassis>> =
        Arc::new(sync::Mutex::new(SimulatedChassis::new()));
    let navigation_computer = Arc::new(NavigationComputer::new());

    if Path::new(CONFIG_FILE_PATH).exists() {
        match read_config_from_file() {
            Ok(parsed_config) => {
                println!(
                    "Buun ti-am scos config din {CONFIG_FILE_PATH:#?}, sigur sigur vrei sa-l folosesti pe asta?"
                );
                println!("{parsed_config:#?}");
                match wait_for_confirmation_of_using_config().await {
                    Ok(()) => {
                        set_chassis_and_target_from_config(
                            &chassis,
                            &navigation_computer,
                            parsed_config,
                        )
                        .await
                    }
                    Err(()) => {
                        read_set_and_save_chassis_and_target_from_keyboard(
                            &chassis,
                            &navigation_computer,
                        )
                        .await
                    }
                };
            }

            Err(err) => {
                println!("{err:#?}");
                read_set_and_save_chassis_and_target_from_keyboard(&chassis, &navigation_computer)
                    .await;
            }
        }
    } else {
        println!("Nu ti-am gasit niciun fisier de {CONFIG_FILE_PATH:#?}, lasa ca ti-l face acm");
        read_set_and_save_chassis_and_target_from_keyboard(&chassis, &navigation_computer).await;
    }

    clear_screen_and_return_to_zero();

    sleep(Duration::from_secs(1)).await;
    execute!(stdout(), Hide).unwrap();
    let join_handle = navigation_computer.clone().start(chassis.clone());
    println!("Running simulation, press q or x to quit...");
    let read_duration = Duration::from_millis(KBRD_READ_TIME);
    loop {
        if poll(read_duration).unwrap() {
            if let Event::Key(event) = read().unwrap() {
                if let KeyCode::Char('q') | KeyCode::Char('x') = event.code {
                    clear_screen_and_return_to_zero();
                    println!("Quitting.");
                    navigation_computer.stop().await;
                    let _ = join_handle.await;
                    sleep(Duration::from_millis(250)).await;
                    clear_screen_and_return_to_zero();
                    execute!(stdout(), Show).unwrap();
                    break;
                }
            }
        }
    }
}

fn read_from_input<T: FromStr>() -> T {
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<T>() {
                Ok(value) => {
                    return value;
                }
                Err(_) => {
                    println!("Please try again, maybe with a number this time.");
                }
            },
            Err(_) => {
                println!("Please try again.");
            }
        }
    }
}

fn read_and_set_motor_efficiency(
    sim_chassis: &mut SimulatedChassis,
    motor_index: &MotorIndex,
) -> u16 {
    println!("How effective is motor {motor_index:#?}?");
    loop {
        let motor_efficiency = read_from_input::<u16>();
        if set_motor_efficiency(sim_chassis, motor_index, motor_efficiency).is_ok() {
            return motor_efficiency;
        };
    }
}

async fn read_set_and_save_chassis_and_target_from_keyboard(
    chassis: &Arc<sync::Mutex<SimulatedChassis>>,
    navigation_computer: &Arc<NavigationComputer>,
) {
    let front_right_eff =
        read_and_set_motor_efficiency(&mut *chassis.lock().await, &MotorIndex::FrontRight);
    let front_left_eff =
        read_and_set_motor_efficiency(&mut *chassis.lock().await, &MotorIndex::FrontLeft);
    let back_left_eff =
        read_and_set_motor_efficiency(&mut *chassis.lock().await, &MotorIndex::BackLeft);
    let back_right_eff =
        read_and_set_motor_efficiency(&mut *chassis.lock().await, &MotorIndex::BackRight);

    let motor_efficiencies = MotorEfficiencies::new(
        front_right_eff,
        front_left_eff,
        back_left_eff,
        back_right_eff,
    );

    let start_config: StartConfig;

    println!("Please input the starting X coordinate:");
    let start_x = read_from_input::<i32>();
    println!("Please input the starting Y coordinate:");
    let start_y = read_from_input::<i32>();
    loop {
        println!("Please input the current orientation:");

        let start_orientation = read_from_input::<u16>();

        match Position::new(None, start_x, start_y, start_orientation) {
            Ok(position) => match (*chassis.lock().await).set_position(position) {
                Ok(output) => {
                    println!("{output:#?}");
                    start_config = StartConfig::new(start_x, start_y, start_orientation);
                    break;
                }
                Err(output) => println!("{output:#?}"),
            },
            Err(output) => println!("{output:#?}"),
        }
    }

    let target_config: TargetConfig;

    println!("Please input the target X coordinate:");
    let target_x = read_from_input::<u32>();
    println!("Please input the target Y coordinate:");
    let target_y = read_from_input::<u32>();
    loop {
        println!("Please input the target orientation:");
        let target_orientation = read_from_input::<u16>();
        match Position::new(None, target_x, target_y, target_orientation) {
            Ok(position) => match navigation_computer.go_to_position(position).await {
                Ok(output) => {
                    println!("{output:#?}");
                    target_config = TargetConfig::new(target_x, target_y, target_orientation);
                    break;
                }
                Err(output) => println!("{output:#?}"),
            },
            Err(output) => println!("{output:#?}"),
        }
    }

    let mut file = fs::File::new(CONFIG_FILE_PATH).unwrap();
    let _ = file.write_all(
        toml::to_string_pretty(&Config::new(
            start_config,
            target_config,
            motor_efficiencies,
        ))
        .unwrap()
        .as_bytes(),
    );
}

async fn set_chassis_and_target_from_config(
    chassis: &Arc<sync::Mutex<SimulatedChassis>>,
    navigation_computer: &Arc<NavigationComputer>,
    config: Config,
) {
    set_motor_efficiencies_from_config(chassis, &config).await;

    //start
    match Position::new(
        None,
        config.start().x(),
        config.start().y(),
        config.start().orientation(),
    ) {
        Ok(position) => match (*chassis.lock().await).set_position(position) {
            Ok(output) => {
                println!("{output:#?}");
            }
            Err(output) => println!("{output:#?}"),
        },
        Err(output) => println!("{output:#?}"),
    }

    //target
    match Position::new(
        None,
        config.target().x(),
        config.target().y(),
        config.target().orientation(),
    ) {
        Ok(position) => match navigation_computer.go_to_position(position).await {
            Ok(output) => {
                println!("{output:#?}");
            }
            Err(output) => println!("{output:#?}"),
        },
        Err(output) => println!("{output:#?}"),
    }
}
