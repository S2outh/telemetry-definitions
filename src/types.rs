use tmtc_system::*;

// # Telecommands
#[derive(TMValue)]
pub enum Telecommand {
    RocketLST(LSTCommand),
    EPS(EPSCommand),
}

#[derive(TMValue)]
pub enum LSTCommand {
    Reboot,
}

#[derive(TMValue)]
pub enum EPSCommand {
    SetSource(FlipFlopState, Option::<u8>),
    EnableSink(Sink, Option::<u8>),
    DisableSink(Sink, Option::<u8>),
}

#[derive(TMValue, Clone, Copy)]
pub enum FlipFlopState {
    On,
    Bat1,
    Bat2,
    AuxPwr,
}

#[derive(TMValue, Clone, Copy)]
pub enum Sink {
    RocketLST,
    SensorUpper,
    GPS,
    RocketHD,
}

// # Vector types
#[derive(TMValue, Default, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, Debug))]
pub struct Vec3I16 {
    x: i16,
    y: i16,
    z: i16,
}
