#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::simple_pwm::IntoPwmPin;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut timer3 = arduino_hal::simple_pwm::Timer3Pwm::new(dp.TC3,
    arduino_hal::simple_pwm::Prescaler::Prescale1024);

    let mut red_led = pins.d5.into_output().into_pwm(&mut timer3);
    let mut green_led = pins.d3.into_output().into_pwm(&mut timer3);
    let mut blue_led = pins.d2.into_output().into_pwm(&mut timer3);

    red_led.enable();
    green_led.enable();
    blue_led.enable();

    red_led.set_duty(255);
    green_led.set_duty(255);
    blue_led.set_duty(255);

    loop {
        red_led.set_duty(0);
        green_led.set_duty(255);
        blue_led.set_duty(255);
        arduino_hal::delay_ms(1000);

        red_led.set_duty(0);
        green_led.set_duty(0);
        blue_led.set_duty(255);
        arduino_hal::delay_ms(1000);

        red_led.set_duty(255);
        green_led.set_duty(0);
        blue_led.set_duty(255);
        arduino_hal::delay_ms(1000);
    }
}
