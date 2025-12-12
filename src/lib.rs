#![no_std]
#![feature(const_trait_impl)]
pub mod command_types;

use tmtc_system::*;
// public reexports
pub use tmtc_system::TMValue;
pub use tmtc_system::DynTMValue;
pub use tmtc_system::DynBeacon;
pub use tmtc_system::DynTelemetryDefinition;
pub use low_rate_telemetry::LowRateTelemetry;
pub use mid_rate_telemetry::MidRateTelemetry;

#[telemetry_definition(id = 0)]
mod telecommands {
    #[tmv(crate::command_types::Telecommand)]
    struct Telecommand;
}

#[telemetry_definition(id = 1)]
pub mod telemetry {

    #[tmv(i64)]
    struct Timestamp;
    
    #[tmm(id = 100)]
    mod lst {
        #[tmv(u32)]
        struct Uptime;

        #[tmv(i8)]
        struct Rssi;

        #[tmv(u8)]
        struct Lqi;

        #[tmv(u32)]
        struct PacketsSend;
        
        #[tmv(u32)]
        struct PacketsGood;

        #[tmv(u32)]
        struct PacketsBadChecksum;

        #[tmv(u32)]
        struct PacketsBadOther;
    }

    #[tmm(id = 200)]
    mod eps {
        #[tmv(u8)]
        struct EnableBitmap;
        
        #[tmv(i16)]
        struct AuxPowerVoltage;

        #[tmv(i16)]
        struct InternalTemperature;

        #[tmv(i16)]
        struct Bat1Voltage;

        #[tmv(i16)]
        struct Bat1Temperature;
        
        #[tmv(i16)]
        struct Bat2Voltage;

        #[tmv(i16)]
        struct Bat2Temperature;
    }
}

beacon!(LowRateTelemetry, telemetry,
    header(0x4E, 0x4A, 1, 0, 37, 0, 0),
    telemetry(
        Timestamp, 
        lst::Uptime,
        lst::Rssi,
        lst::Lqi,
        lst::PacketsSend,
        lst::PacketsGood,
        lst::PacketsBadChecksum,
        lst::PacketsBadOther
    )
);

beacon!(MidRateTelemetry, telemetry,
    header(0x4E, 0x4A, 2, 0, 28, 0, 0),
    telemetry(
        Timestamp, 
        eps::EnableBitmap,
        eps::AuxPowerVoltage,
        eps::InternalTemperature,
        eps::Bat1Voltage,
        eps::Bat1Temperature,
        eps::Bat2Voltage,
        eps::Bat2Temperature
    )
);
