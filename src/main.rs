#![no_std]
#![no_main]

use dshot_pio::dshot_rp2040_hal::{DshotPio, DshotPioTrait};
use hal::{
    entry,
    gpio::{self, Pin},
    {Sio, pac},
};
use panic_halt as _;
use rp2040_hal as hal;
use rp2040_hal::fugit::RateExtU32;
use rp2040_hal::gpio::FunctionPio0;
use rp2040_hal::uart::{DataBits, StopBits, UartConfig};
use rp2040_hal::{Clock, clocks};

#[unsafe(link_section = ".boot_loader")]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    // Configure the clocks
    let clocks = clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let pin0: Pin<_, FunctionPio0, _> = pins.gpio0.into_function();
    let pin1: Pin<_, FunctionPio0, _> = pins.gpio1.into_function();

    let mut dshot = DshotPio::<2, _>::new(pac.PIO0, &mut pac.RESETS, pin0, pin1, (50, 0));

    // Publish these values to the DshotPio just to say hello.
    dshot.throttle_clamp([1000, 1000]);

    let uart_pins = (
        // UART TX on pin 4
        pins.gpio4.into_function(),
        // UART RX on pin 5
        pins.gpio5.into_function(),
    );
    let uart = hal::uart::UartPeripheral::new(pac.UART1, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(115200.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();

    let mut buffer = [0u8; 5];

    // expect a number with 4 digits from the uart terminated by a newline
    loop {
        if uart.read_full_blocking(&mut buffer).is_ok() {
            let mut num = 0_u16;
            for (i, char) in buffer.iter().enumerate().take(4) {
                if (48..=57).contains(char) {
                    let digit = char - 48;
                    let digit_num = digit as u16 * 10_u16.pow(3 - i as u32);
                    num += digit_num;
                } else {
                    continue;
                }
            }
            dshot.throttle_clamp([num, num]);
        };
    }
}
