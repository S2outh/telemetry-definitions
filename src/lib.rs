#![feature(const_trait_impl)]
use telemetry_system::*;

#[telemetry_definition]
mod telemetry {

    #[tmv(i64, id = 1)]
    struct Timestamp;
    
    mod lst {
        #[tmv(u32, id = 100)]
        struct Uptime;

        #[tmv(i8, id = 101)]
        struct Rssi;

        #[tmv(u8, id = 102)]
        struct Lqi;

        #[tmv(u32, id = 103)]
        struct PacketsSend;
        
        #[tmv(u32, id = 104)]
        struct PacketsGood;

        #[tmv(u32, id = 105)]
        struct PacketsBadChecksum;

        #[tmv(u32, id = 106)]
        struct PacketsBadOther;
    }

    mod eps {
        #[tmv(u8, id = 200)]
        struct EnableBitmap;
        
        #[tmv(i16, id = 201)]
        struct AuxPowerVoltage;

        #[tmv(i16, id = 202)]
        struct InternalTemperature;

        #[tmv(i16, id = 203)]
        struct Bat1Voltage;

        #[tmv(i16, id = 204)]
        struct Bat1Temperature;
        
        #[tmv(i16, id = 205)]
        struct Bat2Voltage;

        #[tmv(i16, id = 206)]
        struct Bat2Temperature;
    }
}

beacon!(LowRateTelemetry, telemetry,
    Timestamp, 
    lst::Uptime,
    lst::Rssi,
    lst::Lqi,
    lst::PacketsSend,
    lst::PacketsGood,
    lst::PacketsBadChecksum,
    lst::PacketsBadOther
);

beacon!(MidRateTelemetry, telemetry,
    Timestamp, 
    eps::EnableBitmap,
    eps::AuxPowerVoltage,
    eps::InternalTemperature,
    eps::Bat1Voltage,
    eps::Bat1Temperature,
    eps::Bat2Voltage,
    eps::Bat2Temperature
);
