use tmtc_system::*;

beacon!(LowRateTelemetry, crate::definitions::telemetry,
    id = 0,
    telemetry(
        lst::Uptime,
        lst::Rssi,
        lst::Lqi,
        lst::PacketsSend,
        lst::PacketsGood,
        lst::PacketsBadChecksum,
        lst::PacketsBadOther
    )
);


beacon!(MidRateTelemetry, crate::definitions::telemetry,
    id = 1,
    telemetry(
        eps::EnableBitmap,
        eps::AuxPowerVoltage,
        eps::InternalTemperature,
        eps::Bat1Voltage,
        eps::Bat1Temperature,
        eps::Bat2Voltage,
        eps::Bat2Temperature
    )
);

beacon!(HighRateTelemetry, crate::definitions::telemetry,
    id = 2,
    telemetry(
        upper_sensor::Imu1AccelLowRange,
        upper_sensor::Imu1AccelFullRange,
        upper_sensor::Imu1Gyro,
        upper_sensor::Imu1Temp,
        upper_sensor::Imu2AccelLowRange,
        upper_sensor::Imu2AccelFullRange,
        upper_sensor::Imu2Gyro,
        upper_sensor::Imu2Temp
    )
);
