use tmtc_system::*;

// this is temporary
#[derive(TMValue, Default)]
pub struct Telecommand {
    subsys_id: u8,
    cmd_id: u8,
    payload: u64
}
//enum Telecommand {
//    RocketLST(LSTCommand),
//    EPS(EPSCommand),
//}
//
//enum LSTCommand {
//    Reboot,
//}
//
//enum EPSCommand {
//    SetSource(FlipFlopState, Option<u8>),
//    EnableSink(Sink, Option<u8>),
//    DisableSink(Sink, Option<u8>),
//}

#[derive(TMValue, Default, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Vec3I16 {
    x: i16,
    y: i16,
    z: i16,
}
