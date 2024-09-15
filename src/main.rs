#![no_std]
#![no_main]

use rp_pico::entry;
use panic_halt as _;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::MODE_0;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal::spi::Spi;
use rp_pico::hal::gpio::FunctionSpi;
use rp_pico::hal;
use rp_pico::hal::fugit::RateExtU32;
use st7789::{Orientation, ST7789};
use display_interface_spi::SPIInterface;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;

#[entry]
fn stupid() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC, 
        pac.CLOCKS, 
        pac.PLL_SYS, 
        pac.PLL_USB, 
        &mut pac.RESETS, 
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();

    let sclk = pins.gpio2.into_function::<FunctionSpi>();
    let mosi = pins.gpio3.into_function::<FunctionSpi>();
    let backlight = pins.gpio22.into_push_pull_output();
    let rst = pins.gpio26.into_push_pull_output();
    let cs = pins.gpio21.into_push_pull_output();
    let dc = pins.gpio18.into_push_pull_output();

    let spi_device = pac.SPI0;
    let spi_pin_layout = (mosi, sclk);

    let spi = Spi::<_, _, _, 8>::new(spi_device, spi_pin_layout)
        .init(&mut pac.RESETS, 125_000_000u32.Hz(), 16_000_000u32.Hz(), MODE_0);

    let display_interface = SPIInterface::new(spi, dc, cs);
    let mut display = ST7789::new(display_interface, Some(rst), Some(backlight), 240, 320);

    display.init(&mut delay).unwrap();
    display.set_orientation(Orientation::PortraitSwapped).unwrap();

    let line1 = Line::new(Point::new(100, 20), Point::new(100, 220))
        .into_styled(PrimitiveStyle::with_stroke(RgbColor::WHITE, 10));
    let line2 = Line::new(Point::new(100, 20), Point::new(160, 20))
        .into_styled(PrimitiveStyle::with_stroke(RgbColor::WHITE, 10));
    let line3 = Line::new(Point::new(100, 105), Point::new(160, 105))
        .into_styled(PrimitiveStyle::with_stroke(RgbColor::WHITE, 10));

    let triangle = Triangle::new(
        Point::new(240, 100),
        Point::new(240, 140),
        Point::new(320, 120),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN));

    display.clear(Rgb565::BLACK).unwrap();
    line1.draw(&mut display).unwrap();
    line2.draw(&mut display).unwrap();
    line3.draw(&mut display).unwrap();
    triangle.draw(&mut display).unwrap();

    let mut scroll = 1u16; // absolute scroll offset
    let mut direction = true; // direction
    let scroll_delay = 20u8; // delay between steps
    loop {
        delay.delay_ms(scroll_delay);
        display.set_scroll_offset(scroll).unwrap();

        if scroll % 80 == 0 {
            direction = !direction;
        }

        match direction {
            true => scroll += 1,
            false => scroll -= 1,
        }
    }
}
