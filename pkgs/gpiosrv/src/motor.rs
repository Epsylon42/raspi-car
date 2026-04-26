use rocket::{time::Duration, tokio::time::Instant};
use rust_pigpio::{self as gpio, set_mode, write, pwm};

fn one() -> f32 {
    1.0
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Config {
    pub pos_pin: u32,
    pub neg_pin: u32,

    #[serde(default)]
    pub pwm_frequency: u32,

    #[serde(default)]
    pub pwm_default_power: Option<u8>,

    #[serde(default = "one")]
    pub shutdown_timeout: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotorState {
    Stopped,
    Braking,
    RunningP,
    RunningN,
}

pub struct Motor {
    pos_pin: u32,
    neg_pin: u32,
    state: MotorState,
    power: u32,
    last_ping: Instant,
    config: Config,
}

impl Motor {
    pub fn new(config: &Config) -> Self {
        let mut motor = Motor {
            pos_pin: config.pos_pin,
            neg_pin: config.neg_pin,
            state: MotorState::Stopped,
            power: 100,
            last_ping: Instant::now(),
            config: config.clone(),
        };

        set_mode(motor.pos_pin, gpio::OUTPUT);
        write(motor.pos_pin, gpio::OFF);
        set_mode(motor.neg_pin, gpio::OUTPUT);
        write(motor.neg_pin, gpio::OFF);

        pwm::set_pwm_frequency(motor.pos_pin, motor.freq());
        pwm::set_pwm_frequency(motor.neg_pin, motor.freq());
        pwm::set_pwm_range(motor.pos_pin, 100);
        pwm::set_pwm_range(motor.neg_pin, 100);

        if let Some(power) = config.pwm_default_power {
            motor.set_power(power);
        }

        motor
    }

    fn use_pwm(&self) -> bool {
        self.config.pwm_frequency != 0
    }

    fn brake_duty_cycle(&self) -> f64 {
        1.0 - self.power as f64 / 100.0
    }

    fn freq(&self) -> u32 {
        self.config.pwm_frequency
    }

    fn is_running(&self) -> bool {
        match self.state {
            MotorState::RunningP | MotorState::RunningN => true,
            _ => false,
        }
    }

    pub fn power(&self) -> u8 {
        self.power as u8
    }

    pub fn set_power(&mut self, power: u8) {
        self.power = power as u32;
        if self.is_running() {
            self.set_state_impl(self.state);
        }
    }

    pub fn set_state(&mut self, state: MotorState) {
        if state == self.state {
            return;
        }
        self.last_ping = Instant::now();
        self.set_state_impl(state)
    }

    fn set_state_impl(&mut self, state: MotorState) {
        self.state = state;
        match state {
            MotorState::Stopped => {
                write(self.pos_pin, gpio::OFF);
                write(self.neg_pin, gpio::OFF);
            }
            MotorState::Braking => {
                write(self.pos_pin, gpio::ON);
                write(self.neg_pin, gpio::ON);
            }
            MotorState::RunningP => {
                if self.use_pwm() {
                    write(self.pos_pin, gpio::ON);
                    pwm::pwm(self.neg_pin, 100 - self.power);
                } else {
                    write(self.pos_pin, gpio::ON);
                    write(self.neg_pin, gpio::OFF);
                }
            }
            MotorState::RunningN => {
                if self.use_pwm() {
                    pwm::pwm(self.pos_pin, 100 - self.power);
                    write(self.neg_pin, gpio::ON);
                } else {
                    write(self.pos_pin, gpio::OFF);
                    write(self.neg_pin, gpio::ON);
                }
            }
        }
    }

    pub fn check_timeout(&mut self) {
        if self.is_running()
            && Instant::now() - self.last_ping
                >= Duration::seconds_f32(self.config.shutdown_timeout)
        {
            self.set_state(MotorState::Stopped);
        }
    }
}
