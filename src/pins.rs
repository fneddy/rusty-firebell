use crate::hal::digital::v2::*;
use crate::device::*;

macro_rules! create_pin {
    (
        struct $PinName:ident {
            port_loc: $port:ident,
            port_reg: $port_reg:ident,
            ddr: $ddr:ident,
            bit: $bit:expr,
            input: $PinNameInput:ident,
            output: $PinNameOutput:ident,
        }
    ) => {
        pub struct $PinName;
        pub struct $PinNameInput;
        pub struct $PinNameOutput;


        impl $PinName {
            pub fn as_output() -> $PinNameOutput {
                unsafe {(*<$port>::ptr()).$ddr.modify(|r, w| w.bits(r.bits() | (1 << $bit ))) };
                $PinNameOutput{}
            }

            pub fn as_input() -> $PinNameInput {
                unsafe {(*<$port>::ptr()).$ddr.modify(|r, w| w.bits(r.bits() & !(1 << $bit ))) };
                $PinNameInput{}
            }
        }


        impl OutputPin for $PinNameOutput {
            type Error = ();

            fn set_low(&mut self) -> core::result::Result<(), ()> { 
                unsafe {(*<$port>::ptr()).$port_reg.modify(|r,w| w.bits(r.bits() & !(1 << 0))) } ;
                Ok(())
            }

            fn set_high(&mut self) -> core::result::Result<(), ()> { 
                unsafe {(*<$port>::ptr()).$port_reg.modify(|r,w| w.bits(r.bits() | (1 << 0))) } ;
                Ok(())
            }
        }
    };
}



create_pin! {
    struct PB0 {
        port_loc: PORTB,
        port_reg: portb,
        ddr: ddrb,
        bit: 0,
        input: PB0Input,
        output: PB0Output,
    }
}

create_pin! {
    struct PB1 {
        port_loc: PORTB,
        port_reg: portb,
        ddr: ddrb,
        bit: 1,
        input: PB1Input,
        output: PB1Output,
    }
}

create_pin! {
    struct PB2 {
        port_loc: PORTB,
        port_reg: portb,
        ddr: ddrb,
        bit: 2,
        input: PB2Input,
        output: PB2Output,
    }
}

create_pin! {
    struct PB3 {
        port_loc: PORTB,
        port_reg: portb,
        ddr: ddrb,
        bit: 3,
        input: PB3Input,
        output: PB3Output,
    }
}

create_pin! {
    struct PB4 {
        port_loc: PORTB,
        port_reg: portb,
        ddr: ddrb,
        bit: 4,
        input: PB4Input,
        output: PB4Output,
    }
}
create_pin! {
    struct PB5 {
        port_loc: PORTB,
        port_reg: portb,
        ddr: ddrb,
        bit: 5,
        input: PB5Input,
        output: PB5Output,
    }
}
create_pin! {
    struct PB6 {
        port_loc: PORTB,
        port_reg: portb,
        ddr: ddrb,
        bit: 6,
        input: PB6Input,
        output: PB6Output,
    }
}
create_pin! {
    struct PB7 {
        port_loc: PORTB,
        port_reg: portb,
        ddr: ddrb,
        bit: 7,
        input: PB7Input,
        output: PB7Output,
    }
}
