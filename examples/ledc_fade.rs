use core::ffi::c_void;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_sys::*;
use std::ptr;
use std::time::Duration;

// Option<unsafe extern "C" fn(arg1: *const ledc_cb_param_t, arg: *mut c_void) -> bool>;

fn callback(arg1: *const ledc_cb_param_t, arg: *mut c_void) -> bool {
    println!("Led callback received");
    return true;
}

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();

    let peripherals = Peripherals::take()?;

    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default().frequency(25.kHz().into()),
    )?;

    let mut ledc_driver = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio7,
    )?;

    for _ in 0..2 {
        // Fade up over 2 seconds
        ledc_driver.fade_with_time(
            ledc_driver.get_max_duty(),
            Duration::from_secs(2).as_millis() as i32,
            true,
        )?;

        // Fade down over 2 seconds
        ledc_driver.fade_with_time(0, Duration::from_secs(2).as_millis() as i32, true)?;
    }

    ledc_driver.set_duty(ledc_driver.get_max_duty() / 10)?;
    FreeRtos::delay_ms(10000);

    // TODO: add example demonstrating callback
    // TODO: add example of cancelling an in-progress fade by setting a fixed duty cycle

    ledc_driver.register_fade_callback(Some(&callback), ptr::null_mut());

    Ok(())
}
