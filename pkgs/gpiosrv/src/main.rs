#![allow(dead_code)]

use std::time::{Duration, Instant};

#[macro_use]
extern crate rocket;
use rocket::tokio::sync::Mutex;
use rocket_ws as ws;
use rust_pigpio as gpio;

mod motor;

fn one() -> f32 {
    1.0
}

#[derive(Clone, serde::Deserialize)]
struct Config {
    drive: motor::Config,
    turn: motor::Config,
    #[serde(default = "one")]
    reset_timeout_seconds: f32,
}

struct Peripherals {
    drive: motor::Motor,
    turn: motor::Motor,
    running_since: Instant,
}

impl Peripherals {
    fn try_init(config: &Config) -> Result<Self, String> {
        match gpio::initialize() {
            Ok(_) => {
                let peri = Peripherals {
                    drive: motor::Motor::new(&config.drive),
                    turn: motor::Motor::new(&config.turn),
                    running_since: Instant::now(),
                };
                Ok(peri)
            }

            Err(e) => Err(e),
        }
    }
}

impl Drop for Peripherals {
    fn drop(&mut self) {
        eprintln!("drop");
        gpio::terminate();
    }
}

enum PeriperalsState {
    Uninit,
    Taken,
    Ready(Peripherals),
}

struct StateData {
    config: Config,
    peri: Mutex<PeriperalsState>,
}

enum TakePeripheralsError {
    Taken,
    Error(String),
}

impl StateData {
    pub fn new(config: Config) -> Self {
        StateData {
            config,
            peri: Mutex::new(PeriperalsState::Uninit),
        }
    }

    pub fn try_take_peripherals(&self) -> Result<Peripherals, TakePeripheralsError> {
        let state = self.peri.try_lock();
        let mut state = match state {
            Ok(x) => x,
            Err(e) => return Err(TakePeripheralsError::Error(e.to_string())),
        };

        match &*state {
            PeriperalsState::Uninit => match Peripherals::try_init(&self.config) {
                Ok(peri) => {
                    *state = PeriperalsState::Taken;
                    Ok(peri)
                }

                Err(e) => {
                    eprintln!(
                        "Count not initialize peripherals. Running in mock mode. cause: {:?}",
                        e
                    );
                    Err(TakePeripheralsError::Error(e.to_string()))
                }
            },

            PeriperalsState::Taken => Err(TakePeripheralsError::Taken),

            PeriperalsState::Ready(_) => {
                let state = std::mem::replace(&mut *state, PeriperalsState::Taken);
                match state {
                    PeriperalsState::Ready(peri) => Ok(peri),
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn return_peripherals(&self, mut peri: Peripherals) {
        peri.drive.set_state(motor::MotorState::Stopped);
        peri.turn.set_state(motor::MotorState::Stopped);

        let state = self.peri.try_lock();
        let mut state = match state {
            Ok(x) => x,
            Err(e) => panic!("Could not lock mutex to return peripherals: {e}"),
        };

        *state = PeriperalsState::Ready(peri);
    }
}

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
fn ws_control<'a>(
    ws: ws::WebSocket,
    state: &'a rocket::State<StateData>,
) -> Result<ws::Channel<'a>, rocket::http::Status> {
    let mut peri = match state.try_take_peripherals() {
        Ok(peri) => peri,
        Err(TakePeripheralsError::Taken) => {
            let channel = ws.channel(|mut stream| {
                Box::pin(async move {
                    use rocket::futures::SinkExt;
                    let _ = stream.send(ws::Message::Text(String::from("occupied"))).await;
                    Ok(())
                })
            });
            return Ok(channel);
        }
        Err(TakePeripheralsError::Error(e)) => {
            eprintln!("Could not lock peripherals: {e}");
            return Err(rocket::http::Status::InternalServerError);
        }
    };

    let channel = ws.channel(move |mut stream| {
        Box::pin(async move {
            use rocket::futures::{SinkExt, StreamExt};
            let mksleep = || -> std::pin::Pin<Box<dyn Future<Output = ()> + Send>> {
                Box::pin(rocket::tokio::time::sleep(Duration::from_secs_f32(
                    state.config.reset_timeout_seconds,
                )))
            };
            let _ = stream.send(ws::Message::Text(String::from("ok"))).await;
            let mut timeout = mksleep();
            loop {
                let text: String;
                rocket::tokio::select! {
                    Some(Ok(msg)) = stream.next() => {
                        match msg {
                            ws::Message::Text(t) => {
                                timeout = mksleep();
                                text = t;
                            }
                            ws::Message::Close(_) => break,
                            _ => continue,
                        }
                    }

                    _ = &mut timeout => {
                        text = String::from("s,s");
                        timeout = Box::pin(rocket::futures::future::pending());
                    }

                    else => break,
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

                peri.drive.set_state(drive_cmd);
                peri.turn.set_state(turn_cmd);
            }

            // Stop motors when client disconnects
            peri.drive.set_state(motor::MotorState::Stopped);
            peri.turn.set_state(motor::MotorState::Stopped);
            state.return_peripherals(peri);

            Ok(())
        })
    });

    Ok(channel)
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

    rocket::build()
        .manage(StateData::new(config.clone()))
        .mount("/", routes![ws_control])
}
