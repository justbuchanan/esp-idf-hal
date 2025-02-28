use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_sys::*;
use std::time::Duration;

// Example usage:
// cargo build --examples --target riscv32imc-esp-espidf && espflash flash target/riscv32imc-esp-espidf/debug/examples/ledc_fade --monitor

// Do a fade up and down `cycles` times.
pub fn breathe(
    ledc_driver: &mut LedcDriver,
    min_duty: u32,
    max_duty: u32,
    fade_time_ms: i32,
    cycles: usize,
) -> Result<(), EspError> {
    for _ in 0..cycles {
        // Fade up
        ledc_driver.fade_with_time(max_duty, fade_time_ms, true)?;
        // Fade down
        ledc_driver.fade_with_time(min_duty, fade_time_ms, true)?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();

    // Initialize peripherals
    let peripherals = Peripherals::take()?;
    // let mut led_pin = peripherals.pins.gpio7.into_output()?;

    // Create timer
    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default().frequency(25.kHz().into()),
    )?;

    // Create regular LEDC driver
    let mut ledc_driver = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio7, // 7 is status led
                                // peripherals.pins.gpio10, // 10 is LDD driver
    )?;

    for _ in 0..2 {
        // Fade LED up over 2 seconds
        ledc_driver.fade_with_time(
            ledc_driver.get_max_duty(),
            Duration::from_secs(2).as_millis() as i32,
            true,
        )?;

        // Fade LED down over 2 seconds
        ledc_driver.fade_with_time(0, Duration::from_secs(2).as_millis() as i32, true)?;

        // Do a breathing pattern (3 cycles)
        let md = ledc_driver.get_max_duty();
        breathe(&mut ledc_driver, 0, md, 800, 3)?;

        FreeRtos::delay_ms(1000);
    }

    ledc_driver.set_duty(ledc_driver.get_max_duty() / 10)?;
    FreeRtos::delay_ms(10000);

    // TODO: add example demonstrating callback
    // TODO: add example of cancelling an in-progress fade by setting a fixed duty cycle

    Ok(())
}
