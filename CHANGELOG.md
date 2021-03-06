# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- Add INPUTMUX and PINT peripherals
- Add example using PINT + INPUTMUX to make an external interrupt on a pin

## [v0.0.3] - 2019-11-09
Rename to `lpc55-hal`.

Some work towards Flash.

## [v0.0.2] - 2019-11-09

Several serial peripheral drivers:
- USB device (!!)
- I2C, SPI, USART (not all properly tested)
- All pins for these Flexcomm interfaces exposed

We do not pass one of the `usb_test_class` tests (unclear
if we should), the others pass. See also `test_serial.py`,
and NEVER use `moserial` for testing `(－‸ლ)`

Architectural changes:
- move from "single file per peripheral" to split between
  HAL wrappers for peripherals, and actual drivers
- get rid of a lot of references/lifetimes which don't
  cooperate with RTFM

The clock tree will definitely need some more work.
It's a conundrum between compile-time vs runtime checks,
and keeping things centralized vs distributed among peripherals.


## [v0.0.1] - 2019-09-04

- Initial release

## [v0.0.0] - 2019-08-17

- Squatting release

