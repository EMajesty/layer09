#![no_std]
#![no_main]

use rp_pico::entry;
use panic_halt as _;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::MODE_3;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal::spi::Spi;
use rp_pico::hal::gpio::FunctionSpi;
use rp_pico::hal;
use rp_pico::hal::fugit::RateExtU32;
use display_interface_spi::SPIInterface;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::image::*;
use mipidsi::{Builder, models::ST7789};

const SCR_WT: u16 = 240;
const SCR_HT: u16 = 320;

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
        .init(&mut pac.RESETS, 125_000_000u32.Hz(), 32_000_000u32.Hz(), MODE_3);

    let di = SPIInterface::new(spi, dc);
    let mut display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .init(&mut delay)?;
    // let mut display = ST7789::new(display_interface, Some(rst), Some(backlight), SCR_WT, SCR_HT);

    // display.init(&mut delay).unwrap();
    // display.set_orientation(Orientation::PortraitSwapped).unwrap();

    // let raw_image_data = ImageRawLE::new(include_bytes!("../assets/image.raw"), 240);
    // let image = Image::new(&raw_image_data, Point::new(0, 0));
    // image.draw(&mut display).unwrap();

    loop {
        continue;
        // let mut i = 0;
        //
        // while i < SCR_HT {
        //     let mut j = 0;
        //     while j < SCR_WT {
        //         display.set_pixel(j, i, 59423).unwrap();
        //         j = j + 1;
        //     }
        //     i = i + 1;
        // }
        //
        // let mut i = 0;
        //
        // while i < SCR_HT {
        //     let mut j = 0;
        //     while j < SCR_WT {
        //         display.set_pixel(j, i, 2015).unwrap();
        //         j = j + 1;
        //     }
        //     i = i + 1;
        // }
    }
}
