#![no_std]
#![no_main]

use arduino_hal::hal::port::{self};
use arduino_hal::port::mode::Output;
use arduino_hal::{delay_ms, Delay, Peripherals};

use hd44780_driver::bus::FourBitBus;
use hd44780_driver::HD44780;
use panic_halt as _;

mod utils;

include!(concat!(env!("OUT_DIR"), "/time.rs"));

pub const ALARM_HOUR: u8 = 12;
pub const ALARM_MINUTE: u8 = 00;
const BEEP_COUNT: u8 = 10;

fn setup_screen(
    d2: port::Pin<Output, port::PD2>,
    d3: port::Pin<Output, port::PD3>,
    d4: port::Pin<Output, port::PD4>,
    d5: port::Pin<Output, port::PD5>,
    d6: port::Pin<Output, port::PD6>,
    d7: port::Pin<Output, port::PD7>,
) -> HD44780<
    FourBitBus<
        port::Pin<Output, port::PD2>,
        port::Pin<Output, port::PD3>,
        port::Pin<Output, port::PD4>,
        port::Pin<Output, port::PD5>,
        port::Pin<Output, port::PD6>,
        port::Pin<Output, port::PD7>,
    >,
> {
    let mut delay = Delay::new();
    let mut lcd = HD44780::new_4bit(d2, d3, d4, d5, d6, d7, &mut delay).unwrap();

    lcd.reset(&mut delay).unwrap();
    lcd.clear(&mut delay).unwrap();

    lcd
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let d2 = pins.d2.into_output();
    let d3 = pins.d3.into_output();
    let d4 = pins.d4.into_output();
    let d5 = pins.d5.into_output();
    let d6 = pins.d6.into_output();
    let d7 = pins.d7.into_output();

    let mut buzzer = pins.d8.into_output();
    let mut led = pins.d1.into_output();

    let mut delay = Delay::new();

    let mut screen = setup_screen(d2, d3, d4, d5, d6, d7);

    let mut hours: u8 = CURRENT_HR;
    let mut minutes: u8 = CURRENT_MN;
    let mut seconds: u8 = CURRENT_SEC;

    loop {
        delay_ms(1000);

        seconds += 1;

        if seconds >= 60 {
            seconds = 0;
            minutes += 1;

            if minutes >= 60 {
                minutes = 0;
                hours += 1;

                if hours >= 24 {
                    hours = 0;
                }
            }
        }

        screen.clear(&mut delay).unwrap();

        if (hours == ALARM_HOUR) && (minutes == ALARM_MINUTE) && (seconds < 1) {
            for _ in 0..(BEEP_COUNT * 2) {
                led.toggle();
                buzzer.toggle();
                delay_ms(1000);
            }
            led.set_low();
            buzzer.set_low();
            seconds = BEEP_COUNT * 2;
        }

        let mut buf = [0u8; 64];

        let formatted_time: &str = utils::write_to::show(
            &mut buf,
            format_args!("Hora: {:?}:{:?}:{:?}", hours, minutes, seconds),
        )
        .unwrap();

        screen.set_cursor_pos(0, &mut delay).unwrap();
        screen.write_str(formatted_time, &mut delay).unwrap();
        screen.set_cursor_pos(64, &mut delay).unwrap();

        let formatted_alarm: &str = utils::write_to::show(
            &mut buf,
            format_args!("Alarma: {:?}:{:?}", ALARM_HOUR, ALARM_MINUTE),
        )
        .unwrap();

        screen.write_str(formatted_alarm, &mut delay).unwrap();
    }
}
