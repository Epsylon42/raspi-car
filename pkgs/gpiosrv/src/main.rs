#![allow(dead_code)]

use std::sync::Arc;
use std::time::{Duration, Instant};

#[macro_use]
extern crate rocket;
use rocket::tokio::sync::{Mutex, MutexGuard, MappedMutexGuard};
use rocket_ws as ws;
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

fn parse_motor_cmd(s: &str) -> Option<motor::MotorState> {
    match s {
        "f" | "r" => Some(motor::MotorState::RunningP),
        "b" | "l" => Some(motor::MotorState::RunningN),
        "s" => Some(motor::MotorState::Stopped),
        "br" => Some(motor::MotorState::Braking),
        _ => None,
    }
}

#[get("/ws")]
fn ws_control(ws: ws::WebSocket, state: &rocket::State<State>) -> ws::Channel<'static> {
    let state = (*state).clone();
    ws.channel(move |mut stream| Box::pin(async move {
        use rocket::futures::StreamExt;
        while let Some(msg) = stream.next().await {
            let msg = match msg {
                Ok(msg) => msg,
                Err(_) => break,
            };
            let text = match msg {
                ws::Message::Text(t) => t,
                ws::Message::Close(_) => break,
                _ => continue,
            };

            // Format: "drive_cmd,turn_cmd" e.g. "f,l", "s,s"
            let parts: Vec<&str> = text.split(',').collect();
            if parts.len() != 2 {
                continue;
            }

            let drive_cmd = match parse_motor_cmd(parts[0]) {
                Some(cmd) => cmd,
                None => continue,
            };
            let turn_cmd = match parse_motor_cmd(parts[1]) {
                Some(cmd) => cmd,
                None => continue,
            };

            if let Some(mut gpio) = state.ensure_gpio() {
                gpio.drive.set_state(drive_cmd);
                gpio.turn.set_state(turn_cmd);
            }
        }

        // Stop motors when client disconnects
        if let Some(mut gpio) = state.ensure_gpio() {
            gpio.drive.set_state(motor::MotorState::Stopped);
            gpio.turn.set_state(motor::MotorState::Stopped);
        }

        Ok(())
    }))
}

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
        .mount("/", routes![ws_control])
}
