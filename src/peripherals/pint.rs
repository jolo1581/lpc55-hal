use crate::{
    raw,
    peripherals::{
        syscon,
        inputmux::InputMux,
    },
    drivers::{
        pins::Pin,
    },
    typestates::{
        init_state,
        pin::{
            PinId,
            state,
            gpio::{
                direction,
            }
        },
 
    },
};

pub enum Mode {
    RisingEdge,
    FallingEdge,
    ActiveLow,
    ActiveHigh,
}
use Mode::*;

crate::wrap_stateful_peripheral!(Pint, PINT);

impl<State> Pint<State> {
    pub fn enabled(mut self, syscon: &mut syscon::Syscon) -> Pint<init_state::Enabled> {
        syscon.enable_clock(&mut self.raw);

        Pint {
            raw: self.raw,
            _state: init_state::Enabled(()),
        }
    }

    pub fn disabled(mut self, syscon: &mut syscon::Syscon) -> Pint<init_state::Disabled> {
        syscon.disable_clock(&mut self.raw);

        Pint {
            raw: self.raw,
            _state: init_state::Disabled,
        }
    }

}

impl Pint <init_state::Enabled> {

    /// LPC55 supports 8 external pin interrupts, from any PIO pin.
    /// Use `slot` to indicate (0-7) which slot you'd like to use.
    /// `mode` indicates what kind of interrupt to generate.
    /// You can call this function twice to enable both `RisingEdge` and `FallingEdge` interrupts for same pin + slot.
    pub fn enable_interrupt<PIN: PinId>(
        &mut self, 
        mux: &mut InputMux<init_state::Enabled>, 
        _pin:  &Pin<PIN, state::Gpio<direction::Input>>, 
        slot: u8, mode: Mode
    ){

        // Enable pin as external interrupt for ext int source `slot`
        mux.raw.pintsel[slot as usize].write(|w| unsafe {
            w
            .intpin().bits( (PIN::PORT << 5) as u8 | (PIN::NUMBER) )
        });

        let bit = 1 << slot;

        // Clear respective slot bit (default rising)
        self.raw.isel.modify(|r,w| unsafe {
            w.pmode().bits(
                r.pmode().bits() & (!bit)
            )
        });

        match mode {
            RisingEdge => {
                // enable level/rising interrupt
                self.raw.sienr.write(|w| unsafe {
                    w.setenrl().bits( bit )
                });
            }
            FallingEdge => {
                // enable falling interrupt
                self.raw.sienf.write(|w| unsafe {
                    w.setenaf().bits( bit )
                });
            }
            _ => {

                // Make level interrupt
                self.raw.isel.modify(|r,w| unsafe {
                    w.pmode().bits(
                        r.pmode().bits() | bit
                    )
                });

                // enable level/rising interrupt
                self.raw.sienr.write(|w| unsafe {
                    w.setenrl().bits( bit )
                });

                match mode {
                    ActiveHigh => {
                        // Make level active high
                        self.raw.sienf.write(|w| unsafe {
                            w.setenaf().bits( bit )
                        });  
                    }
                    ActiveLow => {
                        // Make level active low
                        self.raw.cienf.write(|w| unsafe {
                            w.cenaf().bits( bit )
                        });  

                    }
                    _ => {}
                }


            }
        }

    }
}