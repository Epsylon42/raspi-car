#![allow(dead_code)]

use std::sync::Arc;

use std::time::{Duration, Instant};

#[macro_use]
extern crate rocket;
use rocket::tokio::sync::{Mutex, MutexGuard, MappedMutexGuard};
use rust_pigpio as gpio;

mod motor;

fn oneminute() -> Duration {
    Duration::from_secs_f32(60.0)
}

#[derive(Clone, serde::Deserialize)]
struct Config {
    drive: motor::Config,
    turn: motor::Config,
    #[serde(default="oneminute")]
    gpio_timeout: Duration,
}

struct GpioData {
    drive: motor::Motor,
    turn: motor::Motor,
    running_since: Instant,
}

struct StateData {
    config: Config,
    gpio: Mutex<Option<GpioData>>,
}

impl StateData {
    pub fn new(config: Config) -> Self {
        StateData {
            config,
            gpio: Mutex::new(None),
        }
    }

    pub fn ensure_gpio(&self) -> Option<MappedMutexGuard<GpioData>> {
        let gpio = self.gpio.try_lock();
        let mut gpio = match gpio {
            Ok(x) => x,
            Err(_) => return None,
        };

        if gpio.is_none() {
            match gpio::initialize() {
                Ok(_) => {
                    *gpio = Some(GpioData {
                        drive: motor::Motor::new(&self.config.drive),
                        turn: motor::Motor::new(&self.config.turn),
                        running_since: Instant::now(),
                    });
                }

                Err(e) => {
                    eprintln!(
                        "Count not create gpio. Running in mock mode. cause: {:?}",
                        e
                    );
                    return None;
                }
            };
        }

        Some(MutexGuard::map(gpio, |x| x.as_mut().unwrap()))
    }

    pub async fn stop_gpio(&self) {
        let mut gpio = self.gpio.lock().await;

        if gpio.is_some() {
            eprintln!("stopping gpio");
            gpio::terminate();
            *gpio = None;
        }
    }

    pub fn stop_gpio_if_timeout(&self) {
        let gpio = self.gpio.try_lock();
        let mut gpio = match gpio {
            Ok(x) => x,
            Err(_) => return,
        };

        if gpio.is_some() && (Instant::now() - gpio.as_ref().unwrap().running_since) > self.config.gpio_timeout {
            eprintln!("stopping gpio on timeout");
            gpio::terminate();
            *gpio = None;
        }
    }
}

impl Drop for StateData {
    fn drop(&mut self) {
        eprintln!("drop");
        let gpio = self.gpio.get_mut().take();
        if gpio.is_some() {
            drop(gpio);
            gpio::terminate();
        }
    }
}

type State = Arc<StateData>;

impl<'a> rocket::request::FromParam<'a> for motor::MotorState {
    type Error = String;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param {
            "f" | "r" => Ok(motor::MotorState::RunningP),
            "b" | "l" => Ok(motor::MotorState::RunningN),
            "s" => Ok(motor::MotorState::Stopped),
            "br" => Ok(motor::MotorState::Braking),

            _ => Err(String::from("unknown motor command")),
        }
    }
}

#[post("/api/motor/drive/<cmd>")]
fn drive(cmd: motor::MotorState, state: &rocket::State<State>) {
    eprintln!("drive - {:?}", cmd);

    if let Some(mut gpio) = state.ensure_gpio() {
        gpio.drive.set_state(cmd);
    }
}

#[post("/api/motor/turn/<cmd>")]
fn turn(cmd: motor::MotorState, state: &rocket::State<State>) {
    eprintln!("turn - {:?}", cmd);

    if let Some(mut gpio) = state.ensure_gpio() {
        gpio.turn.set_state(cmd);
    }
}

// #[get("/api/motor/drive/power")]
// async fn get_power(state: &rocket::State<State>) -> String {
//     if let Some(ref motor) = state.turn {
//         let motor = motor.lock().await;
//         motor.power().to_string()
//     } else {
//         String::from("100")
//     }
// }
//
// #[post("/api/motor/drive/power/<value>")]
// async fn set_power(value: u8, state: &rocket::State<State>) {
//     eprintln!("drive power = {}", value);
//
//     if let Some(ref motor) = state.turn {
//         let mut motor = motor.lock().await;
//         motor.set_power(value);
//     }
// }

#[launch]
async fn rocket() -> _ {
    let config: Config = serde_json::de::from_reader(
        std::fs::File::open(
            std::env::var("GPIOSRV_CONFIG")
                .ok()
                .unwrap_or(String::from("gpiosrv.json")),
        )
        .unwrap(),
    )
    .unwrap();
    let state = StateData::new(config.clone());
    let state = Arc::new(state);

    let s1 = state.clone();
    rocket::tokio::spawn(async move {
        let state = s1;

        loop {
            rocket::tokio::time::sleep(Duration::from_secs_f32(config.drive.shutdown_timeout))
                .await;
            if let Some(ref mut gpio) = *state.gpio.lock().await {
                gpio.drive.check_timeout();
            }
        }
    });
    let s2 = state.clone();
    rocket::tokio::spawn(async move {
        let state = s2;

        loop {
            rocket::tokio::time::sleep(Duration::from_secs_f32(config.turn.shutdown_timeout)).await;
            if let Some(ref mut gpio) = *state.gpio.lock().await {
                gpio.turn.check_timeout();
            }
        }
    });
    let s3 = state.clone();
    rocket::tokio::spawn(async move {
        let state = s3;

        loop {
            rocket::tokio::time::sleep(Duration::from_secs_f32(5.0)).await;
            state.stop_gpio_if_timeout();
        }
    });

    rocket::build()
        .manage(state.clone())
        .mount("/", routes![drive, turn, /* get_power, set_power */])
}
