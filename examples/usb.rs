#![no_main]
#![no_std]

extern crate panic_semihosting;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{dbg, hprintln};

#[allow(unused_imports)]
use hal::prelude::*;
#[allow(unused_imports)]
use lpc55s6x_hal as hal;

use hal::{reg_read, reg_modify, dbg_reg_modify};

use usbd_serial::{SerialPort, USB_CLASS_CDC};
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use hal::usbfs::bus::UsbBus;

#[entry]
fn main() -> ! {
    // let x: [u16; 3] = [1,2,3];
    // let y = [1,2,3]::<[u16; 3]>;
    let dp = hal::raw::Peripherals::take().unwrap();
    let iocon = hal::iocon::wrap(dp.IOCON);
    let mut syscon = hal::syscon::wrap(dp.SYSCON);
    let mut pmc = hal::pmc::wrap(dp.PMC);
    let mut gpio = hal::gpio::wrap(dp.GPIO).enabled(&mut syscon);

    // BOARD_InitPins
    iocon.configure_pio_0_22_as_usb0_vbus();
    let pins = hal::pins::Pins::take().unwrap();
    let usb0_vbus = pins.pio0_22;
    let mut red_led = pins
        .pio1_6
        .into_gpio_pin(&mut gpio)
        .into_output(hal::gpio::Level::High); // start turned off

    // Setup clocking
    reg_modify!(SYSCON, mainclksela, sel, enum_0x0); // FRO 12 MHz, was enum_0x3
    // either 48 MHz (div = 1, flashtim = 4) or 96 MHz (div = 0, flashtim = 8)
    // dbg!(reg_read!(SYSCON, fmccr, flashtim));
    // reg_modify!(SYSCON, fmccr, flashtim, flashtim4); // This is actually the reset value
    // unsafe { reg_modify!(SYSCON, ahbclkdiv, div, 1u8) }; // This is actually the reset value
    reg_modify!(SYSCON, fmccr, flashtim, flashtim8); // This is actually the reset value
    unsafe { reg_modify!(SYSCON, ahbclkdiv, div, 0u8) }; // This is actually the reset value
    while reg_read!(SYSCON, ahbclkdiv, reqflag, is_ongoing) {}
    reg_modify!(SYSCON, mainclksela, sel, enum_0x3); // FRO 96 MHz

    // Configure USB0 main clock
    // reg_modify!(SYSCON, usb0clkdiv, halt, halt);
    unsafe { reg_modify!(SYSCON, usb0clkdiv, div, 1u8) };
    reg_modify!(SYSCON, usb0clkdiv, halt, run); // <-- toootally don't forget this ~groans~
    reg_modify!(SYSCON, usb0clksel, sel, enum_0x0); // FRO 96 MHz
    while reg_read!(SYSCON, usb0clkdiv, reqflag, is_ongoing) {}  // never stops for div=1 *before* selecting it

    // Turn on USB0 PHY
    reg_modify!(PMC, pdruncfg0, pden_usbfsphy, poweredon);
    reg_modify!(SYSCON, ahbclkctrl1, usb0_dev, enable);
    let usbfsd = hal::usbfs::device::wrap(dp.USB0).enabled(&mut pmc, &mut syscon);

    // Switch USB0 to "device" mode (default is "host")
    reg_modify!(SYSCON, ahbclkctrl2, usb0_hosts, enable);
    // dbg!(reg_read!(USBFSH, portmode, dev_enable));
    reg_modify!(USBFSH, portmode, dev_enable, set_bit);
    // dbg!(reg_read!(USBFSH, portmode, dev_enable));
    reg_modify!(SYSCON, ahbclkctrl2, usb0_hosts, disable);

    // Turn on USB1 SRAM
    reg_modify!(SYSCON, ahbclkctrl2, usb1_ram, enable);

    // let usb_bus = UsbBus::new(dp.USB0, (usb0_vbus,));
    let usb_bus = UsbBus::new(usbfsd, ());
    // let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0xcc1d))
        .manufacturer("nickray")
        .product("Demo Demo Demo")
        .serial_number("2019")
        .max_packet_size_0(64)
        .build();

    // dbg!("main loop");
    loop {
        if !usb_dev.poll(&mut []) {
            continue;
        }
    }

}