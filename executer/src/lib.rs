#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::pwm::SetDutyCycle;
use panic_probe as _;

use rp_pico::{
    self as bsp,
    hal::{self, pwm::Slices},
};

use bsp::hal::{
    clocks::{Clock, init_clocks_and_plls},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

pub struct Executer {
    delay: Delay,
    pins: bsp::Pins,

    pwm_slices: Slices,
}

impl Executer {
    pub fn new() -> Self {
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();
        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        let sio = Sio::new(pac.SIO);

        let external_xtal_freq_hz = 12_000_000u32;
        let clocks = init_clocks_and_plls(
            external_xtal_freq_hz,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        let pins = bsp::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

        info!("Executer initialized");

        Self {
            delay,
            pins,
            pwm_slices,
        }
    }

    pub fn main_loop(mut self) -> ! {
        let pwm = &mut self.pwm_slices.pwm4;
        pwm.set_ph_correct();
        pwm.enable();

        let channel = &mut pwm.channel_b;
        channel.output_to(self.pins.led);

        const LOW: u16 = 0;
        const HIGH: u16 = 65535;

        loop {
            info!("up!");
            for i in LOW..=HIGH {
                self.delay.delay_us(8);
                let _ = channel.set_duty_cycle(i);
            }

            info!("down!");
            for i in (LOW..=HIGH).rev() {
                self.delay.delay_us(8);
                let _ = channel.set_duty_cycle(i);
            }
        }
    }
}
